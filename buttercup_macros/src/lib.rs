extern crate proc_macro;

use proc_macro::TokenStream;

use syn;

use crate::conditions::relational;

mod address;
mod conditions;

#[proc_macro_derive(Address)]
pub fn address_macro_derive(input: TokenStream) -> TokenStream {
    address::impl_macro_derive(&syn::parse(input).unwrap())
}

#[proc_macro_derive(RelationalExpression, attributes(predicate))]
pub fn relational_expression_macro_derive(input: TokenStream) -> TokenStream {
    relational::impl_macro_derive(&syn::parse(input).unwrap())
}