use proc_macro::TokenStream;
use quote::{format_ident, quote};
use shape::Shape;
use syn::{parse_macro_input, GenericArgument, PathArguments, Type};

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

        // Based on https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn/56264023#56264023
        let mut inner_typ = None;
        if let Type::Path(ref path) = field_type {
            let source = path
                .path
                .segments
                .iter()
                .fold("".to_string(), |mut acc, cur| {
                    acc.push_str(&cur.ident.to_string());
                    acc.push_str("::");
                    acc
                });

            let sources = vec![
                "Option::".to_string(),
                "std::option::Option::".to_string(),
                "core::option::Option::".to_string(),
            ];

            if sources.contains(&source) {
                // It is an option
                if let Some(segment) = path.path.segments.last() {
                    if let PathArguments::AngleBracketed(ref arguments) = segment.arguments {
                        if let Some(inner) = arguments.args.first() {
                            if let GenericArgument::Type(ref generic) = inner {
                                inner_typ = Some(generic);
                            }
                        }
                    }
                }
            }
        }

        fields.push(quote! {
            #field_name: #field_type,
        });

        if let Some(typ) = inner_typ {
            withs.push(quote! {
                pub fn #with_name(mut self, #field_name: #typ) -> Self {
                    self.#field_name = Some(#field_name);
                    self
                }
            });
        } else {
            withs.push(quote! {
                pub fn #with_name(mut self, #field_name: #field_type) -> Self {
                    self.#field_name = #field_name;
                    self
                }
            });
        }

        cloned.push(quote! {
            #field_name: self.#field_name,
        });
    }

    let expanded = quote! {
        #[derive(Debug, Clone)]
        #visibility struct #name {
            pub(crate) id: u32,
            #( #fields )*
        }

        impl #name {
            pub fn new(#( #params )*) -> Self {
                Self {
                    id: unsafe { crate::ID_FACTORY.next() },
                    #( #defaults )*
                }
            }

            #( #withs )*
        }
    };

    TokenStream::from(expanded)
}
