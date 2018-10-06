extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

pub fn impl_loggable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Loggable for #name {
            fn log(str: &str) {
                let _mtx = LOGGABLE_MTX.lock().unwrap();
                println!("{}", format!("[ {} : {} ] {}", stringify!(#name).white().bold(), "Info".magenta().bold(), str.green()));
            }

            fn warn(str: &str) {
                let _mtx = LOGGABLE_MTX.lock().unwrap();
                println!("{}", format!("[ {} : {} ] {}", stringify!(#name).white().bold(), "Warning".magenta().bold(), str.yellow()));
            }

            fn err(str: &str) {
                let _mtx = LOGGABLE_MTX.lock().unwrap();
                println!("{}", format!("[ {} : {} ] {}", stringify!(#name).white().bold(), "Error".magenta().bold(), str.red()));
            }
        }
    };
    gen.into()
}
