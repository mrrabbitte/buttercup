use proc_macro::TokenStream;
use quote::quote;

pub fn impl_macro_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Address for #name {
            fn new(id: i32, index: usize) -> #name {
                #name {id, index}
            }

            fn get_id(&self) -> &i32 {
                &self.id
            }

            fn get_index(&self) -> &usize {
                &self.index
            }
        }
    };
    gen.into()
}