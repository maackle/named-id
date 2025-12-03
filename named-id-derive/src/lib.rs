use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, Index, parse_macro_input};

/// Check if a field or variant has the `#[nameables(skip)]` attribute
fn has_skip_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if attr.path().is_ident("nameables") {
            // Parse the tokens inside the parentheses as a path
            if let Ok(path) = attr.parse_args::<syn::Path>() {
                return path.is_ident("skip");
            }
        }
        false
    })
}

#[proc_macro_derive(Nameables, attributes(nameables))]
pub fn derive_nameables(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let impl_block = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    // For named fields, collect all field.nameables() and chain them
                    // Skip fields with #[nameables(skip)]
                    let field_calls: Vec<_> = fields
                        .named
                        .iter()
                        .filter(|field| !has_skip_attr(&field.attrs))
                        .map(|field| {
                            let field_name = &field.ident;
                            quote! {
                                self.#field_name.nameables()
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
                    // For tuple structs, collect all field.nameables() and chain them
                    // Skip fields with #[nameables(skip)]
                    let field_calls: Vec<_> = fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .filter(|(_, field)| !has_skip_attr(&field.attrs))
                        .map(|(idx, _field)| {
                            let index = Index::from(idx);
                            quote! {
                                self.#index.nameables()
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
            // For enums, match on each variant and call nameables() on the inner values
            let match_arms: Vec<_> = data_enum
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = &variant.ident;
                    // If the variant itself has #[nameables(skip)], skip all its fields
                    let variant_skip = has_skip_attr(&variant.attrs);

                    match &variant.fields {
                        Fields::Named(fields) => {
                            if variant_skip {
                                quote! {
                                    #name::#variant_name { .. } => Vec::new(),
                                }
                            } else {
                                // Filter out fields with #[nameables(skip)]
                                let field_calls: Vec<_> = fields
                                    .named
                                    .iter()
                                    .filter(|f| !has_skip_attr(&f.attrs))
                                    .map(|f| {
                                        let field_name = &f.ident;
                                        quote! { #field_name.nameables() }
                                    })
                                    .collect();

                                // We still need to bind all fields in the pattern, even skipped ones
                                let all_field_names: Vec<_> =
                                    fields.named.iter().map(|f| &f.ident).collect();

                                if field_calls.is_empty() {
                                    quote! {
                                        #name::#variant_name { .. } => Vec::new(),
                                    }
                                } else {
                                    quote! {
                                        #name::#variant_name { #(#all_field_names,)* } => {
                                            let mut result = Vec::new();
                                            #(
                                                result.extend(#field_calls);
                                            )*
                                            result
                                        },
                                    }
                                }
                            }
                        }
                        Fields::Unnamed(fields) => {
                            if variant_skip || fields.unnamed.is_empty() {
                                if fields.unnamed.is_empty() {
                                    quote! {
                                        #name::#variant_name() => Vec::new(),
                                    }
                                } else {
                                    let field_idents: Vec<_> = (0..fields.unnamed.len())
                                        .map(|i| {
                                            syn::Ident::new(
                                                &format!("field_{}", i),
                                                Span::call_site(),
                                            )
                                        })
                                        .collect();
                                    quote! {
                                        #name::#variant_name(#(#field_idents,)*) => Vec::new(),
                                    }
                                }
                            } else {
                                // Filter out fields with #[nameables(skip)]
                                let field_calls: Vec<_> = fields
                                    .unnamed
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, field)| !has_skip_attr(&field.attrs))
                                    .map(|(i, _)| {
                                        let ident = syn::Ident::new(
                                            &format!("field_{}", i),
                                            Span::call_site(),
                                        );
                                        quote! { #ident.nameables() }
                                    })
                                    .collect();

                                // We still need to bind all fields in the pattern
                                let all_field_idents: Vec<_> = (0..fields.unnamed.len())
                                    .map(|i| {
                                        syn::Ident::new(&format!("field_{}", i), Span::call_site())
                                    })
                                    .collect();

                                if field_calls.is_empty() {
                                    quote! {
                                        #name::#variant_name(#(#all_field_idents,)*) => Vec::new(),
                                    }
                                } else {
                                    quote! {
                                        #name::#variant_name(#(#all_field_idents,)*) => {
                                            let mut result = Vec::new();
                                            #(
                                                result.extend(#field_calls);
                                            )*
                                            result
                                        },
                                    }
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
            return syn::Error::new_spanned(name, "Nameables cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    // Add Nameables bound to all type parameters
    let mut generics_with_bounds = generics.clone();
    for param in &mut generics_with_bounds.params {
        if let syn::GenericParam::Type(type_param) = param {
            type_param
                .bounds
                .push(syn::parse_quote!(named_id::Nameables));
        }
    }

    // Split generics for impl and where clause
    let (impl_generics, ty_generics, where_clause) = generics_with_bounds.split_for_impl();

    let expanded = quote! {
        impl #impl_generics named_id::Nameables for #name #ty_generics #where_clause {
            fn nameables(&self) -> ::std::vec::Vec<named_id::AnyNameable> {
                #impl_block
            }
        }
    };

    TokenStream::from(expanded)
}
