extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use proc_macro::TokenStream;

pub fn impl_loggable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Loggable for #name {
            fn _loggable_ident() -> &'static str { stringify!(#name) }
        }
    };
    gen.into()
}
