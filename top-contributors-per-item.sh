#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./top-contributors-per-item.sh [TOP_N]
#
# Example:
#   ./top-contributors-per-item.sh 5
#
# Output:
#   top-contributors-per-item.csv
#   top-contributors-per-item.json
#   top-contributors-per-item.md

TOP_N="${1:-3}"

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "ERROR: Run this script inside a git repository (repo root recommended)." >&2
  exit 1
fi

# Ensure we're at repo root for stable relative paths
REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "$REPO_ROOT"

# Items we’ll analyze:
# - each first-level dir under samples/, tools/, tutorials/, lib/
# - each file under scripts/
# - whole directories: schema-gen-helper, docs, docker
declare -a ITEMS=()

add_child_dirs_as_items () {
  local parent="$1"
  if [[ -d "$parent" ]]; then
    while IFS= read -r -d '' d; do
      local rel="${d#./}"
      ITEMS+=("$rel")
    done < <(find "$parent" -mindepth 1 -maxdepth 1 -type d -print0 | sort -z)
  fi
}

add_child_files_as_items () {
  local parent="$1"
  if [[ -d "$parent" ]]; then
    while IFS= read -r -d '' f; do
      local rel="${f#./}"
      ITEMS+=("$rel")
    done < <(find "$parent" -mindepth 1 -maxdepth 1 -type f -print0 | sort -z)
  fi
}

add_if_exists () {
  local path="$1"
  if [[ -e "$path" ]]; then
    ITEMS+=("$path")
  fi
}

add_child_dirs_as_items "samples"
add_child_dirs_as_items "tools"
add_child_dirs_as_items "tutorials"
add_child_dirs_as_items "lib"

add_child_files_as_items "scripts"
add_child_files_as_items "samples/quickstarts"

add_if_exists "schema-gen-helper"
add_if_exists "docs"
add_if_exists "docker"

# De-duplicate items
mapfile -t ITEMS < <(printf "%s\n" "${ITEMS[@]}" | awk '!seen[$0]++' | sort)

# Helper: return "commit_count<TAB>name<TAB>email" lines for a path.
# We count commits by author email to avoid splitting the same person by
# name variants, then pick the most recent author name seen for that email.
contributors_for_path () {
  local path="$1"
  git log --no-merges --format='%ae%x09%an' -- "$path" \
    | awk -F'\t' '
        {
          email=$1; name=$2;
          c[email]++
          last_name[email]=name
        }
        END {
          for (e in c) {
            printf "%d\t%s\t%s\n", c[e], last_name[e], e
          }
        }
      ' \
    | sort -nr -k1,1
}

# Helper: return the most recent commit date touching a path (UTC ISO-8601)
# Example: 2026-01-15T12:34:56Z
last_modified_utc_for_path () {
  local path="$1"
  git log -1 --format='%cI' -- "$path" 2>/dev/null | sed 's/+00:00/Z/'
}

# Escape for JSON (minimal)
json_escape () {
  echo -n "$1" | sed \
    -e 's/\\/\\\\/g' \
    -e 's/"/\\"/g' \
    -e 's/\t/\\t/g' \
    -e 's/\r/\\r/g' \
    -e 's/\n/\\n/g'
}

CSV_OUT="top-contributors-per-item.csv"
JSON_OUT="top-contributors-per-item.json"
MD_OUT="top-contributors-per-item.md"

echo "item_path,last_modified_utc,rank,commit_count,author_name,author_email" > "$CSV_OUT"

# Start JSON
{
  echo "["
} > "$JSON_OUT"

# Start Markdown
{
  echo "# Top contributors per item"
  echo
  echo "- Repo: $(basename "$REPO_ROOT")"
  echo "- Generated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  echo "- Top N per item: $TOP_N"
  echo
} > "$MD_OUT"

first_json_item=1

for item in "${ITEMS[@]}"; do
  contrib_lines="$(contributors_for_path "$item" || true)"
  if [[ -z "$contrib_lines" ]]; then
    continue
  fi

  last_modified="$(last_modified_utc_for_path "$item" || true)"
  if [[ -z "$last_modified" ]]; then
    # Should be rare if contrib_lines is non-empty, but keep it safe.
    last_modified="unknown"
  fi

  echo "## \`$item\`" >> "$MD_OUT"
  echo >> "$MD_OUT"
  echo "- Last modified (UTC): \`$last_modified\`" >> "$MD_OUT"
  echo >> "$MD_OUT"
  echo "| Rank | Commits | Author | Email |" >> "$MD_OUT"
  echo "|---:|---:|---|---|" >> "$MD_OUT"

  # JSON item header
  if [[ $first_json_item -eq 0 ]]; then
    echo "," >> "$JSON_OUT"
  fi
  first_json_item=0

  {
    echo -n "  {\"item\":\"$(json_escape "$item")\",\"last_modified_utc\":\"$(json_escape "$last_modified")\",\"top_contributors\":["
  } >> "$JSON_OUT"

  first_contrib=1
  rank=0

  while IFS=$'\t' read -r commit_count author_name author_email; do
    [[ -z "${commit_count:-}" ]] && continue
    rank=$((rank+1))
    if [[ $rank -gt $TOP_N ]]; then
      break
    fi

    # CSV
    echo "\"$item\",\"$last_modified\",$rank,$commit_count,\"${author_name//\"/\"\"}\",\"${author_email//\"/\"\"}\"" >> "$CSV_OUT"

    # Markdown
    safe_name="${author_name//|/\\|}"
    safe_email="${author_email//|/\\|}"
    echo "| $rank | $commit_count | $safe_name | $safe_email |" >> "$MD_OUT"

    # JSON contributor
    if [[ $first_contrib -eq 0 ]]; then
      echo -n "," >> "$JSON_OUT"
    fi
    first_contrib=0
    echo -n "{\"rank\":$rank,\"commits\":$commit_count,\"name\":\"$(json_escape "$author_name")\",\"email\":\"$(json_escape "$author_email")\"}" >> "$JSON_OUT"
  done <<< "$contrib_lines"

  echo "]}" >> "$JSON_OUT"
  echo >> "$MD_OUT"
done

# End JSON
{
  echo
  echo "]"
} >> "$JSON_OUT"

echo "Wrote:"
echo "  - $CSV_OUT"
echo "  - $JSON_OUT"
echo "  - $MD_OUT"