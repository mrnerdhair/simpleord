extern crate proc_macro;
use proc_macro::TokenStream;
use syn;
use quote::quote;

#[proc_macro_derive(SimpleOrd)]
pub fn simple_ord(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    TokenStream::from(quote! {
        impl ::std::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                ::std::option::Option::Some(self.cmp(other))
            }
        }

        impl ::std::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other) == ::std::cmp::Ordering::Equal
            }
        }

        impl ::std::cmp::Eq for #name {}
    })
}

#[proc_macro_derive(SimplerOrd)]
pub fn simpler_ord(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    TokenStream::from(quote! {
        impl ::std::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.partial_cmp(other) == ::std::option::Option::Some(::std::cmp::Ordering::Equal)
            }
        }
    })
}
