// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, TokenStream as TokenStream2};
use quote::{quote, ToTokens};

use std::env;
use std::path::PathBuf;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

/// WIT path adding macro
/// It accepts following input (the path must not be assigned):
/// - `wit_bindgen::generate!("world_name");`
/// - `wit_bindgen::generate!({world: "world_name"});`
/// - `wit_bindgen::generate!({world: "world_name"}, with: {...});`
/// - `wasmtime::component::bindgen!("world_name");`
/// - `wasmtime::component::bindgen!({world: "world_name", with: {...}});`
///
/// The path is added with the default crate wit path (`sdk_wit_path/graph`).
#[proc_macro_attribute]
pub fn with_default_path(attr: TokenStream, item: TokenStream) -> TokenStream {
    with_default_path2(attr.into(), item.into()).into()
}
fn with_default_path2(_attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    let path = get_default_wit_path();
    let wit_path = syn::LitStr::new(&path, proc_macro2::Span::call_site());
    let item = syn::parse2::<syn::Item>(item).unwrap();
    let item = match item {
        syn::Item::Macro(item) => {
            let mut new_mac = item.mac.clone();
            // Check if the macro path is `wit_bindgen::generate` or `wasmtime::component::bindgen`, otherwise return the item as it is
            let path = new_mac.path.clone().to_token_stream().to_string();
            if path != "wasmtime :: component :: bindgen" && path != "wit_bindgen :: generate" {
                return quote!(#item);
            }

            // Check if the macro token is just only `"world_name"`, if so add the default wit path to the macro
            if let proc_macro2::TokenTree::Literal(lit) =
                &new_mac.tokens.clone().into_iter().next().unwrap()
            {
                let tokens = quote!(#lit in #wit_path);
                let tokens = syn::parse2(tokens).unwrap();
                new_mac.tokens = tokens;
            }
            // Check if the macro is `wasmtime::component::bindgen!({world: "world_name", with: {...}});`,
            // if so add the path field and valued by default wit path
            else if let proc_macro2::TokenTree::Group(group) =
                &new_mac.tokens.clone().into_iter().next().unwrap()
            {
                if group.delimiter() == Delimiter::Brace {
                    let mut new_tokens = Vec::new();
                    let mut path_added = false;
                    let mut end_with_comma = false;
                    for token in group.clone().stream() {
                        end_with_comma = false;
                        if let proc_macro2::TokenTree::Punct(punct) = &token {
                            if punct.to_string() == "," {
                                end_with_comma = true;
                            }
                        }
                        if let proc_macro2::TokenTree::Ident(ident) = &token {
                            if *ident == "path" {
                                path_added = true;
                            }
                        }
                        new_tokens.push(token);
                    }
                    if !path_added {
                        if !end_with_comma {
                            new_tokens.extend(quote!(,));
                        }
                        new_tokens.extend(quote!(path: #wit_path));
                    }
                    new_mac.tokens =
                        Group::new(Delimiter::Brace, TokenStream2::from_iter(new_tokens))
                            .into_token_stream();
                }
            }

            // otheriwse return the item as it is

            syn::Item::Macro(syn::ItemMacro {
                attrs: item.attrs,
                ident: item.ident,
                mac: new_mac,
                semi_token: item.semi_token,
            })
        }
        _ => item,
    };
    quote!(#item)
}

/// WASMTIME bindgen macro
/// `wasmtime_bindgen!(world_name)` -> `wasmtime::component::bindgen!(world_name in default_wit_path);`
#[proc_macro]
pub fn wasmtime_bindgen(input: TokenStream) -> TokenStream {
    wasmtime_bindgen2(input.into()).into()
}
fn wasmtime_bindgen2(input: TokenStream2) -> TokenStream2 {
    let host_name = syn::parse2::<syn::LitStr>(input).unwrap();
    let wit_path = get_default_wit_path();
    let wit_path = syn::LitStr::new(&wit_path, proc_macro2::Span::call_site());
    quote!(wasmtime::component::bindgen!(#host_name in #wit_path);)
}

/// WIT-BINDGEN bindgen macro
/// `wit_bindgen!(world_name)` -> `wit_bindgen::generate!(world_name in default_wit_path);`
#[proc_macro]
pub fn wit_bindgen(input: TokenStream) -> TokenStream {
    wit_bindgen2(input.into()).into()
}
fn wit_bindgen2(input: TokenStream2) -> TokenStream2 {
    let world_name = syn::parse2::<syn::LitStr>(input).unwrap();
    let wit_path = get_default_wit_path();
    let wit_path = syn::LitStr::new(&wit_path, proc_macro2::Span::call_site());
    quote!(wit_bindgen::generate!(#world_name in #wit_path);)
}

/// Return the path to wit file/directory
/// env `WIT_PATH` is used to set the path to wit directory, else
/// `CARGO_MANIFEST_DIR` is used to build the path to wit directory (`graph/`)
#[proc_macro]
pub fn default_wit_path(_input: TokenStream) -> TokenStream {
    default_wit_path2().into()
}
fn default_wit_path2() -> TokenStream2 {
    let path = get_default_wit_path();
    let path = syn::LitStr::new(&path, proc_macro2::Span::call_site());
    quote!(#path)
}
// Return the path to wit file/directory
// env `SDK_WIT_PATH` is used to set the path to wit directory, else
// `CARGO_MANIFEST_DIR` is used to build the path to wit directory (`graph/`)
fn get_default_wit_path() -> String {
    if let Ok(sdk_wit_path) = std::env::var("SDK_WIT_PATH") {
        return sdk_wit_path;
    }
    PathBuf::from(CARGO_MANIFEST_DIR)
        .join("graph")
        .to_str()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_path() {
        let path = get_default_wit_path();
        assert_eq!(path, format!("{CARGO_MANIFEST_DIR}/graph"));
        // Store existing env path
        let existing_env = std::env::var("SDK_WIT_PATH");
        // Set new env path for test
        std::env::set_var("SDK_WIT_PATH", "/path/to/wit");
        let path = get_default_wit_path();
        assert_eq!(path, "/path/to/wit");
        // Reset the env path
        if let Ok(val) = existing_env {
            std::env::set_var("SDK_WIT_PATH", val);
        } else {
            std::env::remove_var("SDK_WIT_PATH");
        }
    }
    #[test]
    fn test_default_wit_path() {
        let path = default_wit_path2().to_string();
        assert_eq!(path, format!("\"{CARGO_MANIFEST_DIR}/graph\""));
    }
    #[test]
    fn test_wit_bindgen() {
        let input = "\"world_name\"".to_string();
        let output = wit_bindgen2(input.parse().unwrap()).to_string();
        let expected = format!(
            "wit_bindgen :: generate ! (\"world_name\" in \"{CARGO_MANIFEST_DIR}/graph\") ;"
        );
        assert_eq!(output, expected);
    }
    #[test]
    fn test_wasmtime_bindgen() {
        let input = "\"world_name\"".to_string();
        let output = wasmtime_bindgen2(input.parse().unwrap()).to_string();
        let expected = format!(
            "wasmtime :: component :: bindgen ! (\"world_name\" in \"{CARGO_MANIFEST_DIR}/graph\") ;"
        );
        assert_eq!(output, expected);
    }
    #[test]
    fn test_with_default_path_no_change_if_not_wit_bindgen_or_wasmtime_bindgen() {
        let input = "wasmtime :: component :: something_else ! (\"world_name\") ;";
        let output = with_default_path2(quote!(), input.parse().unwrap()).to_string();
        assert_eq!(output, input);
        let input = "something_else :: component :: generate ! (\"world_name\") ;";
        let output = with_default_path2(quote!(), input.parse().unwrap()).to_string();
        assert_eq!(output, input);
    }
    #[test]
    fn test_with_default_path_wit_bindgen_no_group() {
        let input = "wit_bindgen::generate!(\"world_name\");";
        let output = with_default_path2(quote!(), input.parse().unwrap()).to_string();
        let expected = format!(
            "wit_bindgen :: generate ! (\"world_name\" in \"{CARGO_MANIFEST_DIR}/graph\") ;"
        );
        assert_eq!(output, expected);
    }
    #[test]
    fn test_with_default_path_wasmtime_bindgen_no_group() {
        let input = "wasmtime::component::bindgen!(\"world_name\");";
        let output = with_default_path2(quote!(), input.parse().unwrap()).to_string();
        let expected = format!(
            "wasmtime :: component :: bindgen ! (\"world_name\" in \"{CARGO_MANIFEST_DIR}/graph\") ;"
        );
        assert_eq!(output, expected);
    }
    #[test]
    fn test_with_default_path_wit_bindgen_group() {
        let input = "wit_bindgen::generate!({world: \"world_name\"});";
        let output = with_default_path2(quote!(), input.parse().unwrap()).to_string();
        let expected = format!(
            "wit_bindgen :: generate ! ({{ world : \"world_name\" , path : \"{CARGO_MANIFEST_DIR}/graph\" }}) ;"
        );
        assert_eq!(output, expected);

        let input = "wit_bindgen::generate!({world: \"world_name\", });";
        let output = with_default_path2(quote!(), input.parse().unwrap()).to_string();
        let expected = format!(
            "wit_bindgen :: generate ! ({{ world : \"world_name\" , path : \"{CARGO_MANIFEST_DIR}/graph\" }}) ;"
        );
        assert_eq!(output, expected);

        let input = "wit_bindgen::generate!({world: \"world_name\", with:{}});";
        let output = with_default_path2(quote!(), input.parse().unwrap()).to_string();
        let expected = format!(
            "wit_bindgen :: generate ! ({{ world : \"world_name\" , with : {{ }} , path : \"{CARGO_MANIFEST_DIR}/graph\" }}) ;"
        );
        assert_eq!(output, expected);
    }
    #[test]
    fn test_with_default_path_wasmtime_bindgen_group() {
        let input = "wasmtime::component::bindgen!({world: \"world_name\"});";
        let output = with_default_path2(quote!(), input.parse().unwrap()).to_string();
        let expected = format!("wasmtime :: component :: bindgen ! ({{ world : \"world_name\" , path : \"{CARGO_MANIFEST_DIR}/graph\" }}) ;");
        assert_eq!(output, expected);

        let input = "wasmtime::component::bindgen!({world: \"world_name\", });";
        let output = with_default_path2(quote!(), input.parse().unwrap()).to_string();
        let expected = format!("wasmtime :: component :: bindgen ! ({{ world : \"world_name\" , path : \"{CARGO_MANIFEST_DIR}/graph\" }}) ;");
        assert_eq!(output, expected);

        let input = "wasmtime::component::bindgen!({world: \"world_name\", with:{}});";
        let output = with_default_path2(quote!(), input.parse().unwrap()).to_string();
        let expected = format!("wasmtime :: component :: bindgen ! ({{ world : \"world_name\" , with : {{ }} , path : \"{CARGO_MANIFEST_DIR}/graph\" }}) ;");
        assert_eq!(output, expected);
    }
}
