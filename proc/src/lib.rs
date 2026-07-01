use proc_macro2::Span;
use quote::{format_ident, quote};
use std::{collections::HashSet, io::ErrorKind, path::PathBuf};

#[derive(serde::Deserialize)]
struct Resource {
    name: String,
}

#[derive(serde::Deserialize)]
struct Resources {
    media: Option<Vec<Resource>>,
}

#[derive(serde::Deserialize)]
struct Pebble {
    #[serde(rename = "messageKeys")]
    message_keys: Option<Vec<String>>,
    resources: Option<Resources>,
}

#[derive(serde::Deserialize)]
struct Package {
    pebble: Option<Pebble>,
}

fn load_package() -> Package {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let mut folder = Some(manifest_dir.as_path());
    while let Some(f) = folder {
        let mut path = PathBuf::from(f);
        path.push("package.json");
        match std::fs::read_to_string(path) {
            Ok(data) => return serde_json::from_str(&data).expect("Invalid package.json"),
            Err(e) => {
                if e.kind() != ErrorKind::NotFound {
                    panic!("Unexpected error locating package.json")
                }
            }
        };
        folder = f.parent();
    }

    panic!("Unable to find package.json for resource_ids/message_keys");
}

#[proc_macro]
pub fn resource_ids(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut token_iter = token_stream.into_iter();
    let Some(proc_macro::TokenTree::Ident(ident)) = token_iter.next() else {
        return quote! {
            compile_error!("Expected identifier, like resource_ids!(ResourceKeys)");
        }
        .into();
    };

    let package = load_package();

    let resources = package
        .pebble
        .and_then(|e| e.resources.map(|f| f.media))
        .flatten()
        .unwrap_or_default();

    let mut names = HashSet::<&str>::new();
    for resource in resources.iter() {
        names.insert(resource.name.as_str());
    }

    let idents = names
        .into_iter()
        .map(|f| proc_macro2::Ident::new(f, Span::call_site()))
        .collect::<Vec<_>>();

    let extern_decls = idents.iter().map(|name| {
        let key_ident = format_ident!("RESOURCE_ID_{}", name);
        quote! { static #key_ident: u32; }
    });

    let accessors = idents.iter().map(|name| {
        let key_ident = format_ident!("RESOURCE_ID_{}", name);
        quote! {
            pub static #name: &'static u32 = unsafe {&#key_ident};
        }
    });

    let ident: proc_macro2::Ident = proc_macro2::Ident::new(&ident.to_string(), Span::call_site());

    quote! {
        mod #ident {
            unsafe extern "C" {
                #(#extern_decls)*
            }
           #(#accessors)*
        }

    }
    .into()
}

#[proc_macro]
pub fn message_keys(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut token_iter = token_stream.into_iter();

    let Some(proc_macro::TokenTree::Ident(ident)) = token_iter.next() else {
        return quote! {
            compile_error!("Expected identifier, like message_keys!(ResourceKeys)");
        }
        .into();
    };

    let package = load_package();

    let message_keys = package
        .pebble
        .and_then(|e| e.message_keys)
        .unwrap_or_default();

    let variant_names = message_keys
        .iter()
        .map(|f| syn::Ident::new(f, Span::call_site()));

    let ident: proc_macro2::Ident =
        proc_macro2::Ident::new(&ident.to_string(), ident.span().into());

    let extern_decls = variant_names.clone().map(|name| {
        let key_ident = format_ident!("MESSAGE_KEY_{}", name);
        quote! { static #key_ident: u32; }
    });

    let accessors = variant_names.map(|name| {
        let key_ident = format_ident!("MESSAGE_KEY_{}", name);
        quote! {
            pub static #name: &'static u32 = unsafe {&#key_ident};
        }
    });

    quote! {
        mod #ident {
            unsafe extern "C" {
                #(#extern_decls)*
            }
           #(#accessors)*
        }

    }
    .into()
}
