struct SynItemEnumMutRef<'item_lt>(&'item_lt mut syn::ItemEnum);
impl<'item_lt> From<&'item_lt mut syn::ItemEnum> for SynItemEnumMutRef<'item_lt> {
    fn from(value: &'item_lt mut syn::ItemEnum) -> Self {
        Self(value)
    }
}
#[proc_macro_attribute]
pub fn errors_with_loc(
    attr_ts: proc_macro::TokenStream,
    input_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if !attr_ts.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "errors_with_loc does not accept arguments",
        )
        .into_compile_error()
        .into();
    }
    let mut item = match syn::parse::<syn::ItemEnum>(input_ts) {
        Ok(v) => v,
        Err(er) => return er.into_compile_error().into(),
    };
    match add_loc_fields(SynItemEnumMutRef::from(&mut item)) {
        Ok(()) => quote::quote! {#item}.into(),
        Err(er) => er.into_compile_error().into(),
    }
}
#[allow(clippy::single_call_fn)] // isolated transformation is unit-tested independently from proc-macro parsing
fn add_loc_fields(item: SynItemEnumMutRef<'_>) -> syn::Result<()> {
    let SynItemEnumMutRef(item_ref) = item;
    item_ref.variants.iter_mut().try_for_each(|variant| {
        let syn::Fields::Named(fields) = &mut variant.fields else {
            return Err(syn::Error::new_spanned(
                variant,
                "errors_with_loc supports only variants with named fields",
            ));
        };
        if fields
            .named
            .iter()
            .any(|field| field.ident.as_ref().is_some_and(|ident| ident == "loc"))
        {
            return Err(syn::Error::new_spanned(
                variant,
                "errors_with_loc variant already has a loc field",
            ));
        }
        fields
            .named
            .push(syn::parse_quote! { loc: loc_lib::loc::Loc });
        Ok(())
    })
}
#[proc_macro_derive(
    Location,
    attributes(
        eo_to_err_string,
        eo_to_err_string_serde,
        eo_loc,
        eo_vec_to_err_string,
        eo_vec_to_err_string_serde,
        eo_vec_loc,
        eo_hashmap_k_string_v_to_err_string,
        eo_hashmap_k_string_v_to_err_string_serde,
        eo_hashmap_k_string_v_loc,
    )
)]
pub fn loc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml)]
    enum SuportedEnumVrt {
        Named,
        Unnamed,
    }
    panic_loc::panic_loc();
    let di: syn::DeriveInput = syn::parse(input).expect("d94f091a");
    let ident = &di.ident;
    let string_ts = token_patterns::StringTs;
    let loc_sc = naming::LocSc;
    let v_sc = naming::VSc;
    let into_serde_version_sc = naming::IntoSerdeVersionSc;
    let generic_prms = &di
        .generics
        .params
        .iter()
        .map(|el_a6a747c1| match &el_a6a747c1 {
            syn::GenericParam::Type(v) => &v.ident,
            syn::GenericParam::Lifetime(_) | syn::GenericParam::Const(_) => {
                panic!("3ce82d11")
            }
        })
        .collect::<Vec<&syn::Ident>>();
    let ident_with_serde_ucc = naming::prm::SelfWithSerdeUcc::from_tokens(&ident);
    let syn::Data::Enum(data_enum) = di.data else {
        panic!("d98214f7");
    };
    let supported_enum_vrt = {
        let mut all_eq: Option<SuportedEnumVrt> = None;
        assert!(!data_enum.variants.is_empty(), "27275ae6");
        data_enum.variants.iter().for_each(|vrt| match &vrt.fields {
            syn::Fields::Named(_) => match &all_eq {
                Some(supported_vrt) => {
                    assert!(!(*supported_vrt == SuportedEnumVrt::Unnamed), "bf6be520");
                }
                None => {
                    all_eq = Some(SuportedEnumVrt::Named);
                }
            },
            syn::Fields::Unnamed(_) => match &all_eq {
                Some(supported_vrt) => {
                    assert!(!(*supported_vrt == SuportedEnumVrt::Named), "02090d85");
                }
                None => {
                    all_eq = Some(SuportedEnumVrt::Unnamed);
                }
            },
            syn::Fields::Unit => panic!("2f2e9385"),
        });
        all_eq.expect("b9da972a")
    };
    let mb_generic_prms_ts = if generic_prms.is_empty() {
        proc_macro2::TokenStream::new()
    } else {
        quote::quote! {<#(#generic_prms),*>}
    };
    let mb_generic_prms_loc_lib_to_err_string_anns_ts = if generic_prms.is_empty() {
        proc_macro2::TokenStream::new()
    } else {
        let v = generic_prms
            .iter()
            .map(|el| quote::quote! {#el: to_err_string::ToErrString});
        quote::quote! {<#(#v),*>}
    };
    let gen_enum_ident_with_serde_ts = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize, optml::Optml)]
            pub enum #ident_with_serde_ucc #mb_generic_prms_ts {
                #ts
            }
        }
    };
    let gen_impl_ident_into_serde_version_ts = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            impl #mb_generic_prms_ts #ident #mb_generic_prms_ts {
                pub fn #into_serde_version_sc(self) -> #ident_with_serde_ucc #mb_generic_prms_ts {
                    #[allow(clippy::redundant_closure_for_method_calls)]
                    match self {
                        #ts
                    }
                }
            }
        }
    };
    let tokens = match supported_enum_vrt {
        SuportedEnumVrt::Named => {
            let loc_sc_str = naming::LocSc.to_string();
            //todo mb impl display was a bad idea. .to_string() casts is dangerous
            let impl_display_h_ts = {
                let vrts_ts = data_enum.variants.iter().map(|el| {
                    let el_ident = &el.ident;
                    let fields = if let syn::Fields::Named(fields) = &el.fields {
                        &fields.named
                    } else {
                        panic!("f64e0d21");
                    };
                    let fields_idents_excluding_loc_ts = {
                        let acc_ts = fields.iter()
                        .filter(|el0| *el0.ident.as_ref().expect("07504636") != *loc_sc_str)
                        .map(|el0| el0.ident.as_ref().expect("971ace15"))
                        .collect::<Vec<&syn::Ident>>();
                        if acc_ts.is_empty() {
                            proc_macro2::TokenStream::new()
                        }
                        else {
                            quote::quote! {#(#acc_ts),*,}
                        }
                    };
                    let fields_format_excluding_loc_ts = gen_quotes::dq_ts(
                        &fields.iter()
                        .filter(|el0| *el0.ident.as_ref().expect("3d70a4f4") != *loc_sc_str)
                        .fold(
                            String::new(),
                            |mut acc, el0| {
                                let el0_ident = &el0.ident.as_ref().expect("2e7cd5fe");
                                assert!(
                                    std::fmt::Write::write_fmt(
                                        &mut acc,
                                        format_args!("{el0_ident}: {{}}\n")
                                    ).is_ok(),
                                    "ab44c70f"
                                );
                                acc
                            }
                        )
                    );
                    let fields_format_vs_excluding_loc_ts = fields.iter()
                    .filter(|el0| *el0.ident.as_ref().expect("f6f6fb24") != *loc_sc_str)
                    .map(|el0| {
                        let el0_ident = &el0.ident.as_ref().expect("e97b25b9");
                        match macros_helpers::loc_data::LocFieldAttr::try_from(el0).expect("8ff56aeb") {
                            macros_helpers::loc_data::LocFieldAttr::EoToErrString | macros_helpers::loc_data::LocFieldAttr::EoToErrStringSerde => {
                                quote::quote! {
                                    to_err_string::ToErrString::to_err_string(#el0_ident)
                                }
                            }
                            macros_helpers::loc_data::LocFieldAttr::EoLoc => {
                                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts::gen_if_write_is_err_ts(&quote::quote! {acc_52e70d22, "\n {el}"}, &quote::quote! {panic!("c751d54a");});
                                quote::quote! {
                                    #el0_ident.to_string().lines().fold(
                                        #string_ts::new(),
                                        |mut acc_52e70d22, el| {
                                            #if_write_is_err_ts
                                            acc_52e70d22
                                        }
                                    )
                                }
                            }
                            macros_helpers::loc_data::LocFieldAttr::EoVecToErrString | macros_helpers::loc_data::LocFieldAttr::EoVecToErrStringSerde => {
                                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts::gen_if_write_is_err_ts(&quote::quote! {acc_a9ba7521, "\n {el_6e4f53ad}"}, &quote::quote! {panic!("b35ed9f5");});
                                quote::quote! {
                                    #el0_ident.iter().fold(
                                        #string_ts::new(),
                                        |mut acc_ac447c4b, el| {
                                            acc_ac447c4b.push_str(
                                                &to_err_string::ToErrString::to_err_string(el)
                                                .lines()
                                                .fold(
                                                    #string_ts::new(),
                                                    |mut acc_a9ba7521, el_6e4f53ad| {
                                                        #if_write_is_err_ts
                                                        acc_a9ba7521
                                                    }
                                                )
                                            );
                                            acc_ac447c4b
                                        }
                                    )
                                }
                            }
                            macros_helpers::loc_data::LocFieldAttr::EoVecLoc => {
                                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts::gen_if_write_is_err_ts(&quote::quote! {acc_1bbd5ef3, "\n {el_3f2fe01d}"}, &quote::quote! {panic!("4dfdd18d");});
                                quote::quote! {
                                    #el0_ident.iter().fold(
                                        #string_ts::new(),
                                        |mut acc_c5adba93, el_37c46c8a| {
                                            acc_c5adba93.push_str(&el_37c46c8a.to_string().lines().fold(
                                                #string_ts::new(),
                                                |mut acc_1bbd5ef3, el_3f2fe01d| {
                                                    #if_write_is_err_ts
                                                    acc_1bbd5ef3
                                                },
                                            ));
                                            acc_c5adba93
                                        }
                                    )
                                }
                            }
                            macros_helpers::loc_data::LocFieldAttr::EoHashMapKStringVToErrString | macros_helpers::loc_data::LocFieldAttr::EoHashMapKStringVToErrStringSerde => {
                                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts::gen_if_write_is_err_ts(&quote::quote! {acc_06473093, "\n {}: {}", &to_err_string::ToErrString::to_err_string(k), &to_err_string::ToErrString::to_err_string(#v_sc)}, &quote::quote! {panic!("d030580a");});
                                quote::quote! {
                                    #el0_ident.iter().fold(
                                        #string_ts::new(),
                                        |mut acc_06473093, (k, #v_sc)| {
                                            #if_write_is_err_ts
                                            acc_06473093
                                        }
                                    )
                                }
                            }
                            macros_helpers::loc_data::LocFieldAttr::EoHashMapKStringVLoc => {
                                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts::gen_if_write_is_err_ts(
                                    &{
                                        let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts::gen_if_write_is_err_ts(&quote::quote! {acc_addfc699, "\n  {el_8b8f577e}"}, &quote::quote! {panic!("d0492fbf");});
                                        quote::quote! {
                                            acc_a47e1ba7,
                                            "\n {}: {}",
                                            to_err_string::ToErrString::to_err_string(k),
                                            #v_sc.to_string().lines().fold(
                                                #string_ts::new(),
                                                |mut acc_addfc699, el_8b8f577e| {
                                                    #if_write_is_err_ts
                                                    acc_addfc699
                                                }
                                            )
                                        }
                                    },
                                    &quote::quote! {panic!("75f6432a");},
                                );
                                quote::quote! {
                                    #el0_ident.iter().fold(
                                        #string_ts::new(),
                                        |mut acc_a47e1ba7, (k, #v_sc)| {
                                            #if_write_is_err_ts
                                            acc_a47e1ba7
                                        }
                                    )
                                }
                            }
                        }
                    });
                    quote::quote! {
                        Self::#el_ident {
                            #fields_idents_excluding_loc_ts
                            ..
                        } => {
                            format!(
                                #fields_format_excluding_loc_ts,
                                #(#fields_format_vs_excluding_loc_ts),*
                            )
                        }
                    }
                });
                let loc_vrts_ts = data_enum.variants.iter().enumerate().map(|(i, el)| {
                    let el_ident = &el.ident;
                    if i == 0 {
                        quote::quote! {
                            Self::#el_ident {
                                #loc_sc,
                                ..
                            }
                        }
                    } else {
                        quote::quote! {
                            | Self::#el_ident {
                                #loc_sc,
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
                            #(#vrts_ts),*
                        },
                        match self {
                            #(#loc_vrts_ts)*
                            => #loc_sc
                        }
                    )
                }
            };
            let impl_display_for_ident_ts =
                macros_helpers::gen_impl_display_ts::gen_impl_display_ts(
                    &mb_generic_prms_loc_lib_to_err_string_anns_ts,
                    &ident,
                    &mb_generic_prms_ts,
                    &impl_display_h_ts,
                );
            let impl_ident_into_serde_version_ts = {
                let vrts_ts = data_enum.variants.iter().map(|el| {
                    let el_ident = &el.ident;
                    let fields = if let syn::Fields::Named(fields) = &el.fields {
                        &fields.named
                    } else {
                        panic!("238b402b");
                    };
                    let fields_idents_ts = fields.iter().map(|el0| &el0.ident);
                    let fields_ts = fields.iter()
                    .map(|el0| {
                        let el0_ident = &el0.ident.as_ref().expect("9a672ac2");
                        if **el0_ident == *loc_sc_str {
                            quote::quote! {#el0_ident}
                        }
                        else {
                            let gen_field_ts = |ts: &dyn quote::ToTokens|quote::quote! {#el0_ident: {#ts}};
                            match macros_helpers::loc_data::LocFieldAttr::try_from(el0).expect("449c3781") {
                                macros_helpers::loc_data::LocFieldAttr::EoToErrString => gen_field_ts(&quote::quote! {
                                    to_err_string::ToErrString::to_err_string(&#el0_ident).into_inner()
                                }),
                                macros_helpers::loc_data::LocFieldAttr::EoToErrStringSerde | macros_helpers::loc_data::LocFieldAttr::EoVecToErrStringSerde => {
                                    quote::quote! {#el0_ident}
                                }
                                macros_helpers::loc_data::LocFieldAttr::EoLoc => gen_field_ts(&quote::quote! {
                                    #el0_ident.into_serde_version()
                                }),
                                macros_helpers::loc_data::LocFieldAttr::EoVecToErrString => gen_field_ts(&quote::quote! {
                                    #el0_ident.into_iter().map(|el|to_err_string::ToErrString::to_err_string(&el).into_inner()).collect()
                                }),
                                macros_helpers::loc_data::LocFieldAttr::EoVecLoc => gen_field_ts(&quote::quote! {
                                    #el0_ident.into_iter().map(|el|el.into_serde_version()).collect()
                                }),
                                macros_helpers::loc_data::LocFieldAttr::EoHashMapKStringVToErrString => gen_field_ts(&quote::quote! {
                                    #el0_ident.into_iter().map(
                                        |(k, v)|(to_err_string::ToErrString::to_err_string(&k).into_inner(), to_err_string::ToErrString::to_err_string(&v).into_inner())
                                    ).collect()
                                }),
                                macros_helpers::loc_data::LocFieldAttr::EoHashMapKStringVToErrStringSerde => gen_field_ts(&quote::quote! {
                                    #el0_ident.into_iter().map(
                                        |(k, v)|(to_err_string::ToErrString::to_err_string(&k).into_inner(), v)
                                    ).collect()
                                }),
                                macros_helpers::loc_data::LocFieldAttr::EoHashMapKStringVLoc => gen_field_ts(&quote::quote! {
                                    #el0_ident.into_iter().map(
                                        |(k, v)|(to_err_string::ToErrString::to_err_string(&k).into_inner(), v.into_serde_version())
                                    ).collect()
                                }),
                            }
                        }
                    });
                    quote::quote! {
                        Self::#el_ident {
                            #(#fields_idents_ts),*
                        } => #ident_with_serde_ucc::#el_ident {
                            #(#fields_ts),*
                        }
                    }
                });
                gen_impl_ident_into_serde_version_ts(&quote::quote! {#(#vrts_ts),*})
            };
            let enum_ident_with_serde_ts = {
                let vrts_ts = data_enum.variants.iter().map(|vrt| {
                    macros_helpers::loc_data::gen_serde_version_of_named_syn_vrt(
                        macros_helpers::loc_data::SynVariantRef::from(vrt),
                    )
                });
                gen_enum_ident_with_serde_ts(&quote::quote! {#(#vrts_ts),*})
            };
            let impl_display_for_ident_with_serde_ts =
                macros_helpers::gen_impl_display_ts::gen_impl_display_ts(
                    &mb_generic_prms_loc_lib_to_err_string_anns_ts,
                    &ident_with_serde_ucc,
                    &mb_generic_prms_ts,
                    &impl_display_h_ts,
                );
            let impl_loc_lib_to_err_string_to_err_string_for_ident_with_serde_ts =
                macros_helpers::gen_impl_to_err_string_ts::gen_impl_to_err_string_ts(
                    &mb_generic_prms_loc_lib_to_err_string_anns_ts,
                    &ident_with_serde_ucc,
                    &mb_generic_prms_ts,
                    &quote::quote! {format!("{self}")},
                );
            quote::quote! {
                #impl_display_for_ident_ts
                #impl_ident_into_serde_version_ts
                #enum_ident_with_serde_ts
                #impl_display_for_ident_with_serde_ts
                #impl_loc_lib_to_err_string_to_err_string_for_ident_with_serde_ts
            }
        }
        SuportedEnumVrt::Unnamed => {
            let display_formatter_unnamed_ts = {
                let vrts_ts = data_enum.variants.iter().map(|el| {
                    let el_ident = &el.ident;
                    quote::quote! {Self::#el_ident(v) => v}
                });
                quote::quote! {match self { #(#vrts_ts),* }}
            };
            let impl_display_for_ident_ts =
                macros_helpers::gen_impl_display_ts::gen_impl_display_ts(
                    &mb_generic_prms_loc_lib_to_err_string_anns_ts,
                    &ident,
                    &mb_generic_prms_ts,
                    &quote::quote! {
                        write!(
                            f,
                            "{}",
                            #display_formatter_unnamed_ts
                        )
                    },
                );
            let impl_ident_into_serde_version_ts = {
                let vrts_ts = data_enum.variants.iter().map(|el| {
                    let el_ident = &el.ident;
                    quote::quote! {
                        Self::#el_ident(v) => #ident_with_serde_ucc::#el_ident(
                            v.#into_serde_version_sc(),
                        )
                    }
                });
                gen_impl_ident_into_serde_version_ts(&quote::quote! {#(#vrts_ts),*})
            };
            let enum_ident_with_serde_ts = {
                let vrts_ts = data_enum.variants.iter().map(|el| {
                    let el_ident = &el.ident;
                    let fields = if let syn::Fields::Unnamed(fields) = &el.fields {
                        &fields.unnamed
                    } else {
                        panic!("5749e920");
                    };
                    let inn_type_with_serde_ts = {
                        format!(
                            "{}{}",
                            {
                                assert!(fields.len() == 1, "d7a6b955");
                                let ft = &fields.iter().next().expect("8a80c36d").ty;
                                quote::quote! {#ft}.to_string()
                            },
                            naming::WithSerdeUcc
                        )
                        .parse::<proc_macro2::TokenStream>()
                        .expect("9ff40f7e")
                    };
                    quote::quote! {#el_ident(#inn_type_with_serde_ts)}
                });
                gen_enum_ident_with_serde_ts(&quote::quote! {#(#vrts_ts),*})
            };
            let impl_display_for_ident_with_serde_ts =
                macros_helpers::gen_impl_display_ts::gen_impl_display_ts(
                    &mb_generic_prms_loc_lib_to_err_string_anns_ts,
                    &ident_with_serde_ucc,
                    &mb_generic_prms_ts,
                    &quote::quote! {
                        write!(
                            f,
                            "{}",
                            #display_formatter_unnamed_ts
                        )
                    },
                );
            //todo mb make a trait?
            quote::quote! {
                #impl_display_for_ident_ts
                #impl_ident_into_serde_version_ts
                #enum_ident_with_serde_ts
                #impl_display_for_ident_with_serde_ts
            }
        }
    };
    let generated = quote::quote! {#tokens};
    // println!("{generated} ");
    // if ident == "" {
    //     macros_helpers::ts_writer::mb_write_ts_into_file(
    //         macros_helpers::ts_writer::ShouldWriteTsIntoFile::True,
    //         "loc",
    //         &generated,
    //         &macros_helpers::ts_writer::FormatWithCargofmt::True,
    //     );
    // }
    generated.into()
}
#[cfg(test)]
mod tests {
    #[test]
    fn adds_loc_to_every_named_variant() {
        let mut item: syn::ItemEnum = syn::parse_quote! {
            enum SampleEr {
                First { value: String },
                Second {},
            }
        };
        super::add_loc_fields(super::SynItemEnumMutRef::from(&mut item)).expect("74c1509e");
        assert_eq!(
            quote::quote! {#item}.to_string(),
            quote::quote! {
                enum SampleEr {
                    First { value: String, loc: loc_lib::loc::Loc },
                    Second { loc: loc_lib::loc::Loc },
                }
            }
            .to_string()
        );
    }
    #[test]
    fn rejects_existing_loc_field() {
        let mut item: syn::ItemEnum = syn::parse_quote! {
            enum SampleEr { First { loc: loc_lib::loc::Loc } }
        };
        let er =
            super::add_loc_fields(super::SynItemEnumMutRef::from(&mut item)).expect_err("371082fa");
        assert_eq!(
            er.to_string(),
            "errors_with_loc variant already has a loc field"
        );
    }
    #[test]
    fn rejects_unnamed_variant() {
        let mut item: syn::ItemEnum = syn::parse_quote! {
            enum SampleEr { First(String) }
        };
        let er =
            super::add_loc_fields(super::SynItemEnumMutRef::from(&mut item)).expect_err("982f4d17");
        assert_eq!(
            er.to_string(),
            "errors_with_loc supports only variants with named fields"
        );
    }
}
