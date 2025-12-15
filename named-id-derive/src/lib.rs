use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::HashSet;
use syn::{Attribute, Data, DeriveInput, Fields, Index, parse_macro_input};

/// Check if a field or variant has the `#[nameables(skip)]` attribute
fn has_skip_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if attr.path().is_ident("named_id") {
            // Parse the tokens inside the parentheses as a path
            if let Ok(path) = attr.parse_args::<syn::Path>() {
                return path.is_ident("skip");
            }
        }
        false
    })
}

/// Collect all generic type parameter identifiers used in a type
fn collect_generic_params_in_type(
    ty: &syn::Type,
    generic_param_names: &HashSet<syn::Ident>,
) -> HashSet<syn::Ident> {
    let mut found = HashSet::new();

    match ty {
        syn::Type::Path(type_path) => {
            // Check if this is a direct reference to a generic parameter
            // (single segment with no arguments and no qself)
            if type_path.qself.is_none() && type_path.path.segments.len() == 1 {
                if let Some(segment) = type_path.path.segments.first() {
                    if segment.arguments.is_empty() {
                        let ident = &segment.ident;
                        if generic_param_names.contains(ident) {
                            found.insert(ident.clone());
                        }
                    } else if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        // Handle nested generics like Vec<X>, Option<Y>, HashMap<K, V>, etc.
                        for arg in &args.args {
                            match arg {
                                syn::GenericArgument::Type(nested_ty) => {
                                    found.extend(collect_generic_params_in_type(
                                        nested_ty,
                                        generic_param_names,
                                    ));
                                }
                                _ => {}
                            }
                        }
                    }
                }
            } else if let Some(last_segment) = type_path.path.segments.last() {
                // Handle paths with multiple segments like std::collections::HashMap<X, Y>
                if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    for arg in &args.args {
                        match arg {
                            syn::GenericArgument::Type(nested_ty) => {
                                found.extend(collect_generic_params_in_type(
                                    nested_ty,
                                    generic_param_names,
                                ));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        syn::Type::Tuple(tuple) => {
            for elem in &tuple.elems {
                found.extend(collect_generic_params_in_type(elem, generic_param_names));
            }
        }
        syn::Type::Array(array) => {
            found.extend(collect_generic_params_in_type(
                &array.elem,
                generic_param_names,
            ));
        }
        syn::Type::Reference(reference) => {
            found.extend(collect_generic_params_in_type(
                &reference.elem,
                generic_param_names,
            ));
        }
        syn::Type::Ptr(ptr) => {
            found.extend(collect_generic_params_in_type(
                &ptr.elem,
                generic_param_names,
            ));
        }
        syn::Type::Slice(slice) => {
            found.extend(collect_generic_params_in_type(
                &slice.elem,
                generic_param_names,
            ));
        }
        _ => {}
    }

    found
}

#[proc_macro_derive(RenameAll, attributes(named_id))]
pub fn derive_rename_all(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let impl_block = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    // For named fields, collect all field.nameables() and chain them
                    // Skip fields with #[named_id(skip)]
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
                    // If the variant itself has #[named_id(skip)], skip all its fields
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

    // Collect all generic type parameter names
    let generic_param_names: HashSet<_> = generics
        .params
        .iter()
        .filter_map(|param| {
            if let syn::GenericParam::Type(type_param) = param {
                Some(type_param.ident.clone())
            } else {
                None
            }
        })
        .collect();

    // Collect generic parameters used in non-skipped fields
    let mut used_generic_params = HashSet::new();

    match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                for field in fields.named.iter() {
                    if !has_skip_attr(&field.attrs) {
                        used_generic_params.extend(collect_generic_params_in_type(
                            &field.ty,
                            &generic_param_names,
                        ));
                    }
                }
            }
            Fields::Unnamed(fields) => {
                for field in fields.unnamed.iter() {
                    if !has_skip_attr(&field.attrs) {
                        used_generic_params.extend(collect_generic_params_in_type(
                            &field.ty,
                            &generic_param_names,
                        ));
                    }
                }
            }
            Fields::Unit => {}
        },
        Data::Enum(data_enum) => {
            for variant in &data_enum.variants {
                let variant_skip = has_skip_attr(&variant.attrs);
                if variant_skip {
                    continue;
                }

                match &variant.fields {
                    Fields::Named(fields) => {
                        for field in fields.named.iter() {
                            if !has_skip_attr(&field.attrs) {
                                used_generic_params.extend(collect_generic_params_in_type(
                                    &field.ty,
                                    &generic_param_names,
                                ));
                            }
                        }
                    }
                    Fields::Unnamed(fields) => {
                        for field in fields.unnamed.iter() {
                            if !has_skip_attr(&field.attrs) {
                                used_generic_params.extend(collect_generic_params_in_type(
                                    &field.ty,
                                    &generic_param_names,
                                ));
                            }
                        }
                    }
                    Fields::Unit => {}
                }
            }
        }
        Data::Union(_) => {}
    }

    // Add bounds to type parameters:
    // - Nameables bound only to type parameters used in non-skipped fields
    // - Debug bound to all type parameters (required by Nameables trait)
    let mut generics_with_bounds = generics.clone();
    for param in &mut generics_with_bounds.params {
        if let syn::GenericParam::Type(type_param) = param {
            // Always add Debug bound (required by Nameables trait)
            type_param.bounds.push(syn::parse_quote!(::std::fmt::Debug));

            // Add Nameables bound only if used in non-skipped fields
            if used_generic_params.contains(&type_param.ident) {
                type_param
                    .bounds
                    .push(syn::parse_quote!(named_id::Nameables));
            }
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

#[proc_macro_derive(RenameNone)]
pub fn derive_no_named(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    // Add Debug bound to all type parameters (required by Nameables trait)
    let mut generics_with_bounds = generics.clone();
    for param in &mut generics_with_bounds.params {
        if let syn::GenericParam::Type(type_param) = param {
            // Always add Debug bound (required by Nameables trait)
            type_param.bounds.push(syn::parse_quote!(::std::fmt::Debug));
        }
    }

    // Split generics for impl and where clause
    let (impl_generics, ty_generics, where_clause) = generics_with_bounds.split_for_impl();

    let expanded = quote! {
        impl #impl_generics named_id::Nameables for #name #ty_generics #where_clause {
            fn nameables(&self) -> ::std::vec::Vec<named_id::AnyNameable> {
                ::std::vec::Vec::new()
            }
        }
    };

    TokenStream::from(expanded)
}
