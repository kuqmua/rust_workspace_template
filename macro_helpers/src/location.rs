#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum LocationFieldAttr {
    EoToErrString,
    EoToErrStringSerde,
    EoLocation,
    EoVecToErrString,
    EoVecToErrStringSerde,
    EoVecLocation,
    EoHashMapKStringVToErrString,
    EoHashMapKStringVToErrStringSerde,
    EoHashMapKStringVLocation,
}
impl std::str::FromStr for LocationFieldAttr {
    type Err = ();
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|item| {
                crate::domain_types::attribute_identifier_string::AttrIdentifierStr::attribute_identifier_string(
                    item,
                )
                .as_ref()
                    == v
            })
            .ok_or(())
    }
}
impl TryFrom<&syn::Field> for LocationFieldAttr {
    type Error = String;
    fn try_from(syn_field: &syn::Field) -> Result<Self, Self::Error> {
        let mut supported_attrs = syn_field.attrs.iter().filter_map(|element| {
            if element.path().segments.len() != 1 {
                return None;
            }
            let first_segment_identifier = &element.path().segments.first()?.ident;
            std::str::FromStr::from_str(&first_segment_identifier.to_string()).ok()
        });
        let optional_attr = supported_attrs.next();
        if supported_attrs.next().is_some() {
            return Err(constants_str::TWO_OR_MORE_SUPPORTED_ATTRS.to_owned());
        }
        optional_attr.map_or_else(|| Err(constants_str::OPT_ATTR_IS_NONE.to_owned()), Ok)
    }
}
impl crate::domain_types::attribute_identifier_string::AttrIdentifierStr for LocationFieldAttr {
    fn attribute_identifier_string(
        &self,
    ) -> crate::domain_types::attribute_identifier_string::AttrIdentifierName<'_> {
        crate::domain_types::attribute_identifier_string::AttrIdentifierName::from(match *self {
            Self::EoToErrString => constants_str::EO_TO_ERR_STRING,
            Self::EoToErrStringSerde => constants_str::EO_TO_ERR_STRING_SERDE,
            Self::EoLocation => constants_str::EO_LOCATION,
            Self::EoVecToErrString => constants_str::EO_VEC_TO_ERR_STRING,
            Self::EoVecToErrStringSerde => constants_str::EO_VEC_TO_ERR_STRING_SERDE,
            Self::EoVecLocation => constants_str::EO_VEC_LOCATION,
            Self::EoHashMapKStringVToErrString => {
                constants_str::EO_HASHMAP_K_STRING_V_TO_ERR_STRING
            }
            Self::EoHashMapKStringVToErrStringSerde => {
                constants_str::EO_HASHMAP_K_STRING_V_TO_ERR_STRING_SERDE
            }
            Self::EoHashMapKStringVLocation => constants_str::EO_HASHMAP_K_STRING_V_LOCATION,
        })
    }
}
impl LocationFieldAttr {
    const ALL: [Self; 9] = [
        Self::EoToErrString,
        Self::EoToErrStringSerde,
        Self::EoLocation,
        Self::EoVecToErrString,
        Self::EoVecToErrStringSerde,
        Self::EoVecLocation,
        Self::EoHashMapKStringVToErrString,
        Self::EoHashMapKStringVToErrStringSerde,
        Self::EoHashMapKStringVLocation,
    ];
    #[must_use]
    pub fn to_attr_view_token_stream(
        &self,
    ) -> crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
        match format!(
            "#[{}]",
            crate::domain_types::attribute_identifier_string::AttrIdentifierStr::attribute_identifier_string(self)
                .as_ref()
        )
        .parse::<proc_macro2::TokenStream>()
        {
            Ok(v) => {
                crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(v)
            }
            Err(error) => compile_error_token_stream(CompileErrorMessage::from(&error.to_string())),
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
struct CompileErrorMessage<'message_lt>(&'message_lt str);
impl<'message_lt> From<&'message_lt String> for CompileErrorMessage<'message_lt> {
    fn from(value: &'message_lt String) -> Self {
        Self(value.as_str())
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct SynVariantRef<'variant_lt>(&'variant_lt syn::Variant);
fn compile_error_token_stream(
    message: CompileErrorMessage<'_>,
) -> crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
{
    let message_value = message.0;
    crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
        quote::quote! {compile_error!(#message_value);},
    )
}
#[must_use]
pub fn generate_serde_version_of_named_syn_variant(
    v: SynVariantRef<'_>,
) -> crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
{
    let variant = v.0;
    let hash_map_upper_camel_case = naming::domain_types::HashMapUpperCamelCase;
    let location_snake_case = naming::domain_types::LocationSnakeCase;
    let string_token_stream = token_patterns::StringTokenStream;
    let with_serde_upper_camel_case = naming::domain_types::WithSerdeUpperCamelCase;
    let element_identifier = &variant.ident;
    let fields = if let syn::Fields::Named(fields) = &variant.fields {
        &fields.named
    } else {
        return compile_error_token_stream(CompileErrorMessage::from(
            constants_str::MACRO_DIAGNOSTICS_EXPECTED_NAMED_VARIANT_FIELDS_ERROR,
        ));
    };
    let fields_with_serde_token_stream = fields.iter().map(|element| {
        let Some(element_c25b655e_identifier) = element.ident.as_ref() else {
            return compile_error_token_stream(CompileErrorMessage::from(
                constants_str::MACRO_DIAGNOSTICS_EXPECTED_NAMED_FIELD_ERROR,
            ));
        };
        let ts = if *element_c25b655e_identifier == *location_snake_case.to_string() {
            quote::quote! {#location_snake_case: location_lib::domain_types::Location}
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
            let location_field_attr = match LocationFieldAttr::try_from(element) {
                Ok(parsed_attr) => parsed_attr,
                Err(error) => return compile_error_token_stream(CompileErrorMessage::from(
                    &constants_str::COMPILE_ERROR_CE_010.replace(
                        constants_str::COMPILE_ERROR_ERROR_PLACEHOLDER,
                        &error,
                    ),
                )),
            };
            let element_type_with_serde_token_stream = match location_field_attr {
                LocationFieldAttr::EoToErrString => quote::quote! {#string_token_stream},
                LocationFieldAttr::EoToErrStringSerde | LocationFieldAttr::EoVecToErrStringSerde => {
                    element_type_token_stream
                }
                LocationFieldAttr::EoLocation => match format!("{element_type_token_stream}{with_serde_upper_camel_case}")
                    .parse::<proc_macro2::TokenStream>()
                {
                    Ok(parsed_token_stream) => parsed_token_stream,
                    Err(error) => {
                        return compile_error_token_stream(CompileErrorMessage::from(
                            &constants_str::COMPILE_ERROR_CE_005
                                .replace(
                                    constants_str::COMPILE_ERROR_ERROR_PLACEHOLDER,
                                    &error.to_string(),
                                ),
                        ));
                    }
                },
                LocationFieldAttr::EoVecToErrString => {
                    quote::quote! {
                        Vec<#string_token_stream>
                    }
                }
                LocationFieldAttr::EoVecLocation => {
                    let segments = if let syn::Type::Path(v0) = &element.ty {
                        &v0.path.segments
                    } else {
                        return compile_error_token_stream(CompileErrorMessage::from(constants_str::COMPILE_ERROR_CE_024));
                    };
                    assert!(segments.len() == 1, "0c65bbaa");
                    let Some(first_segment) = segments.iter().next() else {
                        return compile_error_token_stream(CompileErrorMessage::from(
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
                                    return compile_error_token_stream(CompileErrorMessage(
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
                                return compile_error_token_stream(CompileErrorMessage::from(
                                    &constants_str::COMPILE_ERROR_CE_007
                                        .replace(
                                            constants_str::COMPILE_ERROR_ERROR_PLACEHOLDER,
                                            &error.to_string(),
                                        ),
                                ));
                            }
                        }
                    } else {
                        return compile_error_token_stream(CompileErrorMessage::from(
                            constants_str::MACRO_DIAGNOSTICS_EXPECTED_ANGLE_BRACKETED_ARGS_ERROR,
                        ));
                    };
                    quote::quote! {
                        Vec<#element_vec_type_with_serde_token_stream>
                    }
                }
                LocationFieldAttr::EoHashMapKStringVToErrString => {
                    if get_hashmap_args().is_none() {
                        return compile_error_token_stream(CompileErrorMessage::from(
                            constants_str::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_C1_ERROR,
                        ));
                    }
                    quote::quote! {
                        std::collections::HashMap<#string_token_stream, #string_token_stream>
                    }
                }
                LocationFieldAttr::EoHashMapKStringVToErrStringSerde => {
                    let Some((_, second_argument)) = get_hashmap_args() else {
                        return compile_error_token_stream(CompileErrorMessage::from(
                            constants_str::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_E9_ERROR,
                        ));
                    };
                    quote::quote! {
                        std::collections::HashMap<#string_token_stream, #second_argument>
                    }
                }
                LocationFieldAttr::EoHashMapKStringVLocation => {
                    let Some((_, second_argument)) = get_hashmap_args() else {
                        return compile_error_token_stream(CompileErrorMessage::from(
                            constants_str::MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_C8_ERROR,
                        ));
                    };
                    let element_hashmap_v_type_with_serde_token_stream =
                        match format!("{}{}", quote::quote! {#second_argument}, with_serde_upper_camel_case)
                            .parse::<proc_macro2::TokenStream>()
                        {
                            Ok(parsed_token_stream) => parsed_token_stream,
                            Err(error) => {
                                return compile_error_token_stream(CompileErrorMessage::from(
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
        crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(quote::quote! {#ts,})
    });
    crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
        quote::quote! {
            #element_identifier {
                #(#fields_with_serde_token_stream)*
            }
        },
    )
}
