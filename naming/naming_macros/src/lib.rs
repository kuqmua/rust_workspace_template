const REGEX_VALUE: &str = "^[a-zA-Z0-9]+$";
#[derive(newtype::Newtype)]
#[newtype(to_tokens)]
struct ProcMacro2GeneratedNamingTs(proc_macro2::TokenStream);
#[derive(Clone, Copy)]
struct SynEnumIdentRef<'ident_lt>(&'ident_lt syn::Ident);
#[derive(Clone, Copy)]
struct ProcMacro2VrtMatchingTokensRef<'tokens_lt>(&'tokens_lt [proc_macro2::TokenStream]);
fn gen_impl_to_tokens_ts(
    ts0: &dyn quote::ToTokens,
    ts1: &dyn quote::ToTokens,
) -> ProcMacro2GeneratedNamingTs {
    ProcMacro2GeneratedNamingTs(quote::quote! {
        impl quote::ToTokens for #ts0 {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                #ts1
            }
        }
    })
}
#[proc_macro]
pub fn gen_ucc_and_sc_str_and_ts(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_loc::panic_loc();
    let rgx = regex::Regex::new(REGEX_VALUE).expect("20948d87");
    let ts = serde_json::from_str::<Vec<Vec<String>>>(&input_ts.to_string())
        .expect("90e5793b")
        .into_iter()
        .map(|el| {
            assert!(el.iter().all(|el0| rgx.is_match(el0)), "faadba8a");
            let parts_len = el.iter().map(String::len).sum::<usize>();
            let phrase_part_ucc_str = el.iter().fold(
                String::with_capacity(parts_len),
                |mut acc, el0| {
                    acc.push_str(&naming_cmn::AsRefStrToUccStr::case(el0));
                    acc
                },
            );
            let phrase_part_sc_str = el.iter().enumerate().fold(
                String::with_capacity(parts_len.saturating_add(el.len().saturating_sub(1usize))),
                |mut acc, (i, el0)| {
                        let el_sc_str = naming_cmn::AsRefStrToScStr::case(el0);
                        if i == 0 {
                            acc.push_str(&el_sc_str);
                        } else {
                            assert!(
                                std::fmt::Write::write_fmt(&mut acc, format_args!("_{el_sc_str}"))
                                    .is_ok(),
                                "ef718915"
                            );
                        }
                        acc
                },
            );
            let phrase_part_ucc_ucc_ts = format!("{phrase_part_ucc_str}Ucc")
                .parse::<proc_macro2::TokenStream>()
                .expect("4ab6a54c");
            let phrase_part_sc_ucc_ts = format!("{phrase_part_ucc_str}Sc")
                .parse::<proc_macro2::TokenStream>()
                .expect("0cc47b2e");
            let (ucc_struct_dcl_ts, sc_struct_dcl_ts) = {
                let gen_ts = |ts: &dyn quote::ToTokens| {
                    quote::quote! {
                        #[derive(Debug, optml::Optml)]
                        pub struct #ts;
                    }
                };
                (
                    gen_ts(&phrase_part_ucc_ucc_ts),
                    gen_ts(&phrase_part_sc_ucc_ts),
                )
            };
            let (impl_display_ucc_ts, impl_display_sc_ts) = {
                let gen_ts = |struct_name_ts: &dyn quote::ToTokens,
                              write_ts: &dyn quote::ToTokens| {
                    quote::quote! {
                        impl std::fmt::Display for #struct_name_ts {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                write!(f, #write_ts)
                            }
                        }
                    }
                };
                (
                    gen_ts(
                        &phrase_part_ucc_ucc_ts,
                        &gen_quotes::dq_ts(&phrase_part_ucc_str),
                    ),
                    gen_ts(
                        &phrase_part_sc_ucc_ts,
                        &gen_quotes::dq_ts(&phrase_part_sc_str),
                    ),
                )
            };
            let (impl_to_tokens_ucc_ts, impl_to_tokens_snake_ts) = {
                let gen_ts = |struct_name_ts: &dyn quote::ToTokens,
                              quote_ts: &dyn quote::ToTokens| {
                    gen_impl_to_tokens_ts(
                        struct_name_ts,
                        &quote::quote! {quote::ToTokens::to_tokens(&quote::quote! {#quote_ts}, tokens);},
                    )
                };
                (
                    gen_ts(
                        &phrase_part_ucc_ucc_ts,
                        &phrase_part_ucc_str
                            .parse::<proc_macro2::TokenStream>()
                            .expect("7cf3ffc0"),
                    ),
                    gen_ts(
                        &phrase_part_sc_ucc_ts,
                        &phrase_part_sc_str
                            .parse::<proc_macro2::TokenStream>()
                            .expect("114a573a"),
                    ),
                )
            };
            quote::quote! {
                #ucc_struct_dcl_ts
                #impl_display_ucc_ts
                #impl_to_tokens_ucc_ts
                #sc_struct_dcl_ts
                #impl_display_sc_ts
                #impl_to_tokens_snake_ts
            }
        });
    let generated = quote::quote! {#(#ts)*};
    // println!("{generated}");
    generated.into()
}
#[proc_macro]
pub fn gen_self_ucc_and_sc_str_and_ts(
    input_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_loc::panic_loc();
    let rgx = regex::Regex::new(REGEX_VALUE).expect("cba1b5fb");
    let ts = serde_json::from_str::<Vec<Vec<String>>>(&input_ts.to_string()).expect("9d6a20af").into_iter().map(|el| {
        assert!(el.iter().all(|el0| rgx.is_match(el0)), "4a12d90f");
        let self_match_name = "self";
        {
            let is_self_exists_and_only_one = el.iter().any(|el0| el0 == self_match_name);
            assert!(is_self_exists_and_only_one, "5680dd63");
        };
        let (els_concat_v_ucc_dq_ts, els_concat_v_sc_dq_ts, struct_ucc_ucc_ts, struct_sc_token_ucc_ts, trait_ucc_ucc_ts, trait_sc_token_ucc_ts) = {
            let ucc_ucc_str = "Ucc";
            let sc_ucc_str = "Sc";
            let parts_len = el.iter().map(String::len).sum::<usize>();
            let els_concat_ucc_str = el.iter().fold(String::with_capacity(parts_len), |mut acc, el0| {
                acc.push_str(&naming_cmn::AsRefStrToUccStr::case(el0));
                acc
            });
            let els_concat_v_ucc_dq_ts = gen_quotes::dq_ts(&el.iter().fold(String::with_capacity(parts_len), |mut acc, el0| {
                if el0 == "self" {
                    acc.push_str("{v}");
                } else {
                    acc.push_str(&naming_cmn::AsRefStrToUccStr::case(el0));
                }
                acc
            }));
            let els_concat_v_sc_dq_ts = gen_quotes::dq_ts(&{
                let mut acc = el.iter().fold(String::with_capacity(parts_len.saturating_add(el.len())), |mut acc, el0| {
                    let symbol = '_';
                    if el0 == "self" {
                        assert!(std::fmt::Write::write_fmt(&mut acc, format_args!("{{v}}{symbol}")).is_ok(), "6a02a2ff");
                    } else {
                        assert!(std::fmt::Write::write_fmt(&mut acc, format_args!("{}{symbol}", naming_cmn::AsRefStrToScStr::case(el0))).is_ok(), "d915980a");
                    }
                    acc
                });
                let _: Option<char> = acc.pop();
                acc
            });
            let struct_ucc_ucc_ts = format!("{els_concat_ucc_str}{ucc_ucc_str}").parse::<proc_macro2::TokenStream>().expect("82f4ac08");
            let struct_sc_token_ucc_ts = format!("{els_concat_ucc_str}{sc_ucc_str}").parse::<proc_macro2::TokenStream>().expect("21044eba");
            let (trait_ucc_ucc_ts, trait_sc_token_ucc_ts) = {
                let trait_ucc_str = "Trait";
                let trait_ucc_ucc_ts = format!("{els_concat_ucc_str}{ucc_ucc_str}{trait_ucc_str}").parse::<proc_macro2::TokenStream>().expect("1066857a");
                let trait_sc_token_ucc_ts = format!("{els_concat_ucc_str}{sc_ucc_str}{trait_ucc_str}").parse::<proc_macro2::TokenStream>().expect("8db74cfd");
                (trait_ucc_ucc_ts, trait_sc_token_ucc_ts)
            };
            (
                els_concat_v_ucc_dq_ts,
                els_concat_v_sc_dq_ts,
                struct_ucc_ucc_ts,
                struct_sc_token_ucc_ts,
                trait_ucc_ucc_ts,
                trait_sc_token_ucc_ts,
            )
        };
        let gen_struct_ts = |els_concat_v_case_dq_ts: &dyn quote::ToTokens, is_ucc: bool, trait_ident_ts: &dyn quote::ToTokens| {
            let struct_ident_ts = if is_ucc {
                quote::quote! {#struct_ucc_ucc_ts}
            } else {
                quote::quote! {#struct_sc_token_ucc_ts}
            };
            let casing_ts = {
                let ts = if is_ucc {
                    quote::quote! {AsRefStrToUccStr::case}
                } else {
                    quote::quote! {AsRefStrToScStr::case}
                };
                quote::quote! {naming_cmn::#ts}
            };
            let impl_to_tokens_ts = gen_impl_to_tokens_ts(
                &struct_ident_ts,
                &quote::quote! {quote::ToTokens::to_tokens(&self.to_string().parse::<proc_macro2::TokenStream>().expect("71c8d26b"), tokens);}
            );
            quote::quote! {
                #[derive(Debug, optml::Optml)]
                pub struct #struct_ident_ts(String);
                impl #struct_ident_ts {
                    fn wrap(v: &dyn std::fmt::Display) -> Self {
                        Self(Self::format(v))
                    }
                    fn format(v: &dyn std::fmt::Display) -> String {
                        format!(#els_concat_v_case_dq_ts)
                    }
                    pub fn from_display(v: &dyn std::fmt::Display) -> Self {
                        Self::wrap(&#casing_ts(&v.to_string()))
                    }
                    pub fn from_tokens(v: &dyn quote::ToTokens) -> Self {
                        Self::wrap(&#casing_ts(&{
                            let mut tokens = proc_macro2::TokenStream::new();
                            quote::ToTokens::to_tokens(&v, &mut tokens);
                            tokens
                        }.to_string()))
                    }
                    pub fn from_type_last_segment(v: &syn::Type) -> Self {
                        if let syn::Type::Path(type_path) = v {
                            let path_before_len = type_path.path.segments.len().checked_sub(1).expect("e1f5a332");
                            let path_before_capacity = path_before_len.saturating_mul(16usize);
                            let path_before_str = type_path.path.segments.iter().take(path_before_len)
                            .fold(String::with_capacity(path_before_capacity), |mut acc, el| {
                                assert!(
                                    std::fmt::Write::write_fmt(
                                        &mut acc,
                                        format_args!("{}::", el.ident),
                                    ).is_ok(),
                                    "67c90ce9"
                                );
                                acc
                            });
                            let last = type_path.path.segments.iter().last().expect("19f6e1a6");
                            Self(format!("{path_before_str}{}", Self::format(&#casing_ts(&last.ident.to_string()))))
                        }
                        else {
                            panic!("518933f8");
                        }
                    }
                }
                impl std::fmt::Display for #struct_ident_ts {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.0)
                    }
                }
                #impl_to_tokens_ts
                pub trait #trait_ident_ts: std::fmt::Display + quote::ToTokens {}
                impl #trait_ident_ts for #struct_ident_ts {}
            }
        };
        let pub_struct_ucc_ts = gen_struct_ts(&els_concat_v_ucc_dq_ts, true, &trait_ucc_ucc_ts);
        let pub_struct_sc_ts = gen_struct_ts(&els_concat_v_sc_dq_ts, false, &trait_sc_token_ucc_ts);
        quote::quote! {
            #pub_struct_ucc_ts
            #pub_struct_sc_ts
        }
    });
    let generated = quote::quote! {#(#ts)*};
    // println!("{generated}");
    generated.into()
}
fn gen_impl_trait_for_ident_ts(
    name_ts: &dyn quote::ToTokens,
    ident: SynEnumIdentRef<'_>,
    vrts_matching_ts: ProcMacro2VrtMatchingTokensRef<'_>,
) -> ProcMacro2GeneratedNamingTs {
    let string_ts = token_patterns::StringTs;
    let ident_ref = ident.0;
    let vrt_tokens = vrts_matching_ts.0;
    ProcMacro2GeneratedNamingTs(quote::quote! {
        impl naming_cmn::#name_ts for #ident_ref {
            fn case(&self) -> #string_ts {//todo mb write duplicate Trait with &str instead of String
                match self {#(#vrt_tokens),*}
            }
        }
    })
}
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToUccStr)]
pub fn as_ref_str_enum_with_unit_fields_to_ucc_str(
    input_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_loc::panic_loc();
    let di: syn::DeriveInput = syn::parse(input_ts).expect("a8f22481");
    let ident = &di.ident;
    let syn::Data::Enum(data_enum) = di.data else {
        panic!("d26bf85e")
    };
    let string_ts = token_patterns::StringTs;
    let generated = gen_impl_trait_for_ident_ts(
        &quote::quote! {AsRefStrToUccStr},
        SynEnumIdentRef(ident),
        ProcMacro2VrtMatchingTokensRef(
            &data_enum
                .variants
                .iter()
                .map(|el| match el.fields {
                    syn::Fields::Unit => {
                        let el_ident = &el.ident;
                        let el_ident_ucc_dq_ts =
                            gen_quotes::dq_ts(&naming_cmn::ToTokensToUccStr::case(&el_ident));
                        quote::quote! {Self::#el_ident => #string_ts::from(#el_ident_ucc_dq_ts)}
                    }
                    syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                        panic!("4955c50d")
                    }
                })
                .collect::<Vec<proc_macro2::TokenStream>>(),
        ),
    );
    // println!("{generated}");
    generated.0.into()
}
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToScStr)]
pub fn as_ref_str_enum_with_unit_fields_to_sc_str(
    input_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_loc::panic_loc();
    let di: syn::DeriveInput = syn::parse(input_ts).expect("dea5cbcf");
    let ident = &di.ident;
    let syn::Data::Enum(data_enum) = di.data else {
        panic!("ed6efe2e");
    };
    let string_ts = token_patterns::StringTs;
    let generated = gen_impl_trait_for_ident_ts(
        &quote::quote! {AsRefStrToScStr},
        SynEnumIdentRef(ident),
        ProcMacro2VrtMatchingTokensRef(
            &data_enum
                .variants
                .iter()
                .map(|el| match el.fields {
                    syn::Fields::Unit => {
                        let el_ident = &el.ident;
                        let el_ident_sc_dq_ts =
                            gen_quotes::dq_ts(&naming_cmn::ToTokensToScStr::case(&el_ident));
                        quote::quote! {Self::#el_ident => #string_ts::from(#el_ident_sc_dq_ts)}
                    }
                    syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                        panic!("b3ef2657")
                    }
                })
                .collect::<Vec<proc_macro2::TokenStream>>(),
        ),
    );
    // println!("{generated}");
    generated.0.into()
}
#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToUpperScStr)]
pub fn as_ref_str_enum_with_unit_fields_to_upper_sc_str(
    input_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_loc::panic_loc();
    let di: syn::DeriveInput = syn::parse(input_ts).expect("edabbc24");
    let ident = &di.ident;
    let syn::Data::Enum(data_enum) = di.data else {
        panic!("b2263e7e");
    };
    let string_ts = token_patterns::StringTs;
    let generated = gen_impl_trait_for_ident_ts(
        &quote::quote! {AsRefStrToUpperScStr},
        SynEnumIdentRef(ident),
        ProcMacro2VrtMatchingTokensRef(
            &data_enum
                .variants
                .iter()
                .map(|el| match el.fields {
                    syn::Fields::Unit => {
                        let el_ident = &el.ident;
                        let el_ident_sc_dq_ts =
                            gen_quotes::dq_ts(&naming_cmn::ToTokensToUpperScStr::case(&el_ident));
                        quote::quote! {Self::#el_ident => #string_ts::from(#el_ident_sc_dq_ts)}
                    }
                    syn::Fields::Named(_) | syn::Fields::Unnamed(_) => panic!("b6fedcff"),
                })
                .collect::<Vec<proc_macro2::TokenStream>>(),
        ),
    );
    // println!("{generated}");
    generated.0.into()
}
