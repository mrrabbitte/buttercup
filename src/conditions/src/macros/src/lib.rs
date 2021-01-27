extern crate proc_macro;

use proc_macro::TokenStream;

use syn;

mod relational;

#[proc_macro_derive(RelationalExpression, attributes(predicate))]
pub fn relational_expression_macro_derive(input: TokenStream) -> TokenStream {
    relational::impl_macro_derive(&syn::parse(input).unwrap())
}
