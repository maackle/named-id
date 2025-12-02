use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, parse_macro_input};

#[proc_macro_derive(ContainsAliases)]
pub fn derive_contains_aliases(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let impl_block = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    // For named fields, collect all field.aliased_ids() and chain them
                    let field_calls: Vec<_> = fields
                        .named
                        .iter()
                        .map(|field| {
                            let field_name = &field.ident;
                            quote! {
                                self.#field_name.aliased_ids()
                            }
                        })
                        .collect();

                    if field_calls.is_empty() {
                        quote! {
                            Vec::new()
                        }
                    } else {
                        quote! {
                            {
                                let mut result = Vec::new();
                                #(
                                    result.extend(#field_calls);
                                )*
                                result
                            }
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    // For tuple structs, collect all field.aliased_ids() and chain them
                    let field_calls: Vec<_> = fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(idx, _field)| {
                            let index = Index::from(idx);
                            quote! {
                                self.#index.aliased_ids()
                            }
                        })
                        .collect();

                    if field_calls.is_empty() {
                        quote! {
                            Vec::new()
                        }
                    } else {
                        quote! {
                            {
                                let mut result = Vec::new();
                                #(
                                    result.extend(#field_calls);
                                )*
                                result
                            }
                        }
                    }
                }
                Fields::Unit => {
                    // Unit structs have no fields
                    quote! {
                        Vec::new()
                    }
                }
            }
        }
        Data::Enum(data_enum) => {
            // For enums, match on each variant and call aliased_ids() on the inner values
            let match_arms: Vec<_> = data_enum
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = &variant.ident;
                    match &variant.fields {
                        Fields::Named(fields) => {
                            let field_names: Vec<_> =
                                fields.named.iter().map(|f| &f.ident).collect();
                            let field_calls: Vec<_> = field_names
                                .iter()
                                .map(|field_name| {
                                    quote! {
                                        #field_name.aliased_ids()
                                    }
                                })
                                .collect();

                            if field_calls.is_empty() {
                                quote! {
                                    #name::#variant_name { .. } => Vec::new(),
                                }
                            } else {
                                quote! {
                                    #name::#variant_name { #(#field_names,)* } => {
                                        let mut result = Vec::new();
                                        #(
                                            result.extend(#field_calls);
                                        )*
                                        result
                                    },
                                }
                            }
                        }
                        Fields::Unnamed(fields) => {
                            if fields.unnamed.is_empty() {
                                quote! {
                                    #name::#variant_name() => Vec::new(),
                                }
                            } else {
                                let field_idents: Vec<_> = (0..fields.unnamed.len())
                                    .map(|i| {
                                        syn::Ident::new(&format!("field_{}", i), Span::call_site())
                                    })
                                    .collect();
                                let field_calls: Vec<_> = field_idents
                                    .iter()
                                    .map(|ident| {
                                        quote! {
                                            #ident.aliased_ids()
                                        }
                                    })
                                    .collect();
                                quote! {
                                    #name::#variant_name(#(#field_idents,)*) => {
                                        let mut result = Vec::new();
                                        #(
                                            result.extend(#field_calls);
                                        )*
                                        result
                                    },
                                }
                            }
                        }
                        Fields::Unit => {
                            quote! {
                                #name::#variant_name => Vec::new(),
                            }
                        }
                    }
                })
                .collect();

            quote! {
                match self {
                    #(#match_arms)*
                }
            }
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(name, "ContainsAliases cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        impl crate::ContainsAliases for #name {
            fn aliased_ids(&self) -> ::std::vec::Vec<crate::AnyAlias> {
                #impl_block
            }
        }
    };

    TokenStream::from(expanded)
}
