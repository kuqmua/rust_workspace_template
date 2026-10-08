#[must_use]
pub fn generate_serde_version_of_named_syn_variant(
    syn_variant_ref: crate::syn_variant_ref::SynVariantRef<'_>,
) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let variant = syn_variant_ref.variant();
    let hash_map_upper_camel_case = naming::hash_map_upper_camel_case::HashMapUpperCamelCase;
    let location_snake_case = naming::domain_types::LocationSnakeCase;
    let string_token_stream = token_patterns::StringTokenStream;
    let with_serde_upper_camel_case = naming::domain_types::WithSerdeUpperCamelCase;
    let element_identifier = &variant.ident;
    let fields = if let syn::Fields::Named(fields) = &variant.fields {
        &fields.named
    } else {
        return crate::macro_compile_error_tokens::macro_compile_error_tokens(
            crate::compile_error_message::CompileErrorMessage::from(
                constants_str::MACRO_DIAGNOSTICS_EXPECTED_NAMED_VARIANT_FIELDS_ERROR,
            ),
        );
    };
    let fields_with_serde_token_stream = fields.iter().map(|element| {
        let Some(element_c25b655e_identifier) = element.ident.as_ref() else {
            return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                constants_str::MACRO_DIAGNOSTICS_EXPECTED_NAMED_FIELD_ERROR,
            ));
        };
        let tokens = if *element_c25b655e_identifier == *location_snake_case.to_string() {
            quote::quote! {#location_snake_case: location_lib::location::Location}
        } else {
            let get_hashmap_args = || {
                let segments = if let syn::Type::Path(syn_type_path) = &element.ty {
                    &syn_type_path.path.segments
                } else {
                    return None;
                };
                let last_segment = segments.iter().next_back()?;
                if last_segment.ident != hash_map_upper_camel_case.to_string() {
                    return None;
                }
                let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    args,
                    ..
                }) = &last_segment.arguments
                else {
                    return None;
                };
                if args.len() != 2 {
                    return None;
                }
                Some((args.iter().next()?, args.iter().nth(1)?))
            };
            let element_type_token_stream = {
                let element_type = &element.ty;
                quote::quote! {#element_type}
            };
            let location_field_attr = match crate::location_field_attr::LocationFieldAttr::try_from(element) {
                Ok(parsed_attr) => parsed_attr,
                Err(error) => return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                    &constants_str::COMPILE_ERROR_CE_010.replace(
                        constants_str::COMPILE_ERROR_ERROR_PLACEHOLDER,
                        &error,
                    ),
                )),
            };
            let element_type_with_serde_token_stream = match location_field_attr {
                crate::location_field_attr::LocationFieldAttr::ErrorFieldToErrString => quote::quote! {#string_token_stream},
                crate::location_field_attr::LocationFieldAttr::ErrorFieldToErrStringSerde | crate::location_field_attr::LocationFieldAttr::ErrorFieldVecToErrStringSerde => {
                    element_type_token_stream
                }
                crate::location_field_attr::LocationFieldAttr::ErrorFieldLocation => match format!("{element_type_token_stream}{with_serde_upper_camel_case}")
                    .parse::<proc_macro2::TokenStream>()
                {
                    Ok(parsed_token_stream) => parsed_token_stream,
                    Err(error) => {
                        return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                            &constants_str::COMPILE_ERROR_CE_005
                                .replace(
                                    constants_str::COMPILE_ERROR_ERROR_PLACEHOLDER,
                                    &error.to_string(),
                                ),
                        ));
                    }
                },
                crate::location_field_attr::LocationFieldAttr::ErrorFieldVecToErrString => {
                    quote::quote! {
                        Vec<#string_token_stream>
                    }
                }
                crate::location_field_attr::LocationFieldAttr::ErrorFieldVecLocation => {
                    let segments = if let syn::Type::Path(v0) = &element.ty {
                        &v0.path.segments
                    } else {
                        return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(constants_str::COMPILE_ERROR_CE_024));
                    };
                    if segments.len() != 1 {
                        return crate::macro_compile_error_tokens::macro_compile_error_tokens(
                            crate::compile_error_message::CompileErrorMessage::from(
                                constants_str::COMPILE_ERROR_CE_024,
                            ),
                        );
                    }
                    let Some(first_segment) = segments.iter().next() else {
                        return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                            constants_str::MACRO_DIAGNOSTICS_EXPECTED_FIRST_PATH_SEGMENT_ERROR,
                        ));
                    };
                    if first_segment.ident != constants_str::VEC {
                        return crate::macro_compile_error_tokens::macro_compile_error_tokens(
                            crate::compile_error_message::CompileErrorMessage::from(
                                constants_str::MACRO_DIAGNOSTICS_EXPECTED_VEC_TYPE_ERROR,
                            ),
                        );
                    }
                    let element_vec_type_with_serde_token_stream = if let syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments { args, .. },
                    ) = &first_segment.arguments
                    {
                        if args.len() != 1 {
                            return crate::macro_compile_error_tokens::macro_compile_error_tokens(
                                crate::compile_error_message::CompileErrorMessage::from(
                                    constants_str::MACRO_DIAGNOSTICS_EXPECTED_ANGLE_BRACKETED_ARGS_ERROR,
                                ),
                            );
                        }
                        match format!(
                            "{}{}",
                            {
                                let Some(first_arg) = args.iter().next() else {
                                    return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                                        constants_str::COMPILE_ERROR_CE_053,
                                    ));
                                };
                                quote::quote! {#first_arg}
                            },
                            with_serde_upper_camel_case,
                        )
                        .parse::<proc_macro2::TokenStream>()
                        {
                            Ok(parsed_token_stream) => parsed_token_stream,
                            Err(error) => {
                                return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                                    &constants_str::COMPILE_ERROR_CE_007
                                        .replace(
                                            constants_str::COMPILE_ERROR_ERROR_PLACEHOLDER,
                                            &error.to_string(),
                                        ),
                                ));
                            }
                        }
                    } else {
                        return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                            constants_str::MACRO_DIAGNOSTICS_EXPECTED_ANGLE_BRACKETED_ARGS_ERROR,
                        ));
                    };
                    quote::quote! {
                        Vec<#element_vec_type_with_serde_token_stream>
                    }
                }
                crate::location_field_attr::LocationFieldAttr::ErrorFieldHashMapKeyStringValueToErrString => {
                    if get_hashmap_args().is_none() {
                        return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                            constants_str::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_C1_ERROR,
                        ));
                    }
                    quote::quote! {
                        std::collections::HashMap<#string_token_stream, #string_token_stream>
                    }
                }
                crate::location_field_attr::LocationFieldAttr::ErrorFieldHashMapKeyStringValueToErrStringSerde => {
                    let Some((_, second_argument)) = get_hashmap_args() else {
                        return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                            constants_str::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_E9_ERROR,
                        ));
                    };
                    quote::quote! {
                        std::collections::HashMap<#string_token_stream, #second_argument>
                    }
                }
                crate::location_field_attr::LocationFieldAttr::ErrorFieldHashMapKeyStringValueLocation => {
                    let Some((_, second_argument)) = get_hashmap_args() else {
                        return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                            constants_str::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_C8_ERROR,
                        ));
                    };
                    let element_hashmap_v_type_with_serde_token_stream =
                        match format!("{}{}", quote::quote! {#second_argument}, with_serde_upper_camel_case)
                            .parse::<proc_macro2::TokenStream>()
                        {
                            Ok(parsed_token_stream) => parsed_token_stream,
                            Err(error) => {
                                return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                                    &constants_str::COMPILE_ERROR_CE_020
                                        .replace(
                                            constants_str::COMPILE_ERROR_ERROR_PLACEHOLDER,
                                            &error.to_string(),
                                        ),
                                ));
                            }
                        };
                    quote::quote! {
                        std::collections::HashMap<#string_token_stream, #element_hashmap_v_type_with_serde_token_stream>
                    }
                }
            };
            quote::quote! {#element_c25b655e_identifier: #element_type_with_serde_token_stream}
        };
        crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(quote::quote! {#tokens,})
    });
    crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
        quote::quote! {
            #element_identifier {
                #(#fields_with_serde_token_stream)*
            }
        },
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_malformed_collection_fields_emit_compile_errors() {
        let variants: [syn::Variant; 5] = [
            syn::parse_quote! { Example { #[error_field_hashmap_key_string_value_to_err_string] field: Vec<String> } },
            syn::parse_quote! { Example { #[error_field_hashmap_key_string_value_to_err_string] field: HashMap<String> } },
            syn::parse_quote! { Example { #[error_field_vec_location] field: std::vec::Vec<Location> } },
            syn::parse_quote! { Example { #[error_field_vec_location] field: Vec<Location, Other> } },
            syn::parse_quote! { Example { #[error_field_vec_location] field: Option<Location> } },
        ];
        assert!(variants.iter().all(|variant| {
            crate::generate_serde_version_of_named_syn_variant::generate_serde_version_of_named_syn_variant(
                crate::syn_variant_ref::SynVariantRef::from(variant),
            )
            .to_string()
            .contains(constants_str::SHARED_VALUES_COMPILE_ERROR)
        }));
    }
    #[test]
    fn test_serde_variant_generation_preserves_all_field_modes_and_location_override() {
        let variant: syn::Variant = syn::parse_quote! {
            Example {
                #[error_field_to_err_string] rendered: ErrorValue,
                #[error_field_to_err_string_serde] serialized: ErrorValue,
                #[error_field_location] nested: ErrorValue,
                #[error_field_vec_to_err_string] rendered_values: Vec<ErrorValue>,
                #[error_field_vec_to_err_string_serde] serialized_values: Vec<ErrorValue>,
                #[error_field_vec_location] nested_values: Vec<ErrorValue>,
                #[error_field_hashmap_key_string_value_to_err_string] rendered_map: std::collections::HashMap<Key, ErrorValue>,
                #[error_field_hashmap_key_string_value_to_err_string_serde] serialized_map: HashMap<Key, ErrorValue>,
                #[error_field_hashmap_key_string_value_location] nested_map: HashMap<Key, ErrorValue>,
                location: OriginalLocation,
            }
        };
        let generated = crate::generate_serde_version_of_named_syn_variant::generate_serde_version_of_named_syn_variant(crate::syn_variant_ref::SynVariantRef::from(&variant));
        let expected: syn::Variant = syn::parse_quote! {
            Example {
                rendered: String,
                serialized: ErrorValue,
                nested: ErrorValueWithSerde,
                rendered_values: Vec<String>,
                serialized_values: Vec<ErrorValue>,
                nested_values: Vec<ErrorValueWithSerde>,
                rendered_map: std::collections::HashMap<String, String>,
                serialized_map: std::collections::HashMap<String, ErrorValue>,
                nested_map: std::collections::HashMap<String, ErrorValueWithSerde>,
                location: location_lib::location::Location,
            }
        };
        let observed_result = syn::parse2::<syn::Variant>(generated.as_ref().clone());
        assert!(observed_result.is_ok());
        assert_eq!(observed_result.ok(), Some(expected));
    }

    #[test]
    fn test_serde_variant_generation_preserves_exact_collection_diagnostics() {
        [
            (quote::quote!(#[error_field_vec_location] field: &Location), constants_str::COMPILE_ERROR_CE_024),
            (quote::quote!(#[error_field_vec_location] field: std::vec::Vec<Location>), constants_str::COMPILE_ERROR_CE_024),
            (quote::quote!(#[error_field_vec_location] field: Option<Location>), constants_str::MACRO_DIAGNOSTICS_EXPECTED_VEC_TYPE_ERROR),
            (quote::quote!(#[error_field_vec_location] field: Vec), constants_str::MACRO_DIAGNOSTICS_EXPECTED_ANGLE_BRACKETED_ARGS_ERROR),
            (quote::quote!(#[error_field_vec_location] field: Vec<Location, Other>), constants_str::MACRO_DIAGNOSTICS_EXPECTED_ANGLE_BRACKETED_ARGS_ERROR),
            (quote::quote!(#[error_field_hashmap_key_string_value_to_err_string] field: &Map), constants_str::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_C1_ERROR),
            (quote::quote!(#[error_field_hashmap_key_string_value_to_err_string] field: HashMap), constants_str::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_C1_ERROR),
            (quote::quote!(#[error_field_hashmap_key_string_value_to_err_string_serde] field: Vec<Location>), constants_str::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_E9_ERROR),
            (quote::quote!(#[error_field_hashmap_key_string_value_location] field: HashMap<Key>), constants_str::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_C8_ERROR),
        ].into_iter().fold((), |(), (field, diagnostic)| {
            let variant: syn::Variant = syn::parse_quote!(Example { #field });
            let generated = crate::generate_serde_version_of_named_syn_variant::generate_serde_version_of_named_syn_variant(crate::syn_variant_ref::SynVariantRef::from(&variant));
            let message = syn::LitStr::new(diagnostic, proc_macro2::Span::call_site());
            assert_eq!(generated.to_string(), quote::quote!(Example { compile_error!(#message); }).to_string());
        });
    }

    #[test]
    fn test_serde_variant_generation_preserves_shape_and_field_attribute_diagnostics() {
        let shape_message = syn::LitStr::new(
            constants_str::MACRO_DIAGNOSTICS_EXPECTED_NAMED_VARIANT_FIELDS_ERROR,
            proc_macro2::Span::call_site(),
        );
        [syn::parse_quote!(Example), syn::parse_quote!(Example(ErrorValue))].into_iter().fold((), |(), variant| {
            let generated = crate::generate_serde_version_of_named_syn_variant::generate_serde_version_of_named_syn_variant(crate::syn_variant_ref::SynVariantRef::from(&variant));
            assert_eq!(generated.to_string(), quote::quote!(compile_error!(#shape_message);).to_string());
        });
        [
            (quote::quote!(field: ErrorValue), constants_str::OPT_ATTR_IS_NONE),
            (quote::quote!(#[error_field_location] #[error_field_vec_location] field: ErrorValue), constants_str::TWO_OR_MORE_SUPPORTED_ATTRS),
            (quote::quote!(#[error_field_location(unexpected)] field: ErrorValue), constants_str::SUPPORTED_LOCATION_FIELD_ATTR_MUST_NOT_HAVE_ARGUMENTS),
        ].into_iter().fold((), |(), (field, diagnostic)| {
            let variant: syn::Variant = syn::parse_quote!(Example { #field });
            let generated = crate::generate_serde_version_of_named_syn_variant::generate_serde_version_of_named_syn_variant(crate::syn_variant_ref::SynVariantRef::from(&variant));
            let message = syn::LitStr::new(&constants_str::COMPILE_ERROR_CE_010.replace(constants_str::COMPILE_ERROR_ERROR_PLACEHOLDER, diagnostic), proc_macro2::Span::call_site());
            assert_eq!(generated.to_string(), quote::quote!(Example { compile_error!(#message); }).to_string());
        });
    }
    #[test]
    fn test_serde_variant_generation_rejects_missing_named_field_identifier() {
        let mut variant: syn::Variant = syn::parse_quote! {
            Example { #[error_field_location] field: ErrorValue }
        };
        assert!(matches!(variant.fields, syn::Fields::Named(_)));
        let syn::Fields::Named(fields) = &mut variant.fields else {
            return;
        };
        fields.named.iter_mut().fold((), |(), field| {
            field.ident = None;
        });
        let generated = crate::generate_serde_version_of_named_syn_variant::generate_serde_version_of_named_syn_variant(crate::syn_variant_ref::SynVariantRef::from(&variant));
        let message = syn::LitStr::new(
            constants_str::MACRO_DIAGNOSTICS_EXPECTED_NAMED_FIELD_ERROR,
            proc_macro2::Span::call_site(),
        );
        assert_eq!(
            generated.to_string(),
            quote::quote!(Example { compile_error!(#message); }).to_string()
        );
    }

    #[test]
    fn test_serde_variant_generation_rejects_empty_constructed_hash_map_path() {
        let mut variant: syn::Variant = syn::parse_quote! {
            Example { #[error_field_hashmap_key_string_value_to_err_string] field: HashMap<Key, ErrorValue> }
        };
        assert!(matches!(variant.fields, syn::Fields::Named(_)));
        let syn::Fields::Named(fields) = &mut variant.fields else {
            return;
        };
        fields.named.iter_mut().fold((), |(), field| {
            assert!(matches!(field.ty, syn::Type::Path(_)));
            if let syn::Type::Path(path) = &mut field.ty {
                path.path.segments.clear();
            }
        });
        let generated = crate::generate_serde_version_of_named_syn_variant::generate_serde_version_of_named_syn_variant(crate::syn_variant_ref::SynVariantRef::from(&variant));
        let message = syn::LitStr::new(
            constants_str::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_C1_ERROR,
            proc_macro2::Span::call_site(),
        );
        assert_eq!(
            generated.to_string(),
            quote::quote!(Example { compile_error!(#message); }).to_string()
        );
    }

    #[test]
    fn test_serde_variant_generation_preserves_constructed_type_lexical_errors() {
        let malformed_tokens: proc_macro2::TokenStream = [
            proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                '/',
                proc_macro2::Spacing::Joint,
            )),
            proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                '*',
                proc_macro2::Spacing::Alone,
            )),
        ]
        .into_iter()
        .collect();
        let expected_error = format!(
            "{}{}",
            malformed_tokens,
            naming::domain_types::WithSerdeUpperCamelCase
        )
        .parse::<proc_macro2::TokenStream>()
        .err();
        assert!(expected_error.is_some());
        let Some(error) = expected_error else {
            return;
        };
        let scalar_variant: syn::Variant = syn::parse_quote!(Example {
            #[error_field_location]
            field: ErrorValue
        });
        [
            (scalar_variant, constants_str::COMPILE_ERROR_CE_005),
            (syn::parse_quote!(Example { #[error_field_vec_location] field: Vec<ErrorValue> }), constants_str::COMPILE_ERROR_CE_007),
            (syn::parse_quote!(Example { #[error_field_hashmap_key_string_value_location] field: HashMap<Key, ErrorValue> }), constants_str::COMPILE_ERROR_CE_020),
        ].into_iter().fold((), |(), (mut variant, diagnostic)| {
            assert!(matches!(variant.fields, syn::Fields::Named(_)));
            let syn::Fields::Named(fields) = &mut variant.fields else { return; };
            fields.named.iter_mut().fold((), |(), field| {
                let malformed_type = syn::Type::Verbatim(malformed_tokens.clone());
                if let syn::Type::Path(path) = &mut field.ty
                    && let Some(segment) = path.path.segments.last_mut()
                    && let syn::PathArguments::AngleBracketed(arguments) = &mut segment.arguments
                    && let Some(argument) = arguments.args.last_mut()
                {
                    *argument = syn::GenericArgument::Type(malformed_type);
                } else {
                    field.ty = malformed_type;
                }
            });
            let generated = crate::generate_serde_version_of_named_syn_variant::generate_serde_version_of_named_syn_variant(crate::syn_variant_ref::SynVariantRef::from(&variant));
            let message = syn::LitStr::new(&diagnostic.replace(constants_str::COMPILE_ERROR_ERROR_PLACEHOLDER, &error.to_string()), proc_macro2::Span::call_site());
            assert_eq!(generated.to_string(), quote::quote!(Example { compile_error!(#message); }).to_string());
        });
    }
    #[test]
    fn test_serde_variant_generation_preserves_empty_and_mixed_valid_field_outputs() {
        let empty_variant: syn::Variant = syn::parse_quote!(Example {});
        let empty_generated = crate::generate_serde_version_of_named_syn_variant::generate_serde_version_of_named_syn_variant(
            crate::syn_variant_ref::SynVariantRef::from(&empty_variant),
        );
        assert_eq!(
            empty_generated.to_string(),
            quote::quote!(Example {}).to_string()
        );
        let variant: syn::Variant = syn::parse_quote!(Example {
            #[error_field_to_err_string]
            first: ErrorValue,
            invalid: ErrorValue,
            #[error_field_location(unexpected)]
            location: OriginalLocation,
            #[error_field_to_err_string_serde]
            last: ErrorValue,
        });
        let generated = crate::generate_serde_version_of_named_syn_variant::generate_serde_version_of_named_syn_variant(
            crate::syn_variant_ref::SynVariantRef::from(&variant),
        );
        let message = syn::LitStr::new(
            &constants_str::COMPILE_ERROR_CE_010.replace(
                constants_str::COMPILE_ERROR_ERROR_PLACEHOLDER,
                constants_str::OPT_ATTR_IS_NONE,
            ),
            proc_macro2::Span::call_site(),
        );
        assert_eq!(
            generated.to_string(),
            quote::quote!(Example {
                first: String,
                compile_error!(#message);
                location: location_lib::location::Location,
                last: ErrorValue,
            })
            .to_string()
        );
    }
}
