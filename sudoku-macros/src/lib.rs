use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, spanned::Spanned};

#[proc_macro]
pub fn array_clone(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ArrayClone);
    input.expand()
}

struct ArrayClone {
    expr: syn::Expr,
    _semi: syn::Token![;],
    num: syn::LitInt,
}

impl ArrayClone {
    fn expand(&self) -> TokenStream {
        let num = self.num.base10_parse::<usize>().unwrap();
        let expr = &self.expr;
        let mut body = Vec::with_capacity(num);
        for _ in 0..num {
            body.push(quote_spanned!(expr.span()=> #expr));
        }
        quote_spanned! {
            self.expr.span()=>
            {
                [#(#body),*]
            }
        }
        .into()
    }
}

impl syn::parse::Parse for ArrayClone {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(ArrayClone {
            expr: input.parse()?,
            _semi: input.parse()?,
            num: input.parse()?,
        })
    }
}
