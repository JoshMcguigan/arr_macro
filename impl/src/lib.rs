#![forbid(unsafe_code)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::{parse_macro_input, Token};

use quote::quote;
use syn::Expr;
use syn::LitInt;

struct ArrayInit {
    value: Expr,
    quantity: LitInt,
}

impl Parse for ArrayInit {
    fn parse(input: ParseStream) -> Result<Self> {
        let value: Expr = input.parse()?;
        input.parse::<Token![;]>()?;
        let quantity: LitInt = input.parse()?;
        Ok(ArrayInit { value, quantity })
    }
}

#[proc_macro_hack]
pub fn arr(input: TokenStream) -> TokenStream {
    let ArrayInit { value, quantity } = parse_macro_input!(input as ArrayInit);

    let iter = std::iter::repeat(value).take(
        quantity
            .base10_parse::<usize>()
            .expect("Failed to parse LitInt as usize"),
    );

    let result = quote! {
        [#(#iter),*]
    };
    result.into()
}
