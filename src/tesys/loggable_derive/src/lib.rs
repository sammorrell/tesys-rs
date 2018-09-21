use std::sync::Mutex;
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate lazy_static;

use proc_macro::TokenStream;

#[proc_macro_derive(Loggable)]
pub fn loggable_derive(input: TokenStream) -> TokenStream {
    // Construct a represntation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_loggable(&ast)
}

fn impl_loggable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Loggable for #name {
            fn log(str: &str) {
                //self.LOGGABLE_MTX.lock();
                println!("{}", format!("[ {} : {} ] {}", stringify!(#name), "Info", str));
            }

            fn warn(str: &str) {
                //self.LOGGABLE_MTX.lock();
                println!("{}", format!("[ {} : {} ] {}", stringify!(#name), "Warning", str));
            }

            fn err(str: &str) {
                //self.LOGGABLE_MTX.lock();
                println!("{}", format!("[ {} : {} ] {}", stringify!(#name), "Error", str));
            }
        }
    };
    gen.into()
}
