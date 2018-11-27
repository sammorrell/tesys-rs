extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use proc_macro::TokenStream;

pub fn impl_routable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Routable for #name {
            fn recv(&mut self){
            	println!("{}", self.test_field);
            }
        }
    };
    gen.into()
}
