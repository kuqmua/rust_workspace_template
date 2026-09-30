#![allow(
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::useless_conversion,
    reason = "shared proc-macro implementations preserve original owned entrypoint signatures and conversion points while returning proc_macro2 streams to one-entrypoint facade crates; every result is consumed immediately by its facade"
)]

pub(crate) mod proc_macro2_generated_naming_token_stream;
pub(crate) mod proc_macro2_variant_matching_tokens_ref;
pub(crate) mod syn_enum_identifier_ref;
pub(crate) mod syn_naming_generics_ref;
#[cfg(test)]
mod test_naming_case_overflow;
#[cfg(test)]
mod test_naming_enum_generics;
#[cfg(test)]
mod test_self_placeholder_cardinality;

#[derive(Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
enum EnumCase {
    Snake,
    UpperCamel,
    UpperSnake,
}

fn generate_impl_to_tokens_token_stream(
    type_token_stream: &dyn quote::ToTokens,
    method_body_token_stream: &dyn quote::ToTokens,
) -> proc_macro2_generated_naming_token_stream::ProcMacro2GeneratedNamingTokenStream {
    proc_macro2_generated_naming_token_stream::ProcMacro2GeneratedNamingTokenStream::from(
        quote::quote! {
            impl quote::ToTokens for #type_token_stream {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    #method_body_token_stream
                }
            }
        },
    )
}
pub fn generate_upper_camel_case_and_snake_case_str_and_token_stream(
    token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    panic_location::panic_location();
    let regex = regex::Regex::new(constants_str::NAMING_REGEX_VALUE)
        .expect(constants_str::DIAGNOSTIC_20948D87);
    let generated_tokens = serde_json::from_str::<Vec<Vec<String>>>(&token_stream.to_string())
        .expect(constants_str::DIAGNOSTIC_90E5793B)
        .into_iter()
        .map(|element| {
            assert!(element.iter().all(|element_name| regex.is_match(element_name)), "faadba8a");
            let parts_len = element.iter().map(String::len).sum::<usize>();
            let phrase_part_upper_camel_case_result: Result<
                String,
                naming_common::case_string::CaseStringTryFromStringError,
            > = element.iter().try_fold(
                String::with_capacity(parts_len),
                |mut accumulator, element_name| {
                    accumulator.push_str(&naming_common::domain_types::AsRefStrToUpperCamelCaseStr::try_case(element_name)?);
                    Ok(accumulator)
                },
            );
            let phrase_part_upper_camel_case_str = match phrase_part_upper_camel_case_result {
                Ok(case_string) => case_string,
                Err(error) => {
                    let error_message = error.to_string();
                    return quote::quote! {compile_error!(#error_message);};
                }
            };
            let phrase_part_snake_case_result: Result<
                String,
                naming_common::case_string::CaseStringTryFromStringError,
            > = element.iter().enumerate().try_fold(
                String::with_capacity(parts_len.saturating_add(element.len().saturating_sub(constants_usize::ONE))),
                |mut accumulator, (i, element_name)| {
                        let element_snake_case_str = naming_common::domain_types::AsRefStrToSnakeCaseStr::try_case(element_name)?;
                        if i == 0 {
                            accumulator.push_str(&element_snake_case_str);
                        } else {
                            assert!(
                                std::fmt::Write::write_fmt(&mut accumulator, format_args!("_{element_snake_case_str}"))
                                    .is_ok(),
                                "ef718915"
                            );
                        }
                        Ok(accumulator)
                },
            );
            let phrase_part_snake_case_str = match phrase_part_snake_case_result {
                Ok(case_string) => case_string,
                Err(error) => {
                    let error_message = error.to_string();
                    return quote::quote! {compile_error!(#error_message);};
                }
            };
            let phrase_part_upper_camel_case_upper_camel_case_token_stream = format!("{phrase_part_upper_camel_case_str}UpperCamelCase")
                .parse::<proc_macro2::TokenStream>()
                .expect(constants_str::DIAGNOSTIC_4AB6A54C);
            let phrase_part_snake_case_upper_camel_case_token_stream = format!("{phrase_part_upper_camel_case_str}SnakeCase")
                .parse::<proc_macro2::TokenStream>()
                .expect(constants_str::DIAGNOSTIC_0CC47B2E);
            let (upper_camel_case_struct_declaration_token_stream, snake_case_struct_declaration_token_stream) = {
                let generate_token_stream = |generated_tokens: &dyn quote::ToTokens| {
                    quote::quote! {
                        #[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
                        pub struct #generated_tokens;
                    }
                };
                (
                    generate_token_stream(&phrase_part_upper_camel_case_upper_camel_case_token_stream),
                    generate_token_stream(&phrase_part_snake_case_upper_camel_case_token_stream),
                )
            };
            let (impl_display_upper_camel_case_token_stream, impl_display_snake_case_token_stream) = {
                let generate_token_stream = |struct_name_token_stream: &dyn quote::ToTokens,
                              write_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        impl std::fmt::Display for #struct_name_token_stream {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                write!(f, #write_token_stream)
                            }
                        }
                    }
                };
                (
                    generate_token_stream(
                        &phrase_part_upper_camel_case_upper_camel_case_token_stream,
                        &generate_quotes::double_quoted_token_stream::double_quoted_token_stream(&phrase_part_upper_camel_case_str),
                    ),
                    generate_token_stream(
                        &phrase_part_snake_case_upper_camel_case_token_stream,
                        &generate_quotes::double_quoted_token_stream::double_quoted_token_stream(&phrase_part_snake_case_str),
                    ),
                )
            };
            let (impl_to_tokens_upper_camel_case_token_stream, impl_to_tokens_snake_token_stream) = {
                let generate_token_stream = |struct_name_token_stream: &dyn quote::ToTokens,
                              quote_token_stream: &dyn quote::ToTokens| {
                    generate_impl_to_tokens_token_stream(
                        struct_name_token_stream,
                        &quote::quote! {quote::ToTokens::to_tokens(&quote::quote! {#quote_token_stream}, tokens);},
                    )
                };
                (
                    generate_token_stream(
                        &phrase_part_upper_camel_case_upper_camel_case_token_stream,
                        &phrase_part_upper_camel_case_str
                            .parse::<proc_macro2::TokenStream>()
                            .expect(constants_str::DIAGNOSTIC_7CF3FFC0),
                    ),
                    generate_token_stream(
                        &phrase_part_snake_case_upper_camel_case_token_stream,
                        &phrase_part_snake_case_str
                            .parse::<proc_macro2::TokenStream>()
                            .expect(constants_str::DIAGNOSTIC_114A573A),
                    ),
                )
            };
            quote::quote! {
                #upper_camel_case_struct_declaration_token_stream
                #impl_display_upper_camel_case_token_stream
                #impl_to_tokens_upper_camel_case_token_stream
                #snake_case_struct_declaration_token_stream
                #impl_display_snake_case_token_stream
                #impl_to_tokens_snake_token_stream
            }
        });
    let generated = quote::quote! {#(#generated_tokens)*};
    generated.into()
}
pub fn generate_self_upper_camel_case_and_snake_case_str_and_token_stream(
    token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    panic_location::panic_location();
    let regex = regex::Regex::new(constants_str::NAMING_REGEX_VALUE)
        .expect(constants_str::DIAGNOSTIC_CBA1B5FB);
    let generated_tokens = serde_json::from_str::<Vec<Vec<String>>>(&token_stream.to_string()).expect(constants_str::DIAGNOSTIC_9D6A20AF).into_iter().map(|element| {
        assert!(element.iter().all(|element_name| regex.is_match(element_name)), "4a12d90f");
        let self_match_name = constants_str::SELF_ALT;
        {
            let has_one_self = element.iter().filter(|string| string.as_str() == self_match_name).take(constants_usize::TWO).count() == constants_usize::ONE;
            assert!(has_one_self, "{}", constants_str::DIAGNOSTIC_5680DD63);
        };
        let (elements_concat_v_upper_camel_case_double_quoted_token_stream, elements_concat_v_snake_case_double_quoted_token_stream, struct_upper_camel_case_upper_camel_case_token_stream, struct_snake_case_token_upper_camel_case_token_stream, trait_upper_camel_case_upper_camel_case_token_stream, trait_snake_case_token_upper_camel_case_token_stream) = {
            let upper_camel_case_suffix = constants_str::UPPERCAMELCASE;
            let snake_case_suffix = constants_str::SNAKECASE;
            let parts_len = element.iter().map(String::len).sum::<usize>();
            let compile_case_error = |error: naming_common::case_string::CaseStringTryFromStringError| {
                let error_message = error.to_string();
                quote::quote! {compile_error!(#error_message);}
            };
            let convert_upper_camel = |replace_self: bool| {
                element.iter().try_fold(String::with_capacity(parts_len), |mut accumulator, element_name| {
                    if replace_self && element_name == self_match_name {
                        accumulator.push_str(constants_str::V_ALT);
                    } else {
                        accumulator.push_str(&naming_common::domain_types::AsRefStrToUpperCamelCaseStr::try_case(element_name)?);
                    }
                    Ok::<String, naming_common::case_string::CaseStringTryFromStringError>(accumulator)
                })
            };
            let elements_concat_upper_camel_case_str = match convert_upper_camel(false) {
                Ok(case_string) => case_string,
                Err(error) => return compile_case_error(error),
            };
            let elements_concat_v_upper_camel_case_str = match convert_upper_camel(true) {
                Ok(case_string) => case_string,
                Err(error) => return compile_case_error(error),
            };
            let elements_concat_v_upper_camel_case_double_quoted_token_stream = generate_quotes::double_quoted_token_stream::double_quoted_token_stream(&elements_concat_v_upper_camel_case_str);
            let snake_case_result = element.iter().try_fold(String::with_capacity(parts_len.saturating_add(element.len())), |mut accumulator, element_name| {
                let symbol = '_';
                if element_name == self_match_name {
                    assert!(std::fmt::Write::write_fmt(&mut accumulator, format_args!("{{v}}{symbol}")).is_ok(), "6a02a2ff");
                } else {
                    let converted = naming_common::domain_types::AsRefStrToSnakeCaseStr::try_case(element_name)?;
                    assert!(std::fmt::Write::write_fmt(&mut accumulator, format_args!("{converted}{symbol}")).is_ok(), "d915980a");
                }
                Ok::<String, naming_common::case_string::CaseStringTryFromStringError>(accumulator)
            });
            let mut elements_concat_v_snake_case_str = match snake_case_result {
                Ok(case_string) => case_string,
                Err(error) => return compile_case_error(error),
            };
            let _: Option<char> = elements_concat_v_snake_case_str.pop();
            let elements_concat_v_snake_case_double_quoted_token_stream = generate_quotes::double_quoted_token_stream::double_quoted_token_stream(&elements_concat_v_snake_case_str);
            let struct_upper_camel_case_upper_camel_case_token_stream = format!("{elements_concat_upper_camel_case_str}{upper_camel_case_suffix}").parse::<proc_macro2::TokenStream>().expect(constants_str::DIAGNOSTIC_82F4AC08);
            let struct_snake_case_token_upper_camel_case_token_stream = format!("{elements_concat_upper_camel_case_str}{snake_case_suffix}").parse::<proc_macro2::TokenStream>().expect(constants_str::DIAGNOSTIC_21044EBA);
            let (trait_upper_camel_case_upper_camel_case_token_stream, trait_snake_case_token_upper_camel_case_token_stream) = {
                let trait_upper_camel_case_str = constants_str::TRAIT;
                let trait_upper_camel_case_upper_camel_case_token_stream = format!("{elements_concat_upper_camel_case_str}{upper_camel_case_suffix}{trait_upper_camel_case_str}").parse::<proc_macro2::TokenStream>().expect(constants_str::DIAGNOSTIC_1066857A);
                let trait_snake_case_token_upper_camel_case_token_stream = format!("{elements_concat_upper_camel_case_str}{snake_case_suffix}{trait_upper_camel_case_str}").parse::<proc_macro2::TokenStream>().expect(constants_str::DIAGNOSTIC_8DB74CFD);
                (trait_upper_camel_case_upper_camel_case_token_stream, trait_snake_case_token_upper_camel_case_token_stream)
            };
            (
                elements_concat_v_upper_camel_case_double_quoted_token_stream,
                elements_concat_v_snake_case_double_quoted_token_stream,
                struct_upper_camel_case_upper_camel_case_token_stream,
                struct_snake_case_token_upper_camel_case_token_stream,
                trait_upper_camel_case_upper_camel_case_token_stream,
                trait_snake_case_token_upper_camel_case_token_stream,
            )
        };
        let generate_struct_token_stream = |elements_concat_v_case_double_quoted_token_stream: &dyn quote::ToTokens, is_upper_camel_case: bool, trait_identifier_token_stream: &dyn quote::ToTokens| {
            let struct_identifier_token_stream = if is_upper_camel_case {
                quote::quote! {#struct_upper_camel_case_upper_camel_case_token_stream}
            } else {
                quote::quote! {#struct_snake_case_token_upper_camel_case_token_stream}
            };
            let casing_token_stream = {
                let generated_tokens = if is_upper_camel_case {
                    quote::quote! {AsRefStrToUpperCamelCaseStr::case}
                } else {
                    quote::quote! {AsRefStrToSnakeCaseStr::case}
                };
                quote::quote! {naming_common::domain_types::#generated_tokens}
            };
            let impl_to_tokens_token_stream = generate_impl_to_tokens_token_stream(
                &struct_identifier_token_stream,
                &quote::quote! {quote::ToTokens::to_tokens(&self.to_string().parse::<proc_macro2::TokenStream>().expect("71c8d26b generate_self_upper_camel_case_and_snake_case_str_and_token_stream invariant must hold"), tokens);}
            );
            quote::quote! {
                #[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype_display::Display)]
                pub struct #struct_identifier_token_stream(String);
                impl #struct_identifier_token_stream {
                    fn wrap(v: &dyn std::fmt::Display) -> Self {
                        Self(Self::format(v))
                    }
                    fn format(v: &dyn std::fmt::Display) -> String {
                        format!(#elements_concat_v_case_double_quoted_token_stream)
                    }
                    pub fn from_display(v: &dyn std::fmt::Display) -> Self {
                        Self::wrap(&#casing_token_stream(&v.to_string()))
                    }
                    pub fn from_tokens(v: &dyn quote::ToTokens) -> Self {
                        Self::wrap(&#casing_token_stream(&{
                            let mut tokens = proc_macro2::TokenStream::new();
                            quote::ToTokens::to_tokens(&v, &mut tokens);
                            tokens
                        }.to_string()))
                    }
                    pub fn from_type_last_segment(v: &syn::Type) -> Self {
                        if let syn::Type::Path(type_path) = v {
                            let path_before_len = type_path.path.segments.len().checked_sub(1).expect("e1f5a332 from_type_last_segment invariant must hold");
                            let path_before_capacity = path_before_len.saturating_mul(16usize);
                            let path_before_str = type_path.path.segments.iter().take(path_before_len)
                            .fold(String::with_capacity(path_before_capacity), |mut accumulator, element| {
                                assert!(
                                    std::fmt::Write::write_fmt(
                                        &mut accumulator,
                                        format_args!("{}::", element.ident),
                                    ).is_ok(),
                                    "67c90ce9"
                                );
                                accumulator
                            });
                            let last = type_path.path.segments.iter().last().expect("19f6e1a6 from_type_last_segment invariant must hold");
                            Self(format!("{path_before_str}{}", Self::format(&#casing_token_stream(&last.ident.to_string()))))
                        }
                        else {
                            panic!("518933f8");
                        }
                    }
                }
                #impl_to_tokens_token_stream
                pub trait #trait_identifier_token_stream: std::fmt::Display + quote::ToTokens {}
                impl #trait_identifier_token_stream for #struct_identifier_token_stream {}
            }
        };
        let pub_struct_upper_camel_case_token_stream = generate_struct_token_stream(&elements_concat_v_upper_camel_case_double_quoted_token_stream, true, &trait_upper_camel_case_upper_camel_case_token_stream);
        let pub_struct_snake_case_token_stream = generate_struct_token_stream(&elements_concat_v_snake_case_double_quoted_token_stream, false, &trait_snake_case_token_upper_camel_case_token_stream);
        quote::quote! {
            #pub_struct_upper_camel_case_token_stream
            #pub_struct_snake_case_token_stream
        }
    });
    let generated = quote::quote! {#(#generated_tokens)*};
    generated.into()
}
fn generate_impl_trait_for_identifier_token_stream(
    name_token_stream: &dyn quote::ToTokens,
    syn_enum_identifier_ref: syn_enum_identifier_ref::SynEnumIdentifierRef<'_>,
    syn_naming_generics_ref: syn_naming_generics_ref::SynNamingGenericsRef<'_>,
    proc_macro2_variant_matching_tokens_ref: proc_macro2_variant_matching_tokens_ref::ProcMacro2VariantMatchingTokensRef<'_>,
) -> proc_macro2_generated_naming_token_stream::ProcMacro2GeneratedNamingTokenStream {
    let identifier_ref = syn_enum_identifier_ref.as_ref();
    let (impl_generics, type_generics, where_clause) =
        syn_naming_generics_ref.as_ref().split_for_impl();
    let variant_tokens = proc_macro2_variant_matching_tokens_ref.as_ref();
    let string_token_stream = token_patterns::StringTokenStream;
    proc_macro2_generated_naming_token_stream::ProcMacro2GeneratedNamingTokenStream::from(
        quote::quote! {
            impl #impl_generics naming_common::domain_types::#name_token_stream for #identifier_ref #type_generics #where_clause {
                fn case(&self) -> #string_token_stream {
                    match self {#(#variant_tokens),*}
                }
            }
        },
    )
}
pub fn as_ref_str_enum_with_unit_fields_to_upper_camel_case_str(
    token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    panic_location::panic_location();
    let derive_input: syn::DeriveInput =
        syn::parse2(token_stream).expect(constants_str::DIAGNOSTIC_A8F22481);
    let identifier = &derive_input.ident;
    let syn::Data::Enum(data_enum) = derive_input.data else {
        std::panic::panic_any(constants_str::PANIC_D26BF85E)
    };
    let variant_tokens_result = data_enum
        .variants
        .iter()
        .map(|element| match element.fields {
            syn::Fields::Unit => {
                let element_identifier = &element.ident;
                let value = naming_common::domain_types::ToTokensToUpperCamelCaseStr::try_case(
                    element_identifier,
                )
                .map_err(|error| syn::Error::new_spanned(element_identifier, error))?;
                let quoted =
                    generate_quotes::double_quoted_token_stream::double_quoted_token_stream(&value);
                Ok(quote::quote! {Self::#element_identifier => String::from(#quoted)})
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                std::panic::panic_any(constants_str::PANIC_4955C50D)
            }
        })
        .collect::<syn::Result<Vec<proc_macro2::TokenStream>>>();
    let variant_tokens = match variant_tokens_result {
        Ok(variant_tokens) => variant_tokens,
        Err(error) => return error.into_compile_error(),
    };
    let generated = generate_impl_trait_for_identifier_token_stream(
        &quote::quote! {AsRefStrToUpperCamelCaseStr},
        syn_enum_identifier_ref::SynEnumIdentifierRef::from(identifier),
        syn_naming_generics_ref::SynNamingGenericsRef::from(&derive_input.generics),
        proc_macro2_variant_matching_tokens_ref::ProcMacro2VariantMatchingTokensRef::from(
            variant_tokens.as_slice(),
        ),
    );
    proc_macro2::TokenStream::from(generated).into()
}
pub fn as_ref_str_enum_with_unit_fields_to_snake_case_str(
    token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    panic_location::panic_location();
    let derive_input: syn::DeriveInput =
        syn::parse2(token_stream).expect(constants_str::DIAGNOSTIC_DEA5CBCF);
    let identifier = &derive_input.ident;
    let syn::Data::Enum(data_enum) = derive_input.data else {
        std::panic::panic_any(constants_str::PANIC_ED6EFE2E);
    };
    let variant_tokens_result = data_enum
        .variants
        .iter()
        .map(|element| match element.fields {
            syn::Fields::Unit => {
                let element_identifier = &element.ident;
                let value = naming_common::domain_types::ToTokensToSnakeCaseStr::try_case(
                    element_identifier,
                )
                .map_err(|error| syn::Error::new_spanned(element_identifier, error))?;
                let quoted =
                    generate_quotes::double_quoted_token_stream::double_quoted_token_stream(&value);
                Ok(quote::quote! {Self::#element_identifier => String::from(#quoted)})
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                std::panic::panic_any(constants_str::PANIC_B3EF2657)
            }
        })
        .collect::<syn::Result<Vec<proc_macro2::TokenStream>>>();
    let variant_tokens = match variant_tokens_result {
        Ok(variant_tokens) => variant_tokens,
        Err(error) => return error.into_compile_error(),
    };
    let generated = generate_impl_trait_for_identifier_token_stream(
        &quote::quote! {AsRefStrToSnakeCaseStr},
        syn_enum_identifier_ref::SynEnumIdentifierRef::from(identifier),
        syn_naming_generics_ref::SynNamingGenericsRef::from(&derive_input.generics),
        proc_macro2_variant_matching_tokens_ref::ProcMacro2VariantMatchingTokensRef::from(
            variant_tokens.as_slice(),
        ),
    );
    proc_macro2::TokenStream::from(generated).into()
}
pub fn as_ref_str_enum_with_unit_fields_to_upper_snake_case_str(
    token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    panic_location::panic_location();
    let derive_input: syn::DeriveInput =
        syn::parse2(token_stream).expect(constants_str::DIAGNOSTIC_EDABBC24);
    let identifier = &derive_input.ident;
    let syn::Data::Enum(data_enum) = derive_input.data else {
        std::panic::panic_any(constants_str::PANIC_B2263E7E);
    };
    let variant_tokens_result = data_enum
        .variants
        .iter()
        .map(|element| match element.fields {
            syn::Fields::Unit => {
                let element_identifier = &element.ident;
                let value = naming_common::domain_types::ToTokensToUpperSnakeCaseStr::try_case(
                    element_identifier,
                )
                .map_err(|error| syn::Error::new_spanned(element_identifier, error))?;
                let quoted =
                    generate_quotes::double_quoted_token_stream::double_quoted_token_stream(&value);
                Ok(quote::quote! {Self::#element_identifier => String::from(#quoted)})
            }
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                std::panic::panic_any(constants_str::PANIC_B6FEDCFF)
            }
        })
        .collect::<syn::Result<Vec<proc_macro2::TokenStream>>>();
    let variant_tokens = match variant_tokens_result {
        Ok(variant_tokens) => variant_tokens,
        Err(error) => return error.into_compile_error(),
    };
    let generated = generate_impl_trait_for_identifier_token_stream(
        &quote::quote! {AsRefStrToUpperSnakeCaseStr},
        syn_enum_identifier_ref::SynEnumIdentifierRef::from(identifier),
        syn_naming_generics_ref::SynNamingGenericsRef::from(&derive_input.generics),
        proc_macro2_variant_matching_tokens_ref::ProcMacro2VariantMatchingTokensRef::from(
            variant_tokens.as_slice(),
        ),
    );
    proc_macro2::TokenStream::from(generated).into()
}

fn enum_with_unit_fields_to_case_str(
    token_stream: proc_macro2::TokenStream,
    enum_case: EnumCase,
) -> proc_macro2::TokenStream {
    let derive_input: syn::DeriveInput =
        syn::parse2(token_stream).expect(constants_str::DIAGNOSTIC_C0E870DF);
    let identifier = &derive_input.ident;
    let (impl_generics, type_generics, where_clause) = derive_input.generics.split_for_impl();
    let syn::Data::Enum(data_enum) = derive_input.data else {
        std::panic::panic_any(constants_str::PANIC_3FDBA6EF)
    };
    let method_identifier = syn::Ident::new(
        match enum_case {
            EnumCase::Snake => stringify!(as_snake_case_str),
            EnumCase::UpperCamel => stringify!(as_upper_camel_case_str),
            EnumCase::UpperSnake => stringify!(as_upper_snake_case_str),
        },
        proc_macro2::Span::call_site(),
    );
    let variant_tokens_result = data_enum
        .variants
        .iter()
        .map(|variant| {
            assert!(matches!(variant.fields, syn::Fields::Unit), "4a95589f");
            let variant_identifier = &variant.ident;
            let value = match enum_case {
                EnumCase::Snake => naming_common::domain_types::ToTokensToSnakeCaseStr::try_case(
                    variant_identifier,
                ),
                EnumCase::UpperCamel => {
                    naming_common::domain_types::ToTokensToUpperCamelCaseStr::try_case(
                        variant_identifier,
                    )
                }
                EnumCase::UpperSnake => {
                    naming_common::domain_types::ToTokensToUpperSnakeCaseStr::try_case(
                        variant_identifier,
                    )
                }
            }
            .map_err(|error| syn::Error::new_spanned(variant_identifier, error))?;
            let value_token_stream =
                generate_quotes::double_quoted_token_stream::double_quoted_token_stream(&value);
            let cfg_attributes = variant
                .attrs
                .iter()
                .filter(|attribute| attribute.path().is_ident(stringify!(cfg)));
            Ok(quote::quote! {
                #(#cfg_attributes)*
                Self::#variant_identifier => #value_token_stream
            })
        })
        .collect::<syn::Result<Vec<_>>>();
    let variant_tokens = match variant_tokens_result {
        Ok(variant_tokens) => variant_tokens,
        Err(error) => return error.into_compile_error(),
    };
    quote::quote! {
        impl #impl_generics #identifier #type_generics #where_clause {
            pub(crate) const fn #method_identifier(&self) -> &'static str {
                match self {
                    #(#variant_tokens),*
                }
            }
        }
    }
    .into()
}

pub fn enum_with_unit_fields_to_snake_case_str(
    token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    enum_with_unit_fields_to_case_str(token_stream, EnumCase::Snake)
}

pub fn enum_with_unit_fields_to_upper_camel_case_str(
    token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    enum_with_unit_fields_to_case_str(token_stream, EnumCase::UpperCamel)
}

pub fn enum_with_unit_fields_to_upper_snake_case_str(
    token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    enum_with_unit_fields_to_case_str(token_stream, EnumCase::UpperSnake)
}
