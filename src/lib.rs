#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, LitInt, Token};

struct ArrayInit {
    value: Expr,
    quantity: LitInt,
}

impl Parse for ArrayInit {
    fn parse(input: ParseStream) -> Result<Self> {
        let value: Expr = input.parse()?;
        input.parse::<Token![;]>()?;
        let quantity: LitInt = input.parse()?;
        Ok(ArrayInit {
            value,
            quantity,
        })
    }
}

#[proc_macro]
pub fn arr(input: TokenStream) -> TokenStream {
    let ArrayInit {
        value,
        quantity
    } = parse_macro_input!(input as ArrayInit);

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
