pub(crate) mod proc_macro2_generated_naming_token_stream;
pub(crate) mod proc_macro2_variant_matching_tokens_ref;
pub(crate) mod syn_enum_identifier_ref;

#[derive(Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
enum EnumCase {
    Snake,
    UpperCamel,
    UpperSnake,
}

fn generate_impl_to_tokens_token_stream(
    ts0: &dyn quote::ToTokens,
    ts1: &dyn quote::ToTokens,
) -> proc_macro2_generated_naming_token_stream::ProcMacro2GeneratedNamingTokenStream {
    proc_macro2_generated_naming_token_stream::ProcMacro2GeneratedNamingTokenStream::from(
        quote::quote! {
            impl quote::ToTokens for #ts0 {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    #ts1
                }
            }
        },
    )
}
#[proc_macro]
pub fn generate_upper_camel_case_and_snake_case_str_and_token_stream(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let regex = regex::Regex::new(constants_str::NAMING_REGEX_VALUE)
        .expect(constants_str::DIAGNOSTIC_20948D87);
    let ts = serde_json::from_str::<Vec<Vec<String>>>(&input_token_stream.to_string())
        .expect(constants_str::DIAGNOSTIC_90E5793B)
        .into_iter()
        .map(|element| {
            assert!(element.iter().all(|el0| regex.is_match(el0)), "faadba8a");
            let parts_len = element.iter().map(String::len).sum::<usize>();
            let phrase_part_upper_camel_case_str = element.iter().fold(
                String::with_capacity(parts_len),
                |mut accumulator, el0| {
                    accumulator.push_str(&naming_common::domain_types::AsRefStrToUpperCamelCaseStr::case(el0));
                    accumulator
                },
            );
            let phrase_part_snake_case_str = element.iter().enumerate().fold(
                String::with_capacity(parts_len.saturating_add(element.len().saturating_sub(constants_usize::ONE))),
                |mut accumulator, (i, el0)| {
                        let element_snake_case_str = naming_common::domain_types::AsRefStrToSnakeCaseStr::case(el0);
                        if i == 0 {
                            accumulator.push_str(&element_snake_case_str);
                        } else {
                            assert!(
                                std::fmt::Write::write_fmt(&mut accumulator, format_args!("_{element_snake_case_str}"))
                                    .is_ok(),
                                "ef718915"
                            );
                        }
                        accumulator
                },
            );
            let phrase_part_upper_camel_case_upper_camel_case_token_stream = format!("{phrase_part_upper_camel_case_str}UpperCamelCase")
                .parse::<proc_macro2::TokenStream>()
                .expect(constants_str::DIAGNOSTIC_4AB6A54C);
            let phrase_part_snake_case_upper_camel_case_token_stream = format!("{phrase_part_upper_camel_case_str}SnakeCase")
                .parse::<proc_macro2::TokenStream>()
                .expect(constants_str::DIAGNOSTIC_0CC47B2E);
            let (ucc_struct_declaration_token_stream, sc_struct_declaration_token_stream) = {
                let generate_token_stream = |ts: &dyn quote::ToTokens| {
                    quote::quote! {
                        #[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
                        pub struct #ts;
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
                        &generate_quotes::dq_token_stream::dq_token_stream(&phrase_part_upper_camel_case_str),
                    ),
                    generate_token_stream(
                        &phrase_part_snake_case_upper_camel_case_token_stream,
                        &generate_quotes::dq_token_stream::dq_token_stream(&phrase_part_snake_case_str),
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
                #ucc_struct_declaration_token_stream
                #impl_display_upper_camel_case_token_stream
                #impl_to_tokens_upper_camel_case_token_stream
                #sc_struct_declaration_token_stream
                #impl_display_snake_case_token_stream
                #impl_to_tokens_snake_token_stream
            }
        });
    let generated = quote::quote! {#(#ts)*};
    generated.into()
}
#[proc_macro]
pub fn generate_self_upper_camel_case_and_snake_case_str_and_token_stream(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let regex = regex::Regex::new(constants_str::NAMING_REGEX_VALUE)
        .expect(constants_str::DIAGNOSTIC_CBA1B5FB);
    let ts = serde_json::from_str::<Vec<Vec<String>>>(&input_token_stream.to_string()).expect(constants_str::DIAGNOSTIC_9D6A20AF).into_iter().map(|element| {
        assert!(element.iter().all(|el0| regex.is_match(el0)), "4a12d90f");
        let self_match_name = constants_str::SELF_ALT;
        {
            let is_self_exists_and_only_one = element.iter().any(|el0| el0 == self_match_name);
            assert!(is_self_exists_and_only_one, "5680dd63");
        };
        let (els_concat_v_upper_camel_case_double_quoted_token_stream, els_concat_v_snake_case_double_quoted_token_stream, struct_upper_camel_case_upper_camel_case_token_stream, struct_snake_case_token_upper_camel_case_token_stream, trait_upper_camel_case_upper_camel_case_token_stream, trait_snake_case_token_upper_camel_case_token_stream) = {
            let ucc_upper_camel_case_str = constants_str::UPPERCAMELCASE;
            let sc_upper_camel_case_str = constants_str::SNAKECASE;
            let parts_len = element.iter().map(String::len).sum::<usize>();
            let els_concat_upper_camel_case_str = element.iter().fold(String::with_capacity(parts_len), |mut accumulator, el0| {
                accumulator.push_str(&naming_common::domain_types::AsRefStrToUpperCamelCaseStr::case(el0));
                accumulator
            });
            let els_concat_v_upper_camel_case_double_quoted_token_stream = generate_quotes::dq_token_stream::dq_token_stream(&element.iter().fold(String::with_capacity(parts_len), |mut accumulator, el0| {
                if el0 == constants_str::SELF_ALT {
                    accumulator.push_str(constants_str::V_ALT);
                } else {
                    accumulator.push_str(&naming_common::domain_types::AsRefStrToUpperCamelCaseStr::case(el0));
                }
                accumulator
            }));
            let els_concat_v_snake_case_double_quoted_token_stream = generate_quotes::dq_token_stream::dq_token_stream(&{
                let mut accumulator = element.iter().fold(String::with_capacity(parts_len.saturating_add(element.len())), |mut accumulator, el0| {
                    let symbol = '_';
                    if el0 == constants_str::SELF_ALT {
                        assert!(std::fmt::Write::write_fmt(&mut accumulator, format_args!("{{v}}{symbol}")).is_ok(), "6a02a2ff");
                    } else {
                        assert!(std::fmt::Write::write_fmt(&mut accumulator, format_args!("{}{symbol}", naming_common::domain_types::AsRefStrToSnakeCaseStr::case(el0))).is_ok(), "d915980a");
                    }
                    accumulator
                });
                let _: Option<char> = accumulator.pop();
                accumulator
            });
            let struct_upper_camel_case_upper_camel_case_token_stream = format!("{els_concat_upper_camel_case_str}{ucc_upper_camel_case_str}").parse::<proc_macro2::TokenStream>().expect(constants_str::DIAGNOSTIC_82F4AC08);
            let struct_snake_case_token_upper_camel_case_token_stream = format!("{els_concat_upper_camel_case_str}{sc_upper_camel_case_str}").parse::<proc_macro2::TokenStream>().expect(constants_str::DIAGNOSTIC_21044EBA);
            let (trait_upper_camel_case_upper_camel_case_token_stream, trait_snake_case_token_upper_camel_case_token_stream) = {
                let trait_upper_camel_case_str = constants_str::TRAIT;
                let trait_upper_camel_case_upper_camel_case_token_stream = format!("{els_concat_upper_camel_case_str}{ucc_upper_camel_case_str}{trait_upper_camel_case_str}").parse::<proc_macro2::TokenStream>().expect(constants_str::DIAGNOSTIC_1066857A);
                let trait_snake_case_token_upper_camel_case_token_stream = format!("{els_concat_upper_camel_case_str}{sc_upper_camel_case_str}{trait_upper_camel_case_str}").parse::<proc_macro2::TokenStream>().expect(constants_str::DIAGNOSTIC_8DB74CFD);
                (trait_upper_camel_case_upper_camel_case_token_stream, trait_snake_case_token_upper_camel_case_token_stream)
            };
            (
                els_concat_v_upper_camel_case_double_quoted_token_stream,
                els_concat_v_snake_case_double_quoted_token_stream,
                struct_upper_camel_case_upper_camel_case_token_stream,
                struct_snake_case_token_upper_camel_case_token_stream,
                trait_upper_camel_case_upper_camel_case_token_stream,
                trait_snake_case_token_upper_camel_case_token_stream,
            )
        };
        let generate_struct_token_stream = |els_concat_v_case_double_quoted_token_stream: &dyn quote::ToTokens, is_upper_camel_case: bool, trait_identifier_token_stream: &dyn quote::ToTokens| {
            let struct_identifier_token_stream = if is_upper_camel_case {
                quote::quote! {#struct_upper_camel_case_upper_camel_case_token_stream}
            } else {
                quote::quote! {#struct_snake_case_token_upper_camel_case_token_stream}
            };
            let casing_token_stream = {
                let ts = if is_upper_camel_case {
                    quote::quote! {AsRefStrToUpperCamelCaseStr::case}
                } else {
                    quote::quote! {AsRefStrToSnakeCaseStr::case}
                };
                quote::quote! {naming_common::domain_types::#ts}
            };
            let impl_to_tokens_token_stream = generate_impl_to_tokens_token_stream(
                &struct_identifier_token_stream,
                &quote::quote! {quote::ToTokens::to_tokens(&self.to_string().parse::<proc_macro2::TokenStream>().expect("71c8d26b generate_self_upper_camel_case_and_snake_case_str_and_token_stream invariant must hold"), tokens);}
            );
            quote::quote! {
                #[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::Display)]
                pub struct #struct_identifier_token_stream(String);
                impl #struct_identifier_token_stream {
                    fn wrap(v: &dyn std::fmt::Display) -> Self {
                        Self(Self::format(v))
                    }
                    fn format(v: &dyn std::fmt::Display) -> String {
                        format!(#els_concat_v_case_double_quoted_token_stream)
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
        let pub_struct_upper_camel_case_token_stream = generate_struct_token_stream(&els_concat_v_upper_camel_case_double_quoted_token_stream, true, &trait_upper_camel_case_upper_camel_case_token_stream);
        let pub_struct_snake_case_token_stream = generate_struct_token_stream(&els_concat_v_snake_case_double_quoted_token_stream, false, &trait_snake_case_token_upper_camel_case_token_stream);
        quote::quote! {
            #pub_struct_upper_camel_case_token_stream
            #pub_struct_snake_case_token_stream
        }
    });
    let generated = quote::quote! {#(#ts)*};
    generated.into()
}
fn generate_impl_trait_for_identifier_token_stream(
    name_token_stream: &dyn quote::ToTokens,
    identifier: syn_enum_identifier_ref::SynEnumIdentifierRef<'_>,
    vrts_matching_token_stream: proc_macro2_variant_matching_tokens_ref::ProcMacro2VariantMatchingTokensRef<'_>,
) -> proc_macro2_generated_naming_token_stream::ProcMacro2GeneratedNamingTokenStream {
    let identifier_ref = identifier.as_ref();
    let variant_tokens = vrts_matching_token_stream.as_ref();
    let string_token_stream = token_patterns::StringTokenStream;
    proc_macro2_generated_naming_token_stream::ProcMacro2GeneratedNamingTokenStream::from(
        quote::quote! {
            impl naming_common::domain_types::#name_token_stream for #identifier_ref {
                fn case(&self) -> #string_token_stream {
                    match self {#(#variant_tokens),*}
                }
            }
        },
    )
}
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToUpperCamelCaseStr)]
pub fn as_ref_str_enum_with_unit_fields_to_upper_camel_case_str(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let di: syn::DeriveInput =
        syn::parse(input_token_stream).expect(constants_str::DIAGNOSTIC_A8F22481);
    let identifier = &di.ident;
    let syn::Data::Enum(data_enum) = di.data else {
        std::panic::panic_any(constants_str::PANIC_D26BF85E)
    };
    let generated = generate_impl_trait_for_identifier_token_stream(
        &quote::quote! {AsRefStrToUpperCamelCaseStr},
        syn_enum_identifier_ref::SynEnumIdentifierRef::from(identifier),
        proc_macro2_variant_matching_tokens_ref::ProcMacro2VariantMatchingTokensRef::from(
            data_enum
                .variants
                .iter()
                .map(|element| match element.fields {
                    syn::Fields::Unit => {
                        let element_identifier = &element.ident;
                        let element_identifier_upper_camel_case_double_quoted_token_stream =
                            generate_quotes::dq_token_stream::dq_token_stream(&naming_common::domain_types::ToTokensToUpperCamelCaseStr::case(&element_identifier));
                        quote::quote! {Self::#element_identifier => String::from(#element_identifier_upper_camel_case_double_quoted_token_stream)}
                    }
                    syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                        std::panic::panic_any(constants_str::PANIC_4955C50D)
                    }
                })
                .collect::<Vec<proc_macro2::TokenStream>>()
                .as_slice(),
        ),
    );
    proc_macro2::TokenStream::from(generated).into()
}
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToSnakeCaseStr)]
pub fn as_ref_str_enum_with_unit_fields_to_snake_case_str(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let di: syn::DeriveInput =
        syn::parse(input_token_stream).expect(constants_str::DIAGNOSTIC_DEA5CBCF);
    let identifier = &di.ident;
    let syn::Data::Enum(data_enum) = di.data else {
        std::panic::panic_any(constants_str::PANIC_ED6EFE2E);
    };
    let generated = generate_impl_trait_for_identifier_token_stream(
        &quote::quote! {AsRefStrToSnakeCaseStr},
        syn_enum_identifier_ref::SynEnumIdentifierRef::from(identifier),
        proc_macro2_variant_matching_tokens_ref::ProcMacro2VariantMatchingTokensRef::from(
            data_enum
                .variants
                .iter()
                .map(|element| match element.fields {
                    syn::Fields::Unit => {
                        let element_identifier = &element.ident;
                        let element_identifier_snake_case_double_quoted_token_stream =
                            generate_quotes::dq_token_stream::dq_token_stream(&naming_common::domain_types::ToTokensToSnakeCaseStr::case(&element_identifier));
                        quote::quote! {Self::#element_identifier => String::from(#element_identifier_snake_case_double_quoted_token_stream)}
                    }
                    syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                        std::panic::panic_any(constants_str::PANIC_B3EF2657)
                    }
                })
                .collect::<Vec<proc_macro2::TokenStream>>()
                .as_slice(),
        ),
    );
    proc_macro2::TokenStream::from(generated).into()
}
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToUpperSnakeCaseStr)]
pub fn as_ref_str_enum_with_unit_fields_to_upper_snake_case_str(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let di: syn::DeriveInput =
        syn::parse(input_token_stream).expect(constants_str::DIAGNOSTIC_EDABBC24);
    let identifier = &di.ident;
    let syn::Data::Enum(data_enum) = di.data else {
        std::panic::panic_any(constants_str::PANIC_B2263E7E);
    };
    let generated = generate_impl_trait_for_identifier_token_stream(
        &quote::quote! {AsRefStrToUpperSnakeCaseStr},
        syn_enum_identifier_ref::SynEnumIdentifierRef::from(identifier),
        proc_macro2_variant_matching_tokens_ref::ProcMacro2VariantMatchingTokensRef::from(
            data_enum
                .variants
                .iter()
                .map(|element| match element.fields {
                    syn::Fields::Unit => {
                        let element_identifier = &element.ident;
                        let element_identifier_snake_case_double_quoted_token_stream =
                            generate_quotes::dq_token_stream::dq_token_stream(&naming_common::domain_types::ToTokensToUpperSnakeCaseStr::case(&element_identifier));
                        quote::quote! {Self::#element_identifier => String::from(#element_identifier_snake_case_double_quoted_token_stream)}
                    }
                    syn::Fields::Named(_) | syn::Fields::Unnamed(_) => std::panic::panic_any(constants_str::PANIC_B6FEDCFF),
                })
                .collect::<Vec<proc_macro2::TokenStream>>()
                .as_slice(),
        ),
    );
    proc_macro2::TokenStream::from(generated).into()
}

fn enum_with_unit_fields_to_case_str(
    input_token_stream: proc_macro::TokenStream,
    case: EnumCase,
) -> proc_macro::TokenStream {
    let di: syn::DeriveInput =
        syn::parse(input_token_stream).expect(constants_str::DIAGNOSTIC_C0E870DF);
    let identifier = &di.ident;
    let syn::Data::Enum(data_enum) = di.data else {
        std::panic::panic_any(constants_str::PANIC_3FDBA6EF)
    };
    let method_identifier = syn::Ident::new(
        match case {
            EnumCase::Snake => stringify!(as_snake_case_str),
            EnumCase::UpperCamel => stringify!(as_upper_camel_case_str),
            EnumCase::UpperSnake => stringify!(as_upper_snake_case_str),
        },
        proc_macro2::Span::call_site(),
    );
    let variant_tokens = data_enum.variants.iter().map(|variant| {
        assert!(matches!(variant.fields, syn::Fields::Unit), "4a95589f");
        let variant_identifier = &variant.ident;
        let value = match case {
            EnumCase::Snake => {
                naming_common::domain_types::ToTokensToSnakeCaseStr::case(variant_identifier)
            }
            EnumCase::UpperCamel => {
                naming_common::domain_types::ToTokensToUpperCamelCaseStr::case(variant_identifier)
            }
            EnumCase::UpperSnake => {
                naming_common::domain_types::ToTokensToUpperSnakeCaseStr::case(variant_identifier)
            }
        };
        let value_token_stream = generate_quotes::dq_token_stream::dq_token_stream(&value);
        let cfg_attributes = variant
            .attrs
            .iter()
            .filter(|attribute| attribute.path().is_ident(stringify!(cfg)));
        quote::quote! {
            #(#cfg_attributes)*
            Self::#variant_identifier => #value_token_stream
        }
    });
    quote::quote! {
        impl #identifier {
            pub(crate) const fn #method_identifier(&self) -> &'static str {
                match self {
                    #(#variant_tokens),*
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(EnumWithUnitFieldsToSnakeCaseStr)]
pub fn enum_with_unit_fields_to_snake_case_str(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    enum_with_unit_fields_to_case_str(input_token_stream, EnumCase::Snake)
}

#[proc_macro_derive(EnumWithUnitFieldsToUpperCamelCaseStr)]
pub fn enum_with_unit_fields_to_upper_camel_case_str(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    enum_with_unit_fields_to_case_str(input_token_stream, EnumCase::UpperCamel)
}

#[proc_macro_derive(EnumWithUnitFieldsToUpperSnakeCaseStr)]
pub fn enum_with_unit_fields_to_upper_snake_case_str(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    enum_with_unit_fields_to_case_str(input_token_stream, EnumCase::UpperSnake)
}
