#!/bin/bash
# Test script for quickstart-deploy.md
# Tests steps starting from "Connect cluster to Azure Arc"
# Stops on first failure and reports status.

set -euo pipefail

# ── Helpers ──────────────────────────────────────────────────────────────────

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

step() { echo -e "\n${YELLOW}>>> $1${NC}"; }
ok()   { echo -e "${GREEN}[OK]${NC} $1"; }
fail() { echo -e "${RED}[FAIL]${NC} $1"; exit 1; }

trap 'fail "Script failed at line $LINENO. Last command exit code: $?"' ERR

# ── Collect inputs ────────────────────────────────────────────────────────────

echo "========================================"
echo " Azure IoT Operations quickstart tester"
echo " (quickstart-deploy.md)"
echo "========================================"
echo
echo "Enter required values. Press Enter to accept an existing environment variable."
echo

prompt_var() {
    local varname="$1"
    local prompt="$2"
    local current="${!varname:-}"
    if [[ -n "$current" ]]; then
        read -rp "${prompt} [${current}]: " input
        export "$varname"="${input:-$current}"
    else
        read -rp "${prompt}: " input
        [[ -z "$input" ]] && fail "${varname} is required."
        export "$varname"="$input"
    fi
}

prompt_var SUBSCRIPTION_ID   "Azure Subscription ID"
prompt_var RESOURCE_GROUP    "Resource group name (will be created)"
prompt_var LOCATION          "Azure region (e.g. eastus2)"
prompt_var CLUSTER_NAME      "Cluster name (set to codespace name in Codespaces)"
prompt_var STORAGE_ACCOUNT   "Storage account name (3-24 chars, lowercase/numbers only)"
prompt_var SCHEMA_REGISTRY   "Schema registry name (lowercase, numbers, hyphens)"
prompt_var SCHEMA_REGISTRY_NAMESPACE "Schema registry namespace (lowercase, numbers, hyphens)"

echo
echo "Configuration:"
echo "  SUBSCRIPTION_ID          = $SUBSCRIPTION_ID"
echo "  RESOURCE_GROUP           = $RESOURCE_GROUP"
echo "  LOCATION                 = $LOCATION"
echo "  CLUSTER_NAME             = $CLUSTER_NAME"
echo "  STORAGE_ACCOUNT          = $STORAGE_ACCOUNT"
echo "  SCHEMA_REGISTRY          = $SCHEMA_REGISTRY"
echo "  SCHEMA_REGISTRY_NAMESPACE= $SCHEMA_REGISTRY_NAMESPACE"
echo
read -rp "Proceed? [y/N] " confirm
[[ "${confirm,,}" == "y" ]] || { echo "Aborted."; exit 0; }

# ── Connect cluster to Azure Arc ─────────────────────────────────────────────

step "Signing in to Azure CLI"
az login
ok "Signed in"

step "Setting subscription to $SUBSCRIPTION_ID"
az account set --subscription "$SUBSCRIPTION_ID"
ok "Subscription set"

step "Registering required resource providers"
az provider register -n "Microsoft.ExtendedLocation"
az provider register -n "Microsoft.Kubernetes"
az provider register -n "Microsoft.KubernetesConfiguration"
az provider register -n "Microsoft.IoTOperations"
az provider register -n "Microsoft.DeviceRegistry"
az provider register -n "Microsoft.SecretSyncController"
ok "Resource providers registered (registration may still be in progress)"

step "Creating resource group: $RESOURCE_GROUP"
az group create --location "$LOCATION" --resource-group "$RESOURCE_GROUP"
ok "Resource group created"

step "Arc-enabling Kubernetes cluster: $CLUSTER_NAME"
az connectedk8s connect --name "$CLUSTER_NAME" --location "$LOCATION" --resource-group "$RESOURCE_GROUP"
ok "Cluster connected to Azure Arc"

step "Getting Microsoft Entra ID application objectId for Azure Arc"
export OBJECT_ID=$(az ad sp show --id bc313c14-388c-4e7d-a58e-70017303ee3b --query id -o tsv)
[[ -z "$OBJECT_ID" ]] && fail "Failed to retrieve OBJECT_ID"
ok "OBJECT_ID=$OBJECT_ID"

step "Enabling custom locations on cluster"
az connectedk8s enable-features \
    -n "$CLUSTER_NAME" \
    -g "$RESOURCE_GROUP" \
    --custom-locations-oid "$OBJECT_ID" \
    --features cluster-connect custom-locations
ok "Custom locations enabled"

# ── Install az iot ops extension ─────────────────────────────────────────────

step "Installing / upgrading azure-iot-ops CLI extension"
az extension add --upgrade --name azure-iot-ops
ok "azure-iot-ops extension ready"

# ── Create storage account and schema registry ────────────────────────────────

step "Creating storage account: $STORAGE_ACCOUNT"
az storage account create \
    --name "$STORAGE_ACCOUNT" \
    --location "$LOCATION" \
    --resource-group "$RESOURCE_GROUP" \
    --enable-hierarchical-namespace
ok "Storage account created"

step "Creating schema registry: $SCHEMA_REGISTRY"
az iot ops schema registry create \
    --name "$SCHEMA_REGISTRY" \
    --resource-group "$RESOURCE_GROUP" \
    --registry-namespace "$SCHEMA_REGISTRY_NAMESPACE" \
    --sa-resource-id "$(az storage account show --name "$STORAGE_ACCOUNT" -o tsv --query id)"
ok "Schema registry created"

# ── Create Azure Device Registry namespace ────────────────────────────────────

step "Creating Azure Device Registry namespace: myqsnamespace"
az iot ops ns create -n myqsnamespace -g "$RESOURCE_GROUP"
ok "Device Registry namespace created"

# ── Deploy Azure IoT Operations ───────────────────────────────────────────────

step "Initialising cluster for Azure IoT Operations (may take several minutes)"
az iot ops init --cluster "$CLUSTER_NAME" --resource-group "$RESOURCE_GROUP"
ok "Cluster initialised"

step "Deploying Azure IoT Operations (may take several minutes)"
az iot ops create \
    --cluster "$CLUSTER_NAME" \
    --resource-group "$RESOURCE_GROUP" \
    --name "${CLUSTER_NAME}-instance" \
    --sr-resource-id "$(az iot ops schema registry show --name "$SCHEMA_REGISTRY" --resource-group "$RESOURCE_GROUP" -o tsv --query id)" \
    --ns-resource-id "$(az iot ops ns show --name myqsnamespace --resource-group "$RESOURCE_GROUP" -o tsv --query id)" \
    --broker-frontend-replicas 1 \
    --broker-frontend-workers 1 \
    --broker-backend-part 1 \
    --broker-backend-workers 1 \
    --broker-backend-rf 2 \
    --broker-mem-profile Low
ok "Azure IoT Operations deployed"

# ── Verify ────────────────────────────────────────────────────────────────────

step "Checking pods in azure-iot-operations namespace"
kubectl get pods -n azure-iot-operations
ok "Pod list retrieved"

# ── Done ──────────────────────────────────────────────────────────────────────

echo
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN} All steps completed successfully!${NC}"
echo -e "${GREEN}========================================${NC}"
