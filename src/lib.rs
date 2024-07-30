
use proc_macro::TokenStream;
use quote::quote;
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
pub fn property_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        syn::Data::Struct(data) => {
            let getters_and_setters = data.fields.iter().filter_map(|field| {
                let field_name = field.ident.as_ref()?;
                let field_type = &field.ty;
                let mut getter = None;
                let mut setter = None;

                for attr in &field.attrs {
                    if attr.path().is_ident("property") {
                        if let Ok(_) = attr.parse_nested_meta(|meta| {
                            if let Ok(meta) = meta.value()?.parse::<Meta>() {
                                if let Meta::Path(path) = meta {
                                    if path.is_ident("get") {
                                        let is_reference = matches!(field_type, syn::Type::Reference(_));
                                        getter = Some(if is_reference {
                                            quote! {
                                                pub fn #field_name(&self) -> &#field_type {
                                                    &self.#field_name
                                                }
                                            }
                                        } else {
                                            quote! {
                                                pub fn #field_name(&self) -> #field_type {
                                                    self.#field_name.clone()
                                                }
                                            }
                                        });
                                    } else if path.is_ident("set") {
                                        let setter_name = syn::Ident::new(&format!("set_{}", field_name), field_name.span());
                                        setter = Some(quote! {
                                            pub fn #setter_name(&mut self, value: #field_type) {
                                                self.#field_name = value;
                                            }
                                        });
                                    }
                                }
                            }
                            Ok(())
                        }) {
                            // 属性の解析に成功した場合の処理
                        } else {
                            // 属性の解析に失敗した場合の処理
                            return None;
                        }
                    }
                }

                Some(quote! {
                    #getter
                    #setter
                })
            }).collect::<Vec<_>>();

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
