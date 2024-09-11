use proc_macro::TokenStream;
use quote::{format_ident, quote};
use shape::Shape;
use syn::parse_macro_input;

mod shape;

#[proc_macro]
pub fn shape(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Shape);

    let visibility = input.visibility;
    let name = input.name;

    let mut params = Vec::new();
    let mut defaults = Vec::new();
    let mut fields = Vec::new();
    let mut withs = Vec::new();
    let mut cloned = Vec::new();
    for field in input.fields {
        let field_name = field.name;
        let field_type = field.typ;

        let with_name = format_ident!("with_{}", field_name);

        if let Some(default) = &field.default {
            let value = &default.value;
            defaults.push(quote! {
                #field_name: #value,
            });
        } else {
            params.push(quote! {
                #field_name: #field_type,
            });
            defaults.push(quote! {
                #field_name,
            });
        }

        fields.push(quote! {
            #field_name: #field_type,
        });
        withs.push(quote! {
            pub fn #with_name(mut self, #field_name: #field_type) -> Self {
                self.#field_name = #field_name;
                self
            }
        });
        cloned.push(quote! {
            #field_name: self.#field_name,
        });
    }

    let expanded = quote! {
        #[derive(Debug, Clone)]
        #visibility struct #name {
            #( #fields )*
        }

        impl #name {
            pub fn new(#( #params )*) -> Self {
                Self {
                    #( #defaults )*
                }
            }

            #( #withs )*
        }
    };

    TokenStream::from(expanded)
}
