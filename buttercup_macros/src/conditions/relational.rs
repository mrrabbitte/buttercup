
use quote::quote;
use syn::{Attribute, Meta, NestedMeta, Ident};
use proc_macro::TokenStream;

pub fn impl_macro_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let value_holders_predicate = get_predicate(ast);
    let gen = quote! {

        impl #name {
            pub fn new(specification: RelationalExpressionSpecification) -> #name {
                #name { specification }
            }
        }

        impl ValuesPayloadPredicateSupplier for #name {
            fn get_predicate(self) -> Box<dyn Fn(&ValuesPayload) -> bool + Send + Sync> {
                match self.specification {
                    RelationalExpressionSpecification::NameAndName(first, second) =>
                        Box::new(move |payload|
                            match (payload.get(&first), payload.get(&second)) {
                                (Some(left), Some(right)) =>
                                    left.#value_holders_predicate(right),
                                (_, _) => false
                        }),
                    RelationalExpressionSpecification::NameAndLiteral(name, right) =>
                        Box::new(move |payload|
                            match payload.get(&name) {
                                Some(left) =>
                                    left.#value_holders_predicate(&right),
                                _ => false
                        }),
                    RelationalExpressionSpecification::LiteralAndName(left, name) =>
                        Box::new(move |payload|
                            match payload.get(&name) {
                                Some(right) =>
                                    (&left).#value_holders_predicate(right),
                                _ => false
                        }),
                }
            }

            fn get_value_names(&self) -> Vec<String> {
                self.specification.get_value_names()
            }
        }
    };
    gen.into()
}

fn get_predicate(ast: &syn::DeriveInput) -> Ident {
    let predicate = ast.attrs.iter()
        .filter(|attr| attr.path.segments[0].ident == "predicate")
        .nth(0)
        .expect("Could not find predicate attribute to derive RelationalExpression.");
    match Attribute::parse_meta(predicate) {
        Ok(meta) => match meta {
            Meta::List(meta_list) =>
                match meta_list.nested.first() {
                    None => panic!("Got an empty predicate attribute: {:?}", meta_list),
                    Some(nested) => match nested {
                        NestedMeta::Meta(meta) => match meta {
                            Meta::Path(path) => match path.segments.first() {
                                None => panic!("Got empty Path: {:?}", meta),
                                Some(segment) => segment.ident.clone().into()
                            },
                            _ => panic!("Expected Path, got: {:?}", meta)
                        },
                        _ => panic!("Expected Meta, got: {:?}", nested)
                    }
                },
            _ => panic!(
                "Expected name value attribute for RelationalExpression, got: {:?} instead.", meta)
        },
        Err(err) => panic!("Could not parse Meta for: {:?}, reason: {:?}", predicate, err)
    }
}