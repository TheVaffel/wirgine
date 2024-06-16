// IsReprC + autoderivation
use core::iter::IntoIterator;
use proc_macro::TokenStream;
use proc_macro2 as p2;

use quote::quote;

#[proc_macro_derive(IsReprC)]
pub fn is_repr_c_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input.clone()).unwrap();
    let has_repr_c = ast.attrs.iter().any(|attr: &syn::Attribute| {
        if attr.style != syn::AttrStyle::Outer {
            return false;
        }

        if let syn::Meta::List(meta_list) = &attr.meta {
            let token_list: Vec<p2::TokenTree> = meta_list.tokens.clone().into_iter().collect();
            let is_repr = meta_list.path.get_ident().unwrap().to_string() == "repr";

            if token_list.len() < 1 {
                return false;
            }

            if let p2::TokenTree::Ident(repr_type) = &token_list[0] {
                return repr_type.to_string() == "C" && is_repr;
            }
        }

        false
    });

    assert!(has_repr_c, "Type does not implement Repr C");

    let struct_name = ast.ident;

    let expression = quote! {
        unsafe impl IsReprC for #struct_name { }
    };

    TokenStream::from(expression)
}
