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

struct ParsedColor {
    value: u8, // argb
}

fn parse_hex_digit(b1: u8, b2: u8) -> Option<u8> {
    let bytes = [b1, b2];
    let str = str::from_utf8(&bytes).ok()?;
    u8::from_str_radix(str, 16).ok()
}

fn get_2bit_value(value: u8) -> Option<u8> {
    Some(match value {
        0xff => 0b11,
        0xaa => 0b10,
        0x55 => 0b01,
        0x00 => 0,
        _ => return None,
    })
}

fn make_argb(a: u8, r: u8, g: u8, b: u8) -> Option<u8> {
    Some(
        (get_2bit_value(a)? << 6)
            + (get_2bit_value(r)? << 4)
            + (get_2bit_value(g)? << 2)
            + get_2bit_value(b)?,
    )
}

fn parse_hex_literal(mut literal: &str) -> Option<ParsedColor> {
    literal = literal
        .trim_start_matches('"')
        .trim_start_matches('#')
        .trim_end_matches('"');

    let digits: Vec<u8> = literal.as_bytes().iter().map(ToOwned::to_owned).collect();
    let value;
    match digits.len() {
        3 => {
            let r = parse_hex_digit(digits[0], digits[0])?;
            let g = parse_hex_digit(digits[1], digits[1])?;
            let b = parse_hex_digit(digits[2], digits[2])?;
            value = make_argb(0xff, r, g, b);
        }
        4 => {
            let r = parse_hex_digit(digits[0], digits[0])?;
            let g = parse_hex_digit(digits[1], digits[1])?;
            let b = parse_hex_digit(digits[2], digits[2])?;
            let a = parse_hex_digit(digits[3], digits[3])?;
            value = make_argb(a, r, g, b);
        }
        6 => {
            let r = parse_hex_digit(digits[0], digits[1])?;
            let g = parse_hex_digit(digits[2], digits[3])?;
            let b = parse_hex_digit(digits[4], digits[5])?;
            value = make_argb(0xff, r, g, b);
        }
        8 => {
            let r = parse_hex_digit(digits[0], digits[1])?;
            let g = parse_hex_digit(digits[2], digits[3])?;
            let b = parse_hex_digit(digits[4], digits[5])?;
            let a = parse_hex_digit(digits[6], digits[7])?;
            value = make_argb(a, r, g, b);
        }
        _ => return None,
    }

    let value = value.expect("Hex codes can only include ff, aa, 55, or 00 digits.");
    Some(ParsedColor { value })
}

#[proc_macro]
pub fn hex_color(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut token_iter = token_stream.into_iter();

    let Some(proc_macro::TokenTree::Literal(literal)) = token_iter.next() else {
        return quote! {
            compile_error!("Expected hex string like: hex_color!(\"#ff00ff\")");
        }
        .into();
    };

    if token_iter.next().is_some() {
        return quote! {
            compile_error!("Unexpected additional token after hex code");
        }
        .into();
    }

    let parsed: ParsedColor =
        parse_hex_literal(&literal.to_string()).expect("Hex code failed to parse");

    let argb_token = proc_macro2::Literal::u8_suffixed(parsed.value);

    quote! {
        pebble_rust_2026::GColor{
            argb: #argb_token,
        }
    }
    .into()
}
