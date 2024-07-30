
use proc_macro::TokenStream;
use quote::{quote,format_ident};
use syn::{Type,parse_macro_input, DeriveInput, Data, Fields,Meta};
use syn::spanned::Spanned;

/*
#[proc_macro_derive(Property, attributes(getter, setter))]
pub fn property_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        syn::Data::Struct(data) => {
            let getters_and_setters = data.fields.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;
                let mut getter = None;
                let mut setter = None;

                for attr in &field.attrs {
                    if attr.path().is_ident("getter") {
                        getter = Some(quote! {
                            pub fn #field_name(&self) -> &#field_type {
                                &self.#field_name
                            }
                        });
                    } else if attr.path().is_ident("setter") {
                        let setter_name = syn::Ident::new(&format!("set_{}", field_name.as_ref().unwrap()), field_name.span());
                        setter = Some(quote! {
                            pub fn #setter_name(&mut self, value: #field_type) {
                                self.#field_name = value;
                            }
                        });
                    }
                }

                quote! {
                    #getter
                    #setter
                }
            });

            quote! {
                impl #name {
                    #(#getters_and_setters)*
                }
            }
        }
        _ => quote! {},
    };

    TokenStream::from(expanded)
}*/

/*
#[proc_macro_derive(Property, attributes(get, set))]
pub fn property_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        syn::Data::Struct(data) => {
            let getters_and_setters = data.fields.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;
                let mut getter = None;
                let mut setter = None;

                for attr in &field.attrs {
                    if attr.path().is_ident("get") {
                        // Determine if the field type is a reference
                        let is_reference = matches!(field_type, syn::Type::Reference(_));
                        getter = Some(if is_reference {
                            // Field type is a reference
                            quote! {
                                pub fn #field_name(&self) -> &#field_type {
                                    &self.#field_name
                                }
                            }
                        } else {
                            // Field type is not a reference
                            quote! {
                                pub fn #field_name(&self) -> #field_type {
                                    self.#field_name.clone()
                                }
                            }
                        });
                    } else if attr.path().is_ident("set") {
                        let setter_name = syn::Ident::new(&format!("set_{}", field_name.as_ref().unwrap()), field_name.span());
                        setter = Some(quote! {
                            pub fn #setter_name(&mut self, value: #field_type) {
                                self.#field_name = value;
                            }
                        });
                    }
                }

                quote! {
                    #getter
                    #setter
                }
            });

            quote! {
                impl #name {
                    #(#getters_and_setters)*
                }
            }
        }
        _ => quote! {},
    };

    TokenStream::from(expanded)
}
*/




#[proc_macro_derive(Property, attributes(property))]
pub fn derive_property(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = if let Data::Struct(data) = &input.data {
        let fields = if let Fields::Named(fields) = &data.fields {
            fields.named.iter().map(|f| {
                let field_name = &f.ident;
                let field_type = &f.ty;
                let getter_name = field_name;
                let setter_name = format_ident!("set_{}", field_name.as_ref().unwrap());

                // Determine if the field type is a reference
                let is_reference = matches!(field_type, syn::Type::Reference(_));
                let getter = if is_reference {
                    // Field type is a reference
                    quote! {
                        pub fn #getter_name(&self) -> &#field_type {
                            &self.#field_name
                        }
                    }
                } else {
                    // Field type is not a reference
                    quote! {
                        pub fn #getter_name(&self) -> #field_type {
                            self.#field_name.clone()
                        }
                    }
                };

                quote! {
                    impl #name {
                        #getter

                        pub fn #setter_name(&mut self, value: #field_type) {
                            self.#field_name = value;
                        }
                    }
                }
            })
        } else {
            panic!("Property macro only supports named fields");
        };

        quote! {
            #(#fields)*
        }
    } else {
        panic!("Property macro only supports structs");
    };

    TokenStream::from(expanded)
}
