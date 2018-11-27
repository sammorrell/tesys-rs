#![recursion_limit = "128"] // Maybe due to Nightly rust, but this needs to be added for recursion limit of stringify!
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

mod loggable;
mod routable;

// Provides the derive for the loggable class.
#[proc_macro_derive(Loggable)]
pub fn loggable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    loggable::impl_loggable(&ast)
}

#[proc_macro_derive(Routable)]
pub fn routable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    routable::impl_routable(&ast)
}