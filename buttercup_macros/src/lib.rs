extern crate proc_macro;

use proc_macro::TokenStream;

use syn;

mod address;

#[proc_macro_derive(Address)]
pub fn address_macro_derive(input: TokenStream) -> TokenStream {
    address::impl_macro_derive(&syn::parse(input).unwrap())
}