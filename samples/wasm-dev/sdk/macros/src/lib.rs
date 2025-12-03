// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use sdk_wit::default_wit_path;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, ItemFn};

struct OperatorInput {
    func: ItemFn,
}
impl Parse for OperatorInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let func: ItemFn = input.parse()?;
        Ok(OperatorInput { func })
    }
}
impl OperatorInput {
    #[allow(dead_code)]
    fn func(&self) -> &ItemFn {
        &self.func
    }
    #[allow(dead_code)]
    fn name(&self) -> &syn::Ident {
        &self.func.sig.ident
    }
    #[allow(dead_code)]
    fn input_type(&self) -> &syn::Type {
        let func_inputs = &self.func.sig.inputs;
        let func_input = func_inputs.first().unwrap();
        match func_input {
            syn::FnArg::Typed(pat_type) => &pat_type.ty,
            syn::FnArg::Receiver(_) => panic!("Expected a function argument"),
        }
    }
    #[allow(dead_code)]
    fn input_type_is(&self, ty: &str) -> bool {
        let input_type = self.input_type();
        match input_type {
            syn::Type::Path(type_path) => {
                let path = &type_path.path;
                let segments = &path.segments;
                let segment = segments.first().unwrap();
                let ident = &segment.ident;
                ident == ty
            }
            _ => false,
        }
    }
    #[allow(dead_code)]
    fn output_type(&self) -> &syn::Type {
        let func_output = &self.func.sig.output;
        match func_output {
            syn::ReturnType::Type(_, pat_type) => pat_type.as_ref(),
            syn::ReturnType::Default => panic!("Expected a function return type"),
        }
    }
    #[allow(dead_code)]
    fn output_type_is(&self, ty: &str) -> bool {
        let output_type = self.output_type();
        match output_type {
            syn::Type::Path(type_path) => {
                let path = &type_path.path;
                let segments = &path.segments;
                let segment = segments.first().unwrap();
                let ident = &segment.ident;
                ident == ty
            }
            _ => false,
        }
    }
    #[allow(dead_code)]
    fn output_type_is_option(&self) -> bool {
        let output_type = self.output_type();
        if let syn::Type::Path(type_path) = output_type {
            let path = &type_path.path;
            let segments = &path.segments;
            let segment = segments.first().unwrap();
            let ident = &segment.ident;
            ident == "Option"
        } else {
            false
        }
    }
    #[allow(dead_code)]
    fn output_type_is_option_of(&self, ty: &str) -> bool {
        let output_type = self.output_type();
        if let syn::Type::Path(type_path) = output_type {
            let path = &type_path.path;
            let segments = &path.segments;
            let segment = segments.first().unwrap();
            let ident = &segment.ident;
            if ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(syn::Type::Path(inner_type_path))) =
                        args.args.first()
                    {
                        if let Some(inner_segment) = inner_type_path.path.segments.first() {
                            return inner_segment.ident == ty;
                        }
                    }
                }
            }
        }
        false
    }
}

/// Get options for init/destroy encode/decode and wit path
/// `init="guest_init"` : with fn `guest_init(buffer) -> Result<_, String>`
/// `destroy="guest_destroy"` : with `fn guest_destroy()`
/// `decode="decode_input"` : with `fn decode_input(&DataModel) -> #input_type`
/// `encode="encode_output"` : with `fn encode_output(DataModel, #output_type) -> DataModel`
/// `wit="path/to/graph/"` : with path to wit graph directory, default is `../../wit/graph` to sdk-wit.
#[allow(dead_code)]
fn get_options(
    args: &syn::AttributeArgs,
    _op: &OperatorInput,
) -> (
    Option<Ident>,
    Option<Ident>,
    Option<Ident>,
    Option<Ident>,
    String,
) {
    let mut init: Option<Ident> = None;
    let mut destroy: Option<Ident> = None;
    let mut encode: Option<Ident> = None;
    let mut decode: Option<Ident> = None;
    let mut path = String::from(default_wit_path!());
    for arg in args {
        if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = arg {
            if name_value.path.is_ident("init") {
                if let syn::Lit::Str(lit_str) = &name_value.lit {
                    init = Some(Ident::new(
                        &lit_str.value(),
                        proc_macro::Span::call_site().into(),
                    ));
                }
            }
            if name_value.path.is_ident("destroy") {
                if let syn::Lit::Str(lit_str) = &name_value.lit {
                    destroy = Some(Ident::new(
                        &lit_str.value(),
                        proc_macro::Span::call_site().into(),
                    ));
                }
            }
            if name_value.path.is_ident("encode") {
                if let syn::Lit::Str(lit_str) = &name_value.lit {
                    encode = Some(Ident::new(
                        &lit_str.value(),
                        proc_macro::Span::call_site().into(),
                    ));
                }
            }
            if name_value.path.is_ident("decode") {
                if let syn::Lit::Str(lit_str) = &name_value.lit {
                    decode = Some(Ident::new(
                        &lit_str.value(),
                        proc_macro::Span::call_site().into(),
                    ));
                }
            }
            if name_value.path.is_ident("wit") {
                if let syn::Lit::Str(lit_str) = &name_value.lit {
                    path = lit_str.value();
                }
            }
        }
    }
    (init, destroy, decode, encode, path)
}

#[proc_macro_attribute]
pub fn map_operator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(_attr as syn::AttributeArgs);
    let operator_input = parse_macro_input!(item as OperatorInput);
    let func = operator_input.func();
    let func_name = &operator_input.name();
    let use_defaults = use_defaults_2(&TokenStream::new());
    let impl_defaults = impl_defaults_2(&TokenStream::new());
    let (_init_fn, _destroy_fn, decode_input, encode_output, wit_path) =
        get_options(&args, &operator_input);
    let process_code = if decode_input.is_none() && encode_output.is_none() {
        quote!(
            #func_name(input)
        )
    } else if decode_input.is_some() && encode_output.is_some() {
        if operator_input.output_type_is_option() {
            let decode_fn = decode_input.unwrap();
            let encode_fn = encode_output.unwrap();
            quote!(
                let user_input = #decode_fn(&input);
                let user_output = #func_name(user_input);
                match user_output {
                    Some(user_output) => #encode_fn(input, user_output),
                    None => input,
                }
            )
        } else {
            let decode_fn = decode_input.unwrap();
            let encode_fn = encode_output.unwrap();
            quote!(
                let user_input = #decode_fn(&input);
                let user_output = #func_name(user_input);
                #encode_fn(input, user_output)
            )
        }
    } else {
        panic!("Unsupported encode, decode or type settings");
    };
    // TODO: Implement lifecycle hooks
    // let init_code =
    //     if let Some(load) = init_fn {
    //         quote!(
    //             #load(param);
    //         )
    //     } else {
    //         quote!( Ok(()) )
    //     };
    // let destroy_code =
    //     if let Some(unload) = destroy_fn {
    //         quote!(
    //             #unload(param);
    //         )
    //     } else {
    //         quote!( )
    //     };

    let wit_path = syn::LitStr::new(&wit_path, proc_macro::Span::call_site().into());
    quote!(
        #func

        wit_bindgen::generate!({
            world: "map-impl",
            path: #wit_path,
        });
        #use_defaults
        #impl_defaults

        struct Module;

        impl exports::tinykube_graph::processor::map::Guest for Module {
            // TODO: Implement lifecycle hooks
            // fn load(param: Buffer) -> Result<(), String> {
            //     #init_code
            // }
            // fn unload(param: Buffer) {
            //     #destroy_code
            // }
            fn process(input: DataModel) -> DataModel {
                #process_code
            }
        }

        export!(Module);

    )
    .into()
}

#[proc_macro_attribute]
pub fn filter_operator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(_attr as syn::AttributeArgs);
    let operator_input = parse_macro_input!(item as OperatorInput);
    let func = operator_input.func();
    let func_name = &operator_input.name();
    let use_defaults = use_defaults_2(&TokenStream::new());
    let impl_defaults = impl_defaults_2(&TokenStream::new());
    let (_init_fn, _destroy_fn, decode_input, encode_output, wit_path) =
        get_options(&args, &operator_input);
    let process_code = if decode_input.is_none() && encode_output.is_none() {
        quote!(
            #func_name(input)
        )
    } else if decode_input.is_some() && encode_output.is_none() {
        let decode_fn = decode_input.unwrap();
        quote!(
            let user_input = #decode_fn(&input);
            #func_name(user_input)
        )
    } else {
        panic!("Unsupported encode, decode or type settings");
    };

    let wit_path = syn::LitStr::new(&wit_path, proc_macro::Span::call_site().into());
    quote!(

        #func

        wit_bindgen::generate!({
            world: "filter-impl",
            path: #wit_path,
        });
        #use_defaults
        #impl_defaults

        struct Module;

        impl exports::tinykube_graph::processor::filter::Guest for Module {
            fn process(input: DataModel) -> bool {
                #process_code
            }
        }

        export!(Module);

    )
    .into()
}

#[proc_macro_attribute]
pub fn branch_operator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(_attr as syn::AttributeArgs);
    let operator_input = parse_macro_input!(item as OperatorInput);
    let func = operator_input.func();
    let func_name = &operator_input.name();
    let use_defaults = use_defaults_2(&TokenStream::new());
    let impl_defaults = impl_defaults_2(&TokenStream::new());
    let (_init_fn, _destroy_fn, decode_input, encode_output, wit_path) =
        get_options(&args, &operator_input);
    let process_code = if decode_input.is_none() && encode_output.is_none() {
        quote!(
            #func_name(timestamp, input)
        )
    } else if decode_input.is_some() && encode_output.is_none() {
        let decode_fn = decode_input.unwrap();
        quote!(
            let user_input = #decode_fn(&input);
            #func_name(timestamp, user_input)
        )
    } else {
        panic!("Unsupported encode, decode or type settings");
    };

    let wit_path = syn::LitStr::new(&wit_path, proc_macro::Span::call_site().into());
    quote!(

        #func

        wit_bindgen::generate!({
            world: "branch-impl",
            path: #wit_path,
        });
        #use_defaults
        #impl_defaults

        struct Module;
        impl exports::tinykube_graph::processor::branch::Guest for Module {
            fn process(timestamp: HybridLogicalClock, input: DataModel) -> bool {
                #process_code
            }
        }
        export!(Module);

    )
    .into()
}

#[proc_macro_attribute]
pub fn accumulate_operator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(_attr as syn::AttributeArgs);
    let operator_input = parse_macro_input!(item as OperatorInput);
    let func = operator_input.func();
    let func_name = &operator_input.name();
    let use_defaults = use_defaults_2(&TokenStream::new());
    let impl_defaults = impl_defaults_2(&TokenStream::new());
    let (_init_fn, _destroy_fn, decode_input, encode_output, wit_path) =
        get_options(&args, &operator_input);
    let process_code = if decode_input.is_none() && encode_output.is_none() {
        quote!(
            #func_name(staged, input)
        )
    } else {
        panic!("Unsupported encode, decode or type settings");
    };
    // TODO: Implement lifecycle hooks
    // let init_code =
    //     if let Some(load) = init_fn {
    //         quote!(
    //             #load(param);
    //         )
    //     } else {
    //         quote!( Ok(()) )
    //     };
    // let destroy_code =
    //     if let Some(unload) = destroy_fn {
    //         quote!(
    //             #unload(param);
    //         )
    //     } else {
    //         quote!( )
    //     };

    let wit_path = syn::LitStr::new(&wit_path, proc_macro::Span::call_site().into());
    quote!(
        #func

        wit_bindgen::generate!({
            world: "accumulate-impl",
            path: #wit_path,
        });
        #use_defaults
        #impl_defaults

        struct Module;

        impl exports::tinykube_graph::processor::accumulate::Guest for Module {
            // TODO: Implement lifecycle hooks
            // fn load(param: Buffer) -> Result<(), String> {
            //     #init_code
            // }
            // fn unload(param: Buffer) {
            //     #destroy_code
            // }
            fn process(staged: DataModel, input: Vec<DataModel>) -> DataModel {
                #process_code
            }
        }

        export!(Module);

    )
    .into()
}

#[proc_macro_attribute]
pub fn delay_operator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(_attr as syn::AttributeArgs);
    let operator_input = parse_macro_input!(item as OperatorInput);
    let func = operator_input.func();
    let func_name = &operator_input.name();
    let use_defaults = use_defaults_2(&TokenStream::new());
    let impl_defaults = impl_defaults_2(&TokenStream::new());
    let (_init_fn, _destroy_fn, decode_input, encode_output, wit_path) =
        get_options(&args, &operator_input);
    let process_code = if decode_input.is_none() && encode_output.is_none() {
        quote!(
            #func_name(input, timestamp)
        )
    } else {
        panic!("Unsupported encode, decode or type settings");
    };

    let wit_path = syn::LitStr::new(&wit_path, proc_macro::Span::call_site().into());
    quote!(

        #func

        wit_bindgen::generate!({
            world: "delay-impl",
            path: #wit_path,
        });
        #use_defaults
        #impl_defaults

        struct Module;
        impl exports::tinykube_graph::processor::delay::Guest for Module {
            fn process(input: DataModel, timestamp: HybridLogicalClock) -> HybridLogicalClock {
                #process_code
            }
        }
        export!(Module);

    )
    .into()
}

// struct MacroInputIdents {
//     idents: Vec<Ident>,
// }
// impl Parse for MacroInputIdents {
//     fn parse(input: ParseStream<'_>) -> Result<Self> {
//         let mut idents = Vec::new();
//         while !input.is_empty() {
//             let ident: Ident = input.parse()?;
//             idents.push(ident);
//             if input.peek(Token![,]) {
//                 let _comma: Token![,] = input.parse()?;
//             }
//         }
//         Ok(MacroInputIdents { idents })
//     }
// }

/// Include All use statements for Types by default.
/// - `use std::time::Duration;`
/// - `use tinykube_graph::processor::types::*;`
/// - `use tinykube_graph::processor::hybrid_logical_clock::HybridLogicalClock;`
fn use_defaults_2(_input: &TokenStream) -> TokenStream2 {
    quote!(
        use std::time::Duration;
        use tinykube_graph::processor::types::*;
        use tinykube_graph::processor::hybrid_logical_clock::HybridLogicalClock;
    )
}

/// Include All impl for Types by default.
fn impl_defaults_2(input: &TokenStream) -> TokenStream2 {
    let impl_bufferorbytes = impl_bufferorbytes_2(input);
    let impl_bufferorstring = impl_bufferorstring_to_string_2(input);
    let impl_hybrid_logical_clock = impl_timestamp_to_hybrid_logical_clock_2(input);
    let impl_timespec_to_duration: TokenStream2 = impl_timespec_to_duration_2(input);
    quote!(
        #impl_bufferorbytes
        #impl_bufferorstring
        #impl_hybrid_logical_clock
        #impl_timespec_to_duration
    )
}

/// Implement `Timespec` to/from `Duration` with function(s):
/// - `pub fn from(timespec: Timespec) -> Duration`.
/// - `pub fn from(duration: Duration) -> Timespec`.
#[proc_macro]
pub fn impl_timespec_to_duration(input: TokenStream) -> TokenStream {
    impl_timespec_to_duration_2(&input).into()
}
fn impl_timespec_to_duration_2(_input: &TokenStream) -> TokenStream2 {
    let output = quote!(
        impl From<Timespec> for Duration {
            fn from(timespec: Timespec) -> Self {
                Duration::new(timespec.secs as u64, timespec.nanos as u32)
            }
        }
        impl From<Duration> for Timespec {
            fn from(duration: Duration) -> Self {
                Timespec {
                    secs: duration.as_secs() as u64,
                    nanos: duration.subsec_nanos() as u32,
                }
            }
        }
    );
    output
}
/// Implement `Timestamp` to `HybridLogicalClock` with function(s):
/// - `pub fn from(timestamp: Timestamp) -> Self`.
#[proc_macro]
pub fn impl_timestamp_to_hybrid_logical_clock(input: TokenStream) -> TokenStream {
    impl_timestamp_to_hybrid_logical_clock_2(&input).into()
}
fn impl_timestamp_to_hybrid_logical_clock_2(_input: &TokenStream) -> TokenStream2 {
    let output = quote!(
        impl From<Timestamp> for HybridLogicalClock {
            fn from(timestamp: Timestamp) -> Self {
                HybridLogicalClock {
                    timestamp: timestamp.timestamp,
                    counter: timestamp.counter,
                    node_id: timestamp.node_id.into(),
                }
            }
        }
    );
    output
}
/// Implement `BufferOrString` to `String` with function(s):
/// - `pub fn from(buffer_or_string: BufferOrString) -> Self`.
#[proc_macro]
pub fn impl_bufferorstring_to_string(input: TokenStream) -> TokenStream {
    impl_bufferorstring_to_string_2(&input).into()
}
fn impl_bufferorstring_to_string_2(_input: &TokenStream) -> TokenStream2 {
    let output = quote!(
        impl From<BufferOrString> for String {
            fn from(buffer_or_string: BufferOrString) -> Self {
                match buffer_or_string {
                    BufferOrString::Buffer(buffer) => String::from_utf8(buffer.read()).unwrap(),
                    BufferOrString::String(string) => string,
                }
            }
        }
    );
    output
}
/// Implement `BufferOrBytes` with function(s):
/// - `pub fn read(&self) -> Vec<u8>`.
#[proc_macro]
pub fn impl_bufferorbytes(input: TokenStream) -> TokenStream {
    impl_bufferorbytes_2(&input).into()
}
fn impl_bufferorbytes_2(_input: &TokenStream) -> TokenStream2 {
    let fn_read = quote!(
        pub fn read(&self) -> Vec<u8> {
            match self {
                BufferOrBytes::Buffer(buffer) => buffer.read(),
                BufferOrBytes::Bytes(bytes) => bytes.clone(),
            }
        }
    );
    let output = quote!(
        impl BufferOrBytes {
            #fn_read
        }
    );
    output
}

/// Implement `HybridLogicalClock` with function(s):
/// - `pub fn new() -> Self`.
#[proc_macro]
pub fn impl_hybrid_logical_clock(input: TokenStream) -> TokenStream {
    impl_hybrid_logical_clock_2(&input).into()
}
fn impl_hybrid_logical_clock_2(_input: &TokenStream) -> TokenStream2 {
    let fn_new = quote!(
        pub fn new() -> Self {
            hybrid_logical_clock_default()
        }
    );
    let output = quote!(
        impl HybridLogicalClock {
            #fn_new
        }
    );
    output
}
