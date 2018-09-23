extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod loggable;

// Provides the derive for the loggable class.
#[proc_macro_derive(Loggable)]
pub fn loggable_derive(input: TokenStream) -> TokenStream {
    // Construct a represntation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    loggable::impl_loggable(&ast)
}
