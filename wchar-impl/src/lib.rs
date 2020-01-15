extern crate proc_macro;

use std::iter::once;

use proc_macro_hack::proc_macro_hack;
use syn::{Ident, LitStr, Token};
use syn::parse::{Parse, ParseStream, Result};

struct Input {
    ty: Option<Ident>,
    string: LitStr,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ty = Some(input.parse()?);
            let _ = input.parse::<Token![,]>();
            let string = input.parse()?;
            Ok(Input { ty, string })
        } else {
            let ty = None;
            let string = input.parse()?;
            Ok(Input { ty, string })
        }
    }
}

#[proc_macro_hack]
pub fn wch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input {ty, string} = syn::parse_macro_input!(input);

    let expanded = match ty {
        Some(ident) if ident == &"i16" => {
            let bytes = i16::encode_wide(&string.value());
            quote::quote! { &[#(#bytes),*] }
        }
        Some(ident) if ident == &"u32" => {
            let bytes = u32::encode_wide(&string.value());
            quote::quote! { &[#(#bytes),*] }
        }
        Some(ident) if ident == &"i32" => {
            let bytes = i32::encode_wide(&string.value());
            quote::quote! { &[#(#bytes),*] }
        }
        _ => {
            let bytes = u16::encode_wide(&string.value());
            quote::quote! { &[#(#bytes),*] }
        }
    };

    expanded.into()
}

#[proc_macro_hack]
pub fn wch_c(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input {ty, string} = syn::parse_macro_input!(input);

    if string.value().contains('\0') {
        return syn::Error::new(string.span(), "C-style string cannot contain nul characters")
            .to_compile_error()
            .into();
    }

    let expanded = match ty {
        Some(ident) if ident == &"i16" => {
            let bytes = i16::encode_wide_c(&string.value());
            quote::quote! { &[#(#bytes),*] }
        }
        Some(ident) if ident == &"u32" => {
            let bytes = u32::encode_wide_c(&string.value());
            quote::quote! { &[#(#bytes),*] }
        }
        Some(ident) if ident == &"i32" => {
            let bytes = i32::encode_wide_c(&string.value());
            quote::quote! { &[#(#bytes),*] }
        }
        _ => {
            let bytes = u16::encode_wide_c(&string.value());
            quote::quote! { &[#(#bytes),*] }
        }
    };

    expanded.into()
}

trait Encode: Sized {
    fn encode_wide(s: &str) -> Vec<Self>;
    fn encode_wide_c(s: &str) -> Vec<Self>;
}

impl Encode for u16 {
    fn encode_wide(s: &str) -> Vec<u16> {
        s.encode_utf16().collect()
    }
    fn encode_wide_c(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(once(0)).collect()
    }
}

impl Encode for i16 {
    fn encode_wide(s: &str) -> Vec<i16> {
        s.encode_utf16().map(|x| x as i16).collect()
    }
    fn encode_wide_c(s: &str) -> Vec<i16> {
        s.encode_utf16().chain(once(0)).map(|x| x as i16).collect()
    }
}

impl Encode for u32 {
    fn encode_wide(s: &str) -> Vec<u32> {
        s.chars().map(|x| x as u32).collect()
    }
    fn encode_wide_c(s: &str) -> Vec<u32> {
        s.chars().chain(once('\u{0}')).map(|x| x as u32).collect()
    }
}

impl Encode for i32 {
    fn encode_wide(s: &str) -> Vec<i32> {
        s.chars().map(|x| x as i32).collect()
    }
    fn encode_wide_c(s: &str) -> Vec<i32> {
        s.chars().chain(once('\u{0}')).map(|x| x as i32).collect()
    }
}
