
use proc_macro::TokenStream;
use quote::{quote,format_ident};
use syn::{Lit,Type,parse_macro_input, DeriveInput, Data, Fields,Meta,Ident,Token, ItemImpl, ImplItem};
use syn::spanned::Spanned;
use syn::parse::ParseStream;

/*
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

                let mut generate_getter = false;
                let mut generate_setter = false;
                let mut generate_default = false;
                for attr in &f.attrs {
                    if attr.path().is_ident("property") {
                        attr.parse_args_with(|input: ParseStream| {
                            while !input.is_empty() {
                                let ident: Ident = input.parse()?;
                                if ident == "get" {
                                    generate_getter = true;
                                } else if ident == "set" {
                                    generate_setter = true;
                                }
                                if !input.is_empty() {
                                    input.parse::<Token![,]>()?;
                                }
                            }
                            Ok(())
                        }).unwrap();
                    }
                }

                let getter = if generate_getter {
                    if matches!(field_type, syn::Type::Reference(_)) {
                        quote! {
                            pub fn #getter_name(&self) -> &#field_type {
                                &self.#field_name
                            }
                        }
                    } else {
                        quote! {
                            pub fn #getter_name(&self) -> #field_type {
                                self.#field_name.clone()
                            }
                        }
                    }
                } else {
                    quote! {}
                };

                let setter = if generate_setter {
                    quote! {
                        pub fn #setter_name(&mut self, value: #field_type) {
                            self.#field_name = value;
                        }
                    }
                } else {
                    quote! {}
                };

                quote! {
                    impl #name {
                        #getter
                        #setter
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
}*/


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

                let mut generate_getter = false;
                let mut generate_setter = false;
                let mut default_value = None;

                for attr in &f.attrs {
                    if attr.path().is_ident("property") {
                        attr.parse_args_with(|input: ParseStream| {
                            while !input.is_empty() {
                                let ident: Ident = input.parse()?;
                                if ident == "get" {
                                    generate_getter = true;
                                } else if ident == "set" {
                                    generate_setter = true;
                                } else if ident == "default" {
                                    input.parse::<Token![=]>()?;
                                    let value: Lit = input.parse()?;
                                    default_value = Some(value);
                                }
                                if !input.is_empty() {
                                    input.parse::<Token![,]>()?;
                                }
                            }
                            Ok(())
                        }).unwrap();
                    }
                }

                let getter = if generate_getter {
                    if matches!(field_type, syn::Type::Reference(_)) {
                        quote! {
                            pub fn #getter_name(&self) -> &#field_type {
                                &self.#field_name
                            }
                        }
                    } else {
                        quote! {
                            pub fn #getter_name(&self) -> #field_type {
                                self.#field_name.clone()
                            }
                        }
                    }
                } else {
                    quote! {}
                };

                let setter = if generate_setter {
                    quote! {
                        pub fn #setter_name(&mut self, value: #field_type) {
                            self.#field_name = value;
                        }
                    }
                } else {
                    quote! {}
                };

                let default_impl = if let Some(default_value) = default_value {
                    quote! {
                        impl Default for #name {
                            fn default() -> Self {
                                Self {
                                    #field_name: #default_value,
                                    ..Default::default()
                                }
                            }
                        }
                    }
                } else {
                    quote! {}
                };

                quote! {
                    impl #name {
                        #getter
                        #setter
                    }
                    #default_impl
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
