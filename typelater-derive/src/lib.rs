pub(crate) mod typelater;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Derive macro
#[proc_macro_derive(Typelater, attributes(typelater))]
pub fn typelater_derive(input: TokenStream) -> TokenStream {
    // Parse TokenStream into DeriveInput
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    // Call the internal function that returns proc_macro2::TokenStream
    let expanded = typelater::impl_typelater(derive_input);

    // Convert proc_macro2::TokenStream -> proc_macro::TokenStream for Rust
    expanded.into()
}
