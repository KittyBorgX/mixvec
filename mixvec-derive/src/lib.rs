use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
#[proc_macro_derive(MixVecDerive)]
pub fn mixvec_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl CustomType for #name {}
        impl From<#name> for MixVecElement {
            fn from(item: #name) -> Self {
                MixVecElement::Custom(Box::new(item))
            }
        }
    };

    gen.into()
}
