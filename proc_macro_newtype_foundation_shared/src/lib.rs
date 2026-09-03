#[cfg(test)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[allow(
    dead_code,
    reason = "the marker makes the integration-test-only derive dependency explicit"
)]
struct NewtypeFoundationTestDependencyMarker;

fn foundation_tuple_struct(
    derive_input: &syn::DeriveInput,
) -> syn::Result<(&syn::Type, &syn::Visibility)> {
    let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Unnamed(fields),
        ..
    }) = &derive_input.data
    else {
        return Err(syn::Error::new_spanned(
            derive_input,
            constants_str::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR,
        ));
    };
    if fields.unnamed.len() != 1usize {
        return Err(syn::Error::new_spanned(
            derive_input,
            constants_str::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR,
        ));
    }
    fields
        .unnamed
        .first()
        .map(|field| (&field.ty, &derive_input.vis))
        .ok_or_else(|| {
            syn::Error::new_spanned(
                derive_input,
                constants_str::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR,
            )
        })
}

pub fn foundation_as_ref_inner(token_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    (|| -> syn::Result<proc_macro2::TokenStream> {
        let parsed_input = syn::parse2::<syn::DeriveInput>(token_stream)?;
        let (inner_type, _visibility) = foundation_tuple_struct(&parsed_input)?;
        let (referenced_type, reference_expression) = if let syn::Type::Reference(reference) = inner_type {
            (&*reference.elem, quote::quote! { self.0 })
        } else {
            (inner_type, quote::quote! { &self.0 })
        };
        let identifier = &parsed_input.ident;
        let (implementation_generics, type_generics, where_clause) =
            parsed_input.generics.split_for_impl();
        Ok(quote::quote! {
            #[automatically_derived]
            impl #implementation_generics AsRef<#referenced_type> for #identifier #type_generics #where_clause {
                fn as_ref(&self) -> &#referenced_type {
                    #reference_expression
                }
            }
        })
    })()
    .unwrap_or_else(syn::Error::into_compile_error)
}

pub fn foundation_from_inner(token_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    (|| -> syn::Result<proc_macro2::TokenStream> {
        let parsed_input = syn::parse2::<syn::DeriveInput>(token_stream)?;
        let (inner_type, _visibility) = foundation_tuple_struct(&parsed_input)?;
        let identifier = &parsed_input.ident;
        let (implementation_generics, type_generics, where_clause) =
            parsed_input.generics.split_for_impl();
        Ok(quote::quote! {
            #[automatically_derived]
            impl #implementation_generics From<#inner_type> for #identifier #type_generics #where_clause {
                fn from(value: #inner_type) -> Self {
                    Self(value)
                }
            }
        })
    })()
    .unwrap_or_else(syn::Error::into_compile_error)
}

pub fn foundation_get_inner(token_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    (|| -> syn::Result<proc_macro2::TokenStream> {
        let parsed_input = syn::parse2::<syn::DeriveInput>(token_stream)?;
        let (inner_type, struct_visibility) = foundation_tuple_struct(&parsed_input)?;
        let mut visibility = struct_visibility.clone();
        let borrow = parsed_input
            .attrs
            .iter()
            .any(|attribute| attribute.path().is_ident(constants_str::VALUE_D106CCB1));
        parsed_input
            .attrs
            .iter()
            .filter(|attribute| attribute.path().is_ident(constants_str::ACCESSOR))
            .try_for_each(|attribute| {
                visibility = attribute.parse_args::<syn::Visibility>()?;
                Ok::<(), syn::Error>(())
            })?;
        let identifier = &parsed_input.ident;
        let (implementation_generics, type_generics, where_clause) =
            parsed_input.generics.split_for_impl();
        let receiver = if borrow {
            quote::quote! { &self }
        } else {
            quote::quote! { self }
        };
        Ok(quote::quote! {
            #[automatically_derived]
            impl #implementation_generics #identifier #type_generics #where_clause {
                #[must_use]
                #visibility const fn get(#receiver) -> #inner_type {
                    self.0
                }
            }
        })
    })()
    .unwrap_or_else(syn::Error::into_compile_error)
}

pub fn foundation_to_tokens(token_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    (|| -> syn::Result<proc_macro2::TokenStream> {
        let parsed_input = syn::parse2::<syn::DeriveInput>(token_stream)?;
        let (_inner_type, _visibility) = foundation_tuple_struct(&parsed_input)?;
        let identifier = &parsed_input.ident;
        let (implementation_generics, type_generics, where_clause) =
            parsed_input.generics.split_for_impl();
        Ok(quote::quote! {
            #[automatically_derived]
            impl #implementation_generics quote::ToTokens for #identifier #type_generics #where_clause {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    quote::ToTokens::to_tokens(&self.0, tokens);
                }
            }
        })
    })()
    .unwrap_or_else(syn::Error::into_compile_error)
}
