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
        let ts = if *element_c25b655e_identifier == *location_snake_case.to_string() {
            quote::quote! {#location_snake_case: location_lib::location::Location}
        } else {
            let get_hashmap_args = || {
                let segments = if let syn::Type::Path(syn_type_path) = &element.ty {
                    &syn_type_path.path.segments
                } else {
                    return None;
                };
                let last_segment = segments.iter().next_back()?;
                assert!(last_segment.ident == hash_map_upper_camel_case.to_string(), "5e1bc6b1");
                let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    args,
                    ..
                }) = &last_segment.arguments
                else {
                    return None;
                };
                assert!(args.len() == 2, "47cde1b8");
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
                crate::location_field_attr::LocationFieldAttr::EoToErrString => quote::quote! {#string_token_stream},
                crate::location_field_attr::LocationFieldAttr::EoToErrStringSerde | crate::location_field_attr::LocationFieldAttr::EoVecToErrStringSerde => {
                    element_type_token_stream
                }
                crate::location_field_attr::LocationFieldAttr::EoLocation => match format!("{element_type_token_stream}{with_serde_upper_camel_case}")
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
                crate::location_field_attr::LocationFieldAttr::EoVecToErrString => {
                    quote::quote! {
                        Vec<#string_token_stream>
                    }
                }
                crate::location_field_attr::LocationFieldAttr::EoVecLocation => {
                    let segments = if let syn::Type::Path(v0) = &element.ty {
                        &v0.path.segments
                    } else {
                        return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(constants_str::COMPILE_ERROR_CE_024));
                    };
                    assert!(segments.len() == 1, "0c65bbaa");
                    let Some(first_segment) = segments.iter().next() else {
                        return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                            constants_str::MACRO_DIAGNOSTICS_EXPECTED_FIRST_PATH_SEGMENT_ERROR,
                        ));
                    };
                    let element_vec_type_with_serde_token_stream = if let syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments { args, .. },
                    ) = &first_segment.arguments
                    {
                        assert!(args.len() == 1, "572a9da8");
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
                crate::location_field_attr::LocationFieldAttr::EoHashMapKStringVToErrString => {
                    if get_hashmap_args().is_none() {
                        return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                            constants_str::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_C1_ERROR,
                        ));
                    }
                    quote::quote! {
                        std::collections::HashMap<#string_token_stream, #string_token_stream>
                    }
                }
                crate::location_field_attr::LocationFieldAttr::EoHashMapKStringVToErrStringSerde => {
                    let Some((_, second_argument)) = get_hashmap_args() else {
                        return crate::macro_compile_error_tokens::macro_compile_error_tokens(crate::compile_error_message::CompileErrorMessage::from(
                            constants_str::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_E9_ERROR,
                        ));
                    };
                    quote::quote! {
                        std::collections::HashMap<#string_token_stream, #second_argument>
                    }
                }
                crate::location_field_attr::LocationFieldAttr::EoHashMapKStringVLocation => {
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
        crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(quote::quote! {#ts,})
    });
    crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
        quote::quote! {
            #element_identifier {
                #(#fields_with_serde_token_stream)*
            }
        },
    )
}
