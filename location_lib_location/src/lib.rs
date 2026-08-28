mod syn_item_enum_mut_ref;
#[proc_macro_attribute]
pub fn errors_with_location(
    attr_token_stream: proc_macro::TokenStream,
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if !attr_token_stream.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            constants_str::ERRORS_WITH_LOCATION_DOES_NOT_ACCEPT_ARGUMENTS,
        )
        .into_compile_error()
        .into();
    }
    let mut item = match syn::parse::<syn::ItemEnum>(input_token_stream) {
        Ok(v) => v,
        Err(error) => return error.into_compile_error().into(),
    };
    match add_location_fields(syn_item_enum_mut_ref::SynItemEnumMutRef::from(&mut item)) {
        Ok(()) => quote::quote! {#item}.into(),
        Err(error) => error.into_compile_error().into(),
    }
}
// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(clippy::single_call_fn)] // isolated transformation is unit-tested independently from proc-macro parsing
fn add_location_fields(item: syn_item_enum_mut_ref::SynItemEnumMutRef<'_>) -> syn::Result<()> {
    let item_ref = item.into_inner();
    item_ref.variants.iter_mut().try_for_each(|variant| {
        let syn::Fields::Named(fields) = &mut variant.fields else {
            return Err(syn::Error::new_spanned(
                variant,
                constants_str::ERRORS_WITH_LOCATION_SUPPORTS_ONLY_VARIANTS_WITH_NAMED_FIELDS,
            ));
        };
        if fields.named.iter().any(|field| {
            field
                .ident
                .as_ref()
                .is_some_and(|identifier| identifier == constants_str::LOCATION_ALT)
        }) {
            return Err(syn::Error::new_spanned(
                variant,
                constants_str::ERRORS_WITH_LOCATION_VARIANT_ALREADY_HAS_A_LOCATION_FIELD,
            ));
        }
        fields
            .named
            .push(syn::parse_quote! { location: location_lib::domain_types::Location });
        Ok(())
    })
}
#[proc_macro_derive(
    Location,
    attributes(
        eo_to_err_string,
        eo_to_err_string_serde,
        eo_location,
        eo_vec_to_err_string,
        eo_vec_to_err_string_serde,
        eo_vec_location,
        eo_hashmap_k_string_v_to_err_string,
        eo_hashmap_k_string_v_to_err_string_serde,
        eo_hashmap_k_string_v_location,
        location_to_schema,
    )
)]
pub fn derive_location(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, optimal_memory_layout::OptimalMemoryLayout)]
    enum SuportedEnumVariant {
        Named,
        Unnamed,
    }
    panic_location::panic_location();
    let di: syn::DeriveInput = syn::parse(input).expect("d94f091a location invariant must hold");
    let utoipa_to_schema_token_stream = di
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident(constants_str::LOCATION_TO_SCHEMA))
        .then(|| quote::quote! {utoipa::ToSchema,});
    let identifier = &di.ident;
    let string_token_stream = token_patterns::StringTokenStream;
    let location_snake_case = naming::domain_types::LocationSnakeCase;
    let v_snake_case = naming::domain_types::VSnakeCase;
    let into_serde_version_snake_case = naming::domain_types::IntoSerdeVersionSnakeCase;
    let generic_parameters = &di
        .generics
        .params
        .iter()
        .map(|element_a6a747c1| match &element_a6a747c1 {
            syn::GenericParam::Type(v) => &v.ident,
            syn::GenericParam::Lifetime(_) | syn::GenericParam::Const(_) => {
                panic!("3ce82d11")
            }
        })
        .collect::<Vec<&syn::Ident>>();
    let identifier_with_serde_upper_camel_case =
        naming::domain_types::parameter::SelfWithSerdeUpperCamelCase::from_tokens(&identifier);
    let syn::Data::Enum(data_enum) = di.data else {
        panic!("d98214f7");
    };
    let supported_enum_variant = {
        let mut all_eq: Option<SuportedEnumVariant> = None;
        assert!(!data_enum.variants.is_empty(), "27275ae6");
        data_enum
            .variants
            .iter()
            .for_each(|variant| match &variant.fields {
                syn::Fields::Named(_) => match &all_eq {
                    Some(supported_variant) => {
                        assert!(
                            !(*supported_variant == SuportedEnumVariant::Unnamed),
                            "bf6be520"
                        );
                    }
                    None => {
                        all_eq = Some(SuportedEnumVariant::Named);
                    }
                },
                syn::Fields::Unnamed(_) => match &all_eq {
                    Some(supported_variant) => {
                        assert!(
                            !(*supported_variant == SuportedEnumVariant::Named),
                            "02090d85"
                        );
                    }
                    None => {
                        all_eq = Some(SuportedEnumVariant::Unnamed);
                    }
                },
                syn::Fields::Unit => panic!("2f2e9385"),
            });
        all_eq.expect("b9da972a location invariant must hold")
    };
    let maybe_generic_parameters_token_stream = if generic_parameters.is_empty() {
        proc_macro2::TokenStream::new()
    } else {
        quote::quote! {<#(#generic_parameters),*>}
    };
    let maybe_generic_parameters_location_lib_to_err_string_anns_token_stream =
        if generic_parameters.is_empty() {
            proc_macro2::TokenStream::new()
        } else {
            let v = generic_parameters
                .iter()
                .map(|element| quote::quote! {#element: to_err_string::domain_types::ToErrString});
            quote::quote! {<#(#v),*>}
        };
    let generate_enum_identifier_with_serde_token_stream = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize, #utoipa_to_schema_token_stream optimal_memory_layout::OptimalMemoryLayout)]
            pub enum #identifier_with_serde_upper_camel_case #maybe_generic_parameters_token_stream {
                #ts
            }
        }
    };
    let generate_impl_identifier_into_serde_version_token_stream = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            impl #maybe_generic_parameters_token_stream #identifier #maybe_generic_parameters_token_stream {
                pub fn #into_serde_version_snake_case(self) -> #identifier_with_serde_upper_camel_case #maybe_generic_parameters_token_stream {
                    // The owner module retains lint-sensitive semantics from the original implementation.
                    #[allow(clippy::redundant_closure_for_method_calls)]
                    match self {
                        #ts
                    }
                }
            }
        }
    };
    let tokens = match supported_enum_variant {
        SuportedEnumVariant::Named => {
            let location_snake_case_str = naming::domain_types::LocationSnakeCase.to_string();
            //todo maybe impl display was a bad idea. .to_string() casts is dangerous
            let impl_display_token_stream = {
                let vrts_token_stream = data_enum.variants.iter().map(|element| {
                    let element_identifier = &element.ident;
                    let fields = if let syn::Fields::Named(fields) = &element.fields {
                        &fields.named
                    } else {
                        panic!("f64e0d21");
                    };
                    let fields_idents_excluding_location_token_stream = {
                        let accumulator_token_stream = fields.iter()
                        .filter(|el0| *el0.ident.as_ref().expect("07504636 location invariant must hold") != *location_snake_case_str)
                        .map(|el0| el0.ident.as_ref().expect("971ace15 location invariant must hold"))
                        .collect::<Vec<&syn::Ident>>();
                        if accumulator_token_stream.is_empty() {
                            proc_macro2::TokenStream::new()
                        }
                        else {
                            quote::quote! {#(#accumulator_token_stream),*,}
                        }
                    };
                    let fields_format_excluding_location_token_stream = generate_quotes::domain_types::dq_token_stream(
                        &fields.iter()
                        .filter(|el0| *el0.ident.as_ref().expect("3d70a4f4 location invariant must hold") != *location_snake_case_str)
                        .fold(
                            String::new(),
                            |mut accumulator, el0| {
                                let el0_identifier = &el0.ident.as_ref().expect("2e7cd5fe location invariant must hold");
                                assert!(
                                    std::fmt::Write::write_fmt(
                                        &mut accumulator,
                                        format_args!("{el0_identifier}: {{}}\n")
                                    ).is_ok(),
                                    "ab44c70f"
                                );
                                accumulator
                            }
                        )
                    );
                    let fields_format_vs_excluding_location_token_stream = fields.iter()
                    .filter(|el0| *el0.ident.as_ref().expect("f6f6fb24 location invariant must hold") != *location_snake_case_str)
                    .map(|el0| {
                        let el0_identifier = &el0.ident.as_ref().expect("e97b25b9 location invariant must hold");
                        match macro_helpers::domain_types::location_data::LocationFieldAttr::try_from(el0).expect("8ff56aeb location invariant must hold") {
                            macro_helpers::domain_types::location_data::LocationFieldAttr::EoToErrString | macro_helpers::domain_types::location_data::LocationFieldAttr::EoToErrStringSerde => {
                                quote::quote! {
                                    to_err_string::domain_types::ToErrString::to_err_string(#el0_identifier)
                                }
                            }
                            macro_helpers::domain_types::location_data::LocationFieldAttr::EoLocation => {
                                let if_write_is_err_token_stream = macro_helpers::domain_types::generate_if_write_is_error_token_stream::generate_if_write_is_error_token_stream(&quote::quote! {accumulator_52e70d22, "\n {element}"}, &quote::quote! {panic!("c751d54a");});
                                quote::quote! {
                                    #el0_identifier.to_string().lines().fold(
                                        #string_token_stream::new(),
                                        |mut accumulator_52e70d22, element| {
                                            #if_write_is_err_token_stream
                                            accumulator_52e70d22
                                        }
                                    )
                                }
                            }
                            macro_helpers::domain_types::location_data::LocationFieldAttr::EoVecToErrString | macro_helpers::domain_types::location_data::LocationFieldAttr::EoVecToErrStringSerde => {
                                let if_write_is_err_token_stream = macro_helpers::domain_types::generate_if_write_is_error_token_stream::generate_if_write_is_error_token_stream(&quote::quote! {accumulator_a9ba7521, "\n {element_6e4f53ad}"}, &quote::quote! {panic!("b35ed9f5");});
                                quote::quote! {
                                    #el0_identifier.iter().fold(
                                        #string_token_stream::new(),
                                        |mut accumulator_ac447c4b, element| {
                                            accumulator_ac447c4b.push_str(
                                                &to_err_string::domain_types::ToErrString::to_err_string(element)
                                                .lines()
                                                .fold(
                                                    #string_token_stream::new(),
                                                    |mut accumulator_a9ba7521, element_6e4f53ad| {
                                                        #if_write_is_err_token_stream
                                                        accumulator_a9ba7521
                                                    }
                                                )
                                            );
                                            accumulator_ac447c4b
                                        }
                                    )
                                }
                            }
                            macro_helpers::domain_types::location_data::LocationFieldAttr::EoVecLocation => {
                                let if_write_is_err_token_stream = macro_helpers::domain_types::generate_if_write_is_error_token_stream::generate_if_write_is_error_token_stream(&quote::quote! {accumulator_1bbd5ef3, "\n {element_3f2fe01d}"}, &quote::quote! {panic!("4dfdd18d");});
                                quote::quote! {
                                    #el0_identifier.iter().fold(
                                        #string_token_stream::new(),
                                        |mut accumulator_c5adba93, element_37c46c8a| {
                                            accumulator_c5adba93.push_str(&element_37c46c8a.to_string().lines().fold(
                                                #string_token_stream::new(),
                                                |mut accumulator_1bbd5ef3, element_3f2fe01d| {
                                                    #if_write_is_err_token_stream
                                                    accumulator_1bbd5ef3
                                                },
                                            ));
                                            accumulator_c5adba93
                                        }
                                    )
                                }
                            }
                            macro_helpers::domain_types::location_data::LocationFieldAttr::EoHashMapKStringVToErrString | macro_helpers::domain_types::location_data::LocationFieldAttr::EoHashMapKStringVToErrStringSerde => {
                                let if_write_is_err_token_stream = macro_helpers::domain_types::generate_if_write_is_error_token_stream::generate_if_write_is_error_token_stream(&quote::quote! {accumulator_06473093, "\n {}: {}", &to_err_string::domain_types::ToErrString::to_err_string(k), &to_err_string::domain_types::ToErrString::to_err_string(#v_snake_case)}, &quote::quote! {panic!("d030580a");});
                                quote::quote! {
                                    #el0_identifier.iter().fold(
                                        #string_token_stream::new(),
                                        |mut accumulator_06473093, (k, #v_snake_case)| {
                                            #if_write_is_err_token_stream
                                            accumulator_06473093
                                        }
                                    )
                                }
                            }
                            macro_helpers::domain_types::location_data::LocationFieldAttr::EoHashMapKStringVLocation => {
                                let if_write_is_err_token_stream = macro_helpers::domain_types::generate_if_write_is_error_token_stream::generate_if_write_is_error_token_stream(
                                    &{
                                        let if_write_is_err_token_stream = macro_helpers::domain_types::generate_if_write_is_error_token_stream::generate_if_write_is_error_token_stream(&quote::quote! {accumulator_addfc699, "\n  {element_8b8f577e}"}, &quote::quote! {panic!("d0492fbf");});
                                        quote::quote! {
                                            accumulator_a47e1ba7,
                                            "\n {}: {}",
                                            to_err_string::domain_types::ToErrString::to_err_string(k),
                                            #v_snake_case.to_string().lines().fold(
                                                #string_token_stream::new(),
                                                |mut accumulator_addfc699, element_8b8f577e| {
                                                    #if_write_is_err_token_stream
                                                    accumulator_addfc699
                                                }
                                            )
                                        }
                                    },
                                    &quote::quote! {panic!("75f6432a");},
                                );
                                quote::quote! {
                                    #el0_identifier.iter().fold(
                                        #string_token_stream::new(),
                                        |mut accumulator_a47e1ba7, (k, #v_snake_case)| {
                                            #if_write_is_err_token_stream
                                            accumulator_a47e1ba7
                                        }
                                    )
                                }
                            }
                        }
                    });
                    quote::quote! {
                        Self::#element_identifier {
                            #fields_idents_excluding_location_token_stream
                            ..
                        } => {
                            format!(
                                #fields_format_excluding_location_token_stream,
                                #(#fields_format_vs_excluding_location_token_stream),*
                            )
                        }
                    }
                });
                let location_variants_token_stream =
                    data_enum.variants.iter().enumerate().map(|(i, element)| {
                        let element_identifier = &element.ident;
                        if i == 0 {
                            quote::quote! {
                                Self::#element_identifier {
                                    #location_snake_case,
                                    ..
                                }
                            }
                        } else {
                            quote::quote! {
                                | Self::#element_identifier {
                                    #location_snake_case,
                                    ..
                                }
                            }
                        }
                    });
                quote::quote! {
                    write!(
                        f,
                        "{}{}",
                        match self {
                            #(#vrts_token_stream),*
                        },
                        match self {
                            #(#location_variants_token_stream)*
                            => #location_snake_case
                        }
                    )
                }
            };
            let impl_display_for_identifier_token_stream =
                macro_helpers::domain_types::generate_impl_display_token_stream::generate_impl_display_token_stream(
                    &maybe_generic_parameters_location_lib_to_err_string_anns_token_stream,
                    &identifier,
                    &maybe_generic_parameters_token_stream,
                    &impl_display_token_stream,
                );
            let impl_identifier_into_serde_version_token_stream = {
                let vrts_token_stream = data_enum.variants.iter().map(|element| {
                    let element_identifier = &element.ident;
                    let fields = if let syn::Fields::Named(fields) = &element.fields {
                        &fields.named
                    } else {
                        panic!("238b402b");
                    };
                    let fields_idents_token_stream = fields.iter().map(|el0| &el0.ident);
                    let fields_token_stream = fields.iter()
                    .map(|el0| {
                        let el0_identifier = &el0.ident.as_ref().expect("9a672ac2 location invariant must hold");
                        if **el0_identifier == *location_snake_case_str {
                            quote::quote! {#el0_identifier}
                        }
                        else {
                            let generate_field_token_stream = |ts: &dyn quote::ToTokens|quote::quote! {#el0_identifier: {#ts}};
                            match macro_helpers::domain_types::location_data::LocationFieldAttr::try_from(el0).expect("449c3781 location invariant must hold") {
                                macro_helpers::domain_types::location_data::LocationFieldAttr::EoToErrString => generate_field_token_stream(&quote::quote! {
                                    to_err_string::domain_types::ToErrString::to_err_string(&#el0_identifier).into_inner()
                                }),
                                macro_helpers::domain_types::location_data::LocationFieldAttr::EoToErrStringSerde | macro_helpers::domain_types::location_data::LocationFieldAttr::EoVecToErrStringSerde => {
                                    quote::quote! {#el0_identifier}
                                }
                                macro_helpers::domain_types::location_data::LocationFieldAttr::EoLocation => generate_field_token_stream(&quote::quote! {
                                    #el0_identifier.into_serde_version()
                                }),
                                macro_helpers::domain_types::location_data::LocationFieldAttr::EoVecToErrString => generate_field_token_stream(&quote::quote! {
                                    #el0_identifier.into_iter().map(|element|to_err_string::domain_types::ToErrString::to_err_string(&element).into_inner()).collect()
                                }),
                                macro_helpers::domain_types::location_data::LocationFieldAttr::EoVecLocation => generate_field_token_stream(&quote::quote! {
                                    #el0_identifier.into_iter().map(|element|element.into_serde_version()).collect()
                                }),
                                macro_helpers::domain_types::location_data::LocationFieldAttr::EoHashMapKStringVToErrString => generate_field_token_stream(&quote::quote! {
                                    #el0_identifier.into_iter().map(
                                        |(k, v)|(to_err_string::domain_types::ToErrString::to_err_string(&k).into_inner(), to_err_string::domain_types::ToErrString::to_err_string(&v).into_inner())
                                    ).collect()
                                }),
                                macro_helpers::domain_types::location_data::LocationFieldAttr::EoHashMapKStringVToErrStringSerde => generate_field_token_stream(&quote::quote! {
                                    #el0_identifier.into_iter().map(
                                        |(k, v)|(to_err_string::domain_types::ToErrString::to_err_string(&k).into_inner(), v)
                                    ).collect()
                                }),
                                macro_helpers::domain_types::location_data::LocationFieldAttr::EoHashMapKStringVLocation => generate_field_token_stream(&quote::quote! {
                                    #el0_identifier.into_iter().map(
                                        |(k, v)|(to_err_string::domain_types::ToErrString::to_err_string(&k).into_inner(), v.into_serde_version())
                                    ).collect()
                                }),
                            }
                        }
                    });
                    quote::quote! {
                        Self::#element_identifier {
                            #(#fields_idents_token_stream),*
                        } => #identifier_with_serde_upper_camel_case::#element_identifier {
                            #(#fields_token_stream),*
                        }
                    }
                });
                generate_impl_identifier_into_serde_version_token_stream(
                    &quote::quote! {#(#vrts_token_stream),*},
                )
            };
            let enum_identifier_with_serde_token_stream = {
                let vrts_token_stream = data_enum.variants.iter().map(|variant| {
                    macro_helpers::domain_types::location_data::generate_serde_version_of_named_syn_variant(
                        macro_helpers::domain_types::location_data::SynVariantRef::from(variant),
                    )
                });
                generate_enum_identifier_with_serde_token_stream(
                    &quote::quote! {#(#vrts_token_stream),*},
                )
            };
            let impl_display_for_identifier_with_serde_token_stream =
                macro_helpers::domain_types::generate_impl_display_token_stream::generate_impl_display_token_stream(
                    &maybe_generic_parameters_location_lib_to_err_string_anns_token_stream,
                    &identifier_with_serde_upper_camel_case,
                    &maybe_generic_parameters_token_stream,
                    &impl_display_token_stream,
                );
            let impl_location_lib_to_err_string_to_err_string_for_identifier_with_serde_token_stream =
                macro_helpers::domain_types::generate_impl_to_err_string_token_stream::generate_impl_to_err_string_token_stream(
                    &maybe_generic_parameters_location_lib_to_err_string_anns_token_stream,
                    &identifier_with_serde_upper_camel_case,
                    &maybe_generic_parameters_token_stream,
                    &quote::quote! {format!("{self}")},
                );
            quote::quote! {
                #impl_display_for_identifier_token_stream
                #impl_identifier_into_serde_version_token_stream
                #enum_identifier_with_serde_token_stream
                #impl_display_for_identifier_with_serde_token_stream
                #impl_location_lib_to_err_string_to_err_string_for_identifier_with_serde_token_stream
            }
        }
        SuportedEnumVariant::Unnamed => {
            let display_formatter_unnamed_token_stream = {
                let vrts_token_stream = data_enum.variants.iter().map(|element| {
                    let element_identifier = &element.ident;
                    quote::quote! {Self::#element_identifier(v) => v}
                });
                quote::quote! {match self { #(#vrts_token_stream),* }}
            };
            let impl_display_for_identifier_token_stream =
                macro_helpers::domain_types::generate_impl_display_token_stream::generate_impl_display_token_stream(
                    &maybe_generic_parameters_location_lib_to_err_string_anns_token_stream,
                    &identifier,
                    &maybe_generic_parameters_token_stream,
                    &quote::quote! {
                        write!(
                            f,
                            "{}",
                            #display_formatter_unnamed_token_stream
                        )
                    },
                );
            let impl_identifier_into_serde_version_token_stream = {
                let vrts_token_stream = data_enum.variants.iter().map(|element| {
                    let element_identifier = &element.ident;
                    quote::quote! {
                        Self::#element_identifier(v) => #identifier_with_serde_upper_camel_case::#element_identifier(
                            v.#into_serde_version_snake_case(),
                        )
                    }
                });
                generate_impl_identifier_into_serde_version_token_stream(
                    &quote::quote! {#(#vrts_token_stream),*},
                )
            };
            let enum_identifier_with_serde_token_stream = {
                let vrts_token_stream = data_enum.variants.iter().map(|element| {
                    let element_identifier = &element.ident;
                    let fields = if let syn::Fields::Unnamed(fields) = &element.fields {
                        &fields.unnamed
                    } else {
                        panic!("5749e920");
                    };
                    let inner_type_with_serde_token_stream = {
                        format!(
                            "{}{}",
                            {
                                assert!(fields.len() == 1, "d7a6b955");
                                let field_type = &fields
                                    .iter()
                                    .next()
                                    .expect("8a80c36d location invariant must hold")
                                    .ty;
                                quote::quote! {#field_type}.to_string()
                            },
                            naming::domain_types::WithSerdeUpperCamelCase
                        )
                        .parse::<proc_macro2::TokenStream>()
                        .expect("9ff40f7e location invariant must hold")
                    };
                    quote::quote! {#element_identifier(#inner_type_with_serde_token_stream)}
                });
                generate_enum_identifier_with_serde_token_stream(
                    &quote::quote! {#(#vrts_token_stream),*},
                )
            };
            let impl_display_for_identifier_with_serde_token_stream =
                macro_helpers::domain_types::generate_impl_display_token_stream::generate_impl_display_token_stream(
                    &maybe_generic_parameters_location_lib_to_err_string_anns_token_stream,
                    &identifier_with_serde_upper_camel_case,
                    &maybe_generic_parameters_token_stream,
                    &quote::quote! {
                        write!(
                            f,
                            "{}",
                            #display_formatter_unnamed_token_stream
                        )
                    },
                );
            //todo maybe make a trait?
            quote::quote! {
                #impl_display_for_identifier_token_stream
                #impl_identifier_into_serde_version_token_stream
                #enum_identifier_with_serde_token_stream
                #impl_display_for_identifier_with_serde_token_stream
            }
        }
    };
    let generated = quote::quote! {#tokens};
    generated.into()
}
#[cfg(test)]
mod tests {
    #[test]
    fn adds_location_to_every_named_variant() {
        let mut item: syn::ItemEnum = syn::parse_quote! {
            enum SampleError {
                First { value: String },
                Second {},
            }
        };
        super::add_location_fields(super::syn_item_enum_mut_ref::SynItemEnumMutRef::from(
            &mut item,
        ))
        .expect("74c1509e adds_location_to_every_named_variant invariant must hold");
        assert_eq!(
            quote::quote! {#item}.to_string(),
            quote::quote! {
                enum SampleError {
                    First { value: String, location: location_lib::domain_types::Location },
                    Second { location: location_lib::domain_types::Location },
                }
            }
            .to_string()
        );
    }
    #[test]
    fn rejects_existing_location_field() {
        let mut item: syn::ItemEnum = syn::parse_quote! {
            enum SampleError { First { location: location_lib::domain_types::Location } }
        };
        let error = super::add_location_fields(
            super::syn_item_enum_mut_ref::SynItemEnumMutRef::from(&mut item),
        )
        .expect_err(constants_str::VALUE_371082FA);
        assert_eq!(
            error.to_string(),
            "errors_with_location variant already has a location field"
        );
    }
    #[test]
    fn rejects_unnamed_variant() {
        let mut item: syn::ItemEnum = syn::parse_quote! {
            enum SampleError { First(String) }
        };
        let error = super::add_location_fields(
            super::syn_item_enum_mut_ref::SynItemEnumMutRef::from(&mut item),
        )
        .expect_err(constants_str::VALUE_982F4D17);
        assert_eq!(
            error.to_string(),
            "errors_with_location supports only variants with named fields"
        );
    }
}
