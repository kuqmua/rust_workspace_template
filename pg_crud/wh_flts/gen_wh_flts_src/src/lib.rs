#[derive(Debug, Clone, Copy)]
pub struct ProcMacro2GenWhFltsInput<'input_lt>(&'input_lt proc_macro2::TokenStream);
impl<'input_lt> From<&'input_lt proc_macro2::TokenStream> for ProcMacro2GenWhFltsInput<'input_lt> {
    fn from(value: &'input_lt proc_macro2::TokenStream) -> Self {
        Self(value)
    }
}
impl AsRef<proc_macro2::TokenStream> for ProcMacro2GenWhFltsInput<'_> {
    fn as_ref(&self) -> &proc_macro2::TokenStream {
        self.0
    }
}
#[derive(Debug)]
pub struct ProcMacro2GenWhFltsTs(proc_macro2::TokenStream);
impl From<proc_macro2::TokenStream> for ProcMacro2GenWhFltsTs {
    fn from(value: proc_macro2::TokenStream) -> Self {
        Self(value)
    }
}
impl From<ProcMacro2GenWhFltsTs> for proc_macro2::TokenStream {
    fn from(value: ProcMacro2GenWhFltsTs) -> Self {
        value.0
    }
}
impl std::fmt::Display for ProcMacro2GenWhFltsTs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[must_use]
pub fn gen_wh_flts(input_ts: ProcMacro2GenWhFltsInput<'_>) -> ProcMacro2GenWhFltsTs {
    #[derive(Clone, optml::Optml)]
    enum Generic {
        False,
        True {
            mb_extra_traits_ts: Option<proc_macro2::TokenStream>,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Clone, optml::Optml)]
    enum PgTypePtrn {
        Stdrt,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum PgTypeKind {
        Stdrt,
    }
    impl PgTypeKind {
        const fn format_argument(&self) -> &'static str {
            match &self {
                Self::Stdrt => "",
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde::Deserialize, optml::Optml)]
    struct GenWhFltsConfig {
        pg_types_write_into_file: macros_helpers::ts_writer::ShouldWriteTsIntoFile,
        whole_write_into_file: macros_helpers::ts_writer::ShouldWriteTsIntoFile,
    }
    panic_loc::panic_loc();
    let gen_wh_flts_config =
        match serde_json::from_str::<GenWhFltsConfig>(&input_ts.as_ref().to_string()) {
            Ok(v) => v,
            Err(er) => {
                let msg = format!("failed to parse GenWhFltsConfig: {er}");
                return ProcMacro2GenWhFltsTs::from(quote::quote! { compile_error!(#msg); });
            }
        };
    let col_sc = naming::ColSc;
    let er_sc = naming::ErSc;
    let incr_sc = naming::IncrSc;
    let pub_sc = naming::PubSc;
    let query_sc = naming::QuerySc;
    let self_sc = naming::SelfSc;
    let v_sc = naming::VSc;
    let pg_crud_cmn_dflt_some_one_el = token_patterns::PgCrudCmnDfltSomeOneEl;
    let pg_crud_cmn_dflt_some_one_el_call = token_patterns::PgCrudCmnDfltSomeOneElCall;
    let import = pg_crud_macros_cmn::Import::PgCrudCmn;
    let t_ts = quote::quote! {T};
    let t_ann_generic_ts = quote::quote! {<#t_ts>};
    let proc_macro2_ts_new = proc_macro2::TokenStream::new();
    let pub_v_t_ts = quote::quote! {pub #v_sc: T};
    let v_dflt_some_one_el_ts = quote::quote! {
        #v_sc: #pg_crud_cmn_dflt_some_one_el_call
    };
    let gen_struct_ts = |flt_init_with_try_new_result_is_ok,
                         generic: &Generic,
                         ident: &dyn quote::ToTokens,
                         struct_extra_fields_ts: &dyn quote::ToTokens| {
        let mb_pub_ts: &dyn quote::ToTokens = if flt_init_with_try_new_result_is_ok {
            &proc_macro2_ts_new
        } else {
            &pub_sc
        };
        macros_helpers::derive_ts_builder::DTsBuilder::new()
            .make_pub()
            .d_debug()
            .d_clone()
            .d_partial_eq()
            .d_serde_serialize()
            .d_serde_deserialize_if(if flt_init_with_try_new_result_is_ok {
                macros_helpers::derive_ts_builder::DSerdeDeserialize::False
            } else {
                macros_helpers::derive_ts_builder::DSerdeDeserialize::True
            })
            .d_schemars_json_schema()
            .build_struct(
                &proc_macro2::TokenStream::new(),
                &ident,
                &match &generic {
                    Generic::False => proc_macro2_ts_new.clone(),
                    Generic::True { mb_extra_traits_ts } => mb_extra_traits_ts
                        .as_ref()
                        .map_or_else(|| quote::quote! {<#t_ts>}, |v| quote::quote! {<#t_ts: #v>}),
                },
                &quote::quote! {{
                    #mb_pub_ts oprtr: #import::Oprtr,
                    #struct_extra_fields_ts
                }},
            )
    };
    let gen_impl_dflt_some_one_el_ts =
        |generic: &Generic, ident: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
            pg_crud_macros_cmn::gen_impl_dflt_some_one_el_ts(
                &match &generic {
                    Generic::False => proc_macro2::TokenStream::new(),
                    Generic::True { mb_extra_traits_ts } => {
                        mb_extra_traits_ts.as_ref().map_or_else(
                            || quote::quote! {<T: #pg_crud_cmn_dflt_some_one_el>},
                            |v| quote::quote! {<T: #v + #pg_crud_cmn_dflt_some_one_el>},
                        )
                    }
                },
                &pg_crud_macros_cmn::Import::PgCrudCmn,
                &ident,
                match &generic {
                    Generic::False => &proc_macro2_ts_new,
                    Generic::True { .. } => &t_ann_generic_ts,
                },
                &quote::quote! {
                    Self {
                        oprtr: #pg_crud_cmn_dflt_some_one_el_call,
                        #ts
                    }
                },
            )
        };
    let gen_impl_pg_type_wh_flt_ts =
        |generic: &Generic,
         ident: &dyn quote::ToTokens,
         incr_prm_undrscr: &pg_crud_macros_cmn::IncrPrmUndrscr,
         add_oprtr_undrscr: &pg_crud_macros_cmn::AddOprtrUndrscr,
         qp_ts: &dyn quote::ToTokens,
         is_qb_mut: &pg_crud_macros_cmn::IsQbMut,
         qb_ts: &dyn quote::ToTokens| {
            pg_crud_macros_cmn::impl_pg_type_wh_flt_for_ident_ts(
                &{
                    let mb_t_extra_traits_for_pg_type_wh_flt_ts: &dyn quote::ToTokens =
                        match &generic {
                            Generic::False => &proc_macro2_ts_new,
                            Generic::True { mb_extra_traits_ts } => {
                                let send_and_lt_ts = quote::quote! {Send + 'lt};
                                let ts = mb_extra_traits_ts.as_ref().map_or_else(
                                    || send_and_lt_ts.clone(),
                                    |v| quote::quote! {#v + #send_and_lt_ts},
                                );
                                &quote::quote! {, T: #ts}
                            }
                        };
                    quote::quote! {<'lt #mb_t_extra_traits_for_pg_type_wh_flt_ts>}
                },
                &ident,
                &match &generic {
                    Generic::False => &proc_macro2_ts_new,
                    Generic::True { .. } => &t_ann_generic_ts,
                },
                incr_prm_undrscr,
                &pg_crud_macros_cmn::ColPrmUndrscr::False,
                add_oprtr_undrscr,
                &qp_ts,
                is_qb_mut,
                &qb_ts,
                &pg_crud_macros_cmn::Import::PgCrudCmn,
            )
        };
    let add_rgx_case_and_v_dcl_ts = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            #ts
            pub rgx_case: RgxCase,
            pub #v_sc: RegexRgx
        }
    };
    let add_rgx_case_and_v_dflt_init_ts = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            #ts
            rgx_case: #pg_crud_cmn_dflt_some_one_el_call,
            #v_dflt_some_one_el_ts
        }
    };
    let gen_match_incr_checked_add_one_init_ts = |ts: &dyn quote::ToTokens| {
        let match_ts = pg_crud_macros_cmn::ts_helpers::gen_match_ok_or_return_err_ts(
            &quote::quote! {#import::incr_checked_add_one_returning_incr(#incr_sc)},
            &quote::quote! {v_25d59e01},
        );
        quote::quote! {
            let #ts = #match_ts;
        }
    };
    let v_match_incr_checked_add_one_init_ts = gen_match_incr_checked_add_one_init_ts(&v_sc);
    let self_oprtr_to_qp_ts = quote::quote! {&#self_sc.oprtr.to_qp(add_oprtr),};
    let gen_rgx_qp_format_ts =
        |v: &dyn std::fmt::Display,
         mb_dims_ies_init_ts: &dyn quote::ToTokens,
         mb_extra_prms_ts: &dyn quote::ToTokens| {
            let format_ts = gen_quotes::dq_ts(&v);
            quote::quote! {
                #mb_dims_ies_init_ts
                #v_match_incr_checked_add_one_init_ts
                Ok(#import::QpFragment::try_from(format!(
                    #format_ts,
                    #self_oprtr_to_qp_ts
                    #col_sc,
                    #mb_extra_prms_ts
                    #self_sc.rgx_case.postgreql_syntax(),
                    #v_sc
                ))?)
            }
        };
    let if_let_err_query_try_bind_self_v_to_string_ts = quote::quote! {
        if let Err(#er_sc) = #query_sc.as_mut().try_bind(#self_sc.#v_sc.to_string()) {
            return Err(match #import::PgQueryBindEr::try_from(#er_sc.to_string()) {
                Ok(v) => v,
                Err(bind_er) => #import::PgQueryBindEr::from(bind_er),
            });
        }
        Ok(#query_sc)
    };
    let if_let_err_query_try_bind_self_v_ts = quote::quote! {
        if let Err(#er_sc) = #query_sc.as_mut().try_bind(#self_sc.#v_sc) {
            return Err(match #import::PgQueryBindEr::try_from(#er_sc.to_string()) {
                Ok(v) => v,
                Err(bind_er) => #import::PgQueryBindEr::from(bind_er),
            });
        }
    };
    let qb_one_v_ts = quote::quote! {
        #if_let_err_query_try_bind_self_v_ts
        Ok(#query_sc)
    };
    let generic_false = Generic::False;
    let generic_true_debug_partial_eq_partial_ord_clone_type_encode = Generic::True {
        mb_extra_traits_ts: Some(quote::quote! {
            std::fmt::Debug
            + PartialEq
            + PartialOrd
            + Clone
            + sqlx::Type<sqlx::Postgres>
            + for<'__> sqlx::Encode<'__, sqlx::Postgres>
        }),
    };
    let pub_v_btwn_t_ts = quote::quote! {pub #v_sc: Btwn<T>};
    let gen_match_qb_ts = |field_ts: &dyn quote::ToTokens| {
        pg_crud_macros_cmn::ts_helpers::gen_match_ok_assign_or_return_err_ts(
            &quote::quote! {#field_ts.qb(#query_sc)},
            &query_sc,
            &quote::quote! {v_f6d31bdd},
        )
    };
    let query_self_v_qb_ts = {
        let ts = gen_match_qb_ts(&quote::quote! {#self_sc.#v_sc});
        quote::quote! {
            #ts
            Ok(#query_sc)
        }
    };
    let pg_type_ptrn_stdrt = PgTypePtrn::Stdrt;
    let gen_ident_match_field_fn_ok_v_return_err_ts =
        |ident_ts: &dyn quote::ToTokens,
         field_ts: &dyn quote::ToTokens,
         fn_ts: &dyn quote::ToTokens| {
            let match_ts = pg_crud_macros_cmn::ts_helpers::gen_match_ok_or_return_err_ts(
                &quote::quote! {self.#field_ts.#fn_ts(#incr_sc, #col_sc, add_oprtr)},
                &quote::quote! {v_0a22ee9a},
            );
            quote::quote! {
                let #ident_ts = #match_ts;
            }
        };
    let v_match_self_v_qp_init_ts =
        gen_ident_match_field_fn_ok_v_return_err_ts(&v_sc, &v_sc, &quote::quote! {qp});
    let gen_mb_dims_dcl_pub_v_t_ts = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            #ts
            #pub_v_t_ts
        }
    };
    let gen_mb_dims_dflt_init_v_dflt_ts = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            #ts
            #v_dflt_some_one_el_ts
        }
    };
    let gen_two_ts = |mb_dims_qb_ts: &dyn quote::ToTokens, trailing_ts: &dyn quote::ToTokens| {
        quote::quote! {
            #mb_dims_qb_ts
            #trailing_ts
        }
    };
    let is_qb_mut_true = pg_crud_macros_cmn::IsQbMut::True;
    let is_qb_mut_false = pg_crud_macros_cmn::IsQbMut::False;
    let gen_qp_format_with_v_ts =
        |mb_dims_ies_init_ts: &dyn quote::ToTokens,
         format_ts: &dyn quote::ToTokens,
         mb_extra_prms_ts: &dyn quote::ToTokens| {
            quote::quote! {
                #mb_dims_ies_init_ts
                #v_match_incr_checked_add_one_init_ts
                Ok(#import::QpFragment::try_from(format!(
                    #format_ts,
                    #self_oprtr_to_qp_ts
                    #col_sc,
                    #mb_extra_prms_ts
                    #v_sc
                ))?)
            }
        };
    let gen_pg_type_dims_helpers = |pg_type_ptrn: &PgTypePtrn| match pg_type_ptrn {
        PgTypePtrn::Stdrt => (
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
            PgTypeKind::Stdrt,
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
        ),
    };
    let pg_type_ts = {
        let gen_flts_ts = |flt: &pg_crud_macros_cmn::flts::PgTypeFlt| {
            let ident = naming::prm::PgTypeWhSelfUcc::from_display(&flt);
            let (
                generic,
                struct_extra_fields_ts,
                impl_dflt_some_one_el_extra_fields_ts,
                incr_prm_undrscr,
                qp_ts,
                is_qb_mut,
                qb_ts,
            ) = {
                let sqlx_type_pg_encode_ts = quote::quote! {sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>};
                let generic_true_type_encode = Generic::True {
                    mb_extra_traits_ts: Some(sqlx_type_pg_encode_ts.clone()),
                };
                let gen_pg_type_dims_helpers_pg_type =
                    |pg_type_ptrn: &PgTypePtrn| gen_pg_type_dims_helpers(pg_type_ptrn);
                let gen_cmp_flt_ts =
                    |pg_type_ptrn: &PgTypePtrn,
                     gen_format_h_str: &dyn Fn(&PgTypeKind) -> String| {
                        let (
                            mb_dims_dcl_ts,
                            mb_dims_dflt_init_ts,
                            mb_dims_ies_init_ts,
                            pg_type_kind,
                            mb_extra_prms_ts,
                            mb_dims_qb_ts,
                        ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                        (
                            generic_true_type_encode.clone(),
                            gen_mb_dims_dcl_pub_v_t_ts(&mb_dims_dcl_ts),
                            gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                            pg_crud_macros_cmn::IncrPrmUndrscr::False,
                            gen_qp_format_with_v_ts(
                                &mb_dims_ies_init_ts,
                                &gen_quotes::dq_ts(&gen_format_h_str(&pg_type_kind)),
                                &mb_extra_prms_ts,
                            ),
                            is_qb_mut_true,
                            gen_two_ts(&mb_dims_qb_ts, &qb_one_v_ts),
                        )
                    };
                let gen_oprtr_cmp_flt_ts =
                    |pg_type_ptrn: &PgTypePtrn, oprtr: &dyn std::fmt::Display| {
                        gen_cmp_flt_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                            format!("{{}}({{}}{} {oprtr} ${{}})", pg_type_kind.format_argument())
                        })
                    };
                let gen_greater_than_ts =
                    |pg_type_ptrn: &PgTypePtrn| gen_oprtr_cmp_flt_ts(pg_type_ptrn, &">");
                let gen_btwn_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_prms_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        generic_true_debug_partial_eq_partial_ord_clone_type_encode.clone(),
                        quote::quote! {
                            #mb_dims_dcl_ts
                            #pub_v_btwn_t_ts
                        },
                        gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                        pg_crud_macros_cmn::IncrPrmUndrscr::False,
                        {
                            let format_ts = gen_quotes::dq_ts(&format!(
                                "{{}}({{}}{} {{}})",
                                pg_type_kind.format_argument()
                            ));
                            quote::quote! {
                                #mb_dims_ies_init_ts
                                #v_match_self_v_qp_init_ts
                                Ok(#import::QpFragment::try_from(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #col_sc,
                                    #mb_extra_prms_ts
                                    #v_sc
                                ))?)
                            }
                        },
                        is_qb_mut_true,
                        quote::quote! {
                            #mb_dims_qb_ts
                            #query_self_v_qb_ts
                        },
                    )
                };
                let gen_in_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_prms_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        Generic::True {
                            mb_extra_traits_ts: Some(
                                quote::quote! {std::fmt::Debug + PartialEq + Clone + #sqlx_type_pg_encode_ts},
                            ),
                        },
                        quote::quote! {
                            #mb_dims_dcl_ts
                            pub #v_sc: PgTypeNotEmptyUnqVec<T>
                        },
                        gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                        pg_crud_macros_cmn::IncrPrmUndrscr::False,
                        {
                            let format_ts = gen_quotes::dq_ts(&format!(
                                "{{}}({{}}{} in ({{}}))",
                                pg_type_kind.format_argument()
                            ));
                            let if_write_is_err_ts =
                                macros_helpers::gen_if_write_is_err_ts::gen_if_write_is_err_ts(
                                    &quote::quote! {acc, "${v_daedba9c},"},
                                    &quote::quote! {return Err(#import::QpEr::WriteIntoBuffer { loc: loc_macros::loc!() });},
                                );
                            quote::quote! {
                                #mb_dims_ies_init_ts
                                let #v_sc = {
                                    let mut acc = String::default();
                                    for _ in #self_sc.#v_sc.to_vec() {
                                        match #import::incr_checked_add_one_returning_incr(#incr_sc) {
                                            Ok(v_daedba9c) => {
                                                #if_write_is_err_ts
                                            },
                                            Err(#er_sc) => {
                                                return Err(#er_sc);
                                            },
                                        }
                                    }
                                    let _: Option<char> = acc.pop();
                                    acc
                                };
                                Ok(#import::QpFragment::try_from(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #col_sc,
                                    #mb_extra_prms_ts
                                    #v_sc
                                ))?)
                            }
                        },
                        is_qb_mut_true,
                        quote::quote! {
                            #mb_dims_qb_ts
                            for el in #self_sc.#v_sc.into_vec() {
                                if let Err(#er_sc) = #query_sc.as_mut().try_bind(el) {
                                    return Err(match #import::PgQueryBindEr::try_from(#er_sc.to_string()) {
                                        Ok(v) => v,
                                        Err(bind_er) => #import::PgQueryBindEr::from(bind_er),
                                    });
                                }
                            }
                            Ok(#query_sc)
                        },
                    )
                };
                let gen_rgx_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_prms_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        generic_false.clone(),
                        add_rgx_case_and_v_dcl_ts(&mb_dims_dcl_ts),
                        add_rgx_case_and_v_dflt_init_ts(&mb_dims_dflt_init_ts),
                        pg_crud_macros_cmn::IncrPrmUndrscr::False,
                        gen_rgx_qp_format_ts(
                            &format!("{{}}({{}}{} {{}} ${{}})", pg_type_kind.format_argument()),
                            &mb_dims_ies_init_ts,
                            &mb_extra_prms_ts,
                        ),
                        is_qb_mut_true,
                        gen_two_ts(
                            &mb_dims_qb_ts,
                            &if_let_err_query_try_bind_self_v_to_string_ts,
                        ),
                    )
                };
                let gen_pg_syntax_flt_ts =
                    |pg_type_ptrn: &PgTypePtrn, pg_syntax: &dyn std::fmt::Display| {
                        let (
                            mb_dims_dcl_ts,
                            mb_dims_dflt_init_ts,
                            mb_dims_ies_init_ts,
                            pg_type_kind,
                            mb_extra_prms_ts,
                            mb_dims_qb_ts,
                        ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                        (
                            generic_false.clone(),
                            mb_dims_dcl_ts,
                            mb_dims_dflt_init_ts,
                            pg_crud_macros_cmn::IncrPrmUndrscr::True,
                            {
                                let format_ts = gen_quotes::dq_ts(&format!(
                                    "{{}}({{}}{} {pg_syntax})",
                                    pg_type_kind.format_argument()
                                ));
                                quote::quote! {
                                    #mb_dims_ies_init_ts
                                    Ok(#import::QpFragment::try_from(format!(
                                        #format_ts,
                                        #self_oprtr_to_qp_ts
                                        #col_sc,
                                        #mb_extra_prms_ts
                                    ))?)
                                }
                            },
                            is_qb_mut_false,
                            quote::quote! {
                                #mb_dims_qb_ts
                                Ok(#query_sc)
                            },
                        )
                    };
                let gen_eq_to_encoded_string_representation_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_prms_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        generic_false.clone(),
                        quote::quote! {
                            #mb_dims_dcl_ts
                            pub encode_format: EncodeFormat,
                            pub encoded_string_representation: String,
                        },
                        quote::quote! {
                            #mb_dims_dflt_init_ts
                            encode_format: #pg_crud_cmn_dflt_some_one_el_call,
                            encoded_string_representation: String::default()
                        },
                        pg_crud_macros_cmn::IncrPrmUndrscr::False,
                        {
                            let format_ts = gen_quotes::dq_ts(&format!(
                                "{{}}(encode({{}}{}, '{{}}') = ${{}})",
                                pg_type_kind.format_argument()
                            ));
                            quote::quote! {
                                #mb_dims_ies_init_ts
                                #v_match_incr_checked_add_one_init_ts
                                Ok(#import::QpFragment::try_from(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #col_sc,
                                    #mb_extra_prms_ts
                                    &#self_sc.encode_format,
                                    #v_sc
                                ))?)
                            }
                        },
                        is_qb_mut_true,
                        quote::quote! {
                            #mb_dims_qb_ts
                            if let Err(#er_sc) = #query_sc.as_mut().try_bind(self.encoded_string_representation) {
                                return Err(match #import::PgQueryBindEr::try_from(#er_sc.to_string()) {
                                    Ok(v) => v,
                                    Err(bind_er) => #import::PgQueryBindEr::from(bind_er),
                                });
                            }
                            Ok(#query_sc)
                        },
                    )
                };
                let gen_range_bound_cmp_flt_ts = |pg_type_ptrn: &PgTypePtrn, bound_fn, oprtr| {
                    gen_cmp_flt_ts(pg_type_ptrn, &|pg_type_kind: &PgTypeKind| {
                        format!(
                            "{{}}({bound_fn}({{}}{}) {oprtr} ${{}})",
                            pg_type_kind.format_argument()
                        )
                    })
                };
                let gen_range_len_ts = |pg_type_ptrn: &PgTypePtrn| {
                    let (
                        mb_dims_dcl_ts,
                        mb_dims_dflt_init_ts,
                        mb_dims_ies_init_ts,
                        pg_type_kind,
                        mb_extra_prms_ts,
                        mb_dims_qb_ts,
                    ) = gen_pg_type_dims_helpers_pg_type(pg_type_ptrn);
                    (
                        Generic::False,
                        quote::quote! {
                            #mb_dims_dcl_ts
                            pub #v_sc: #import::NotZeroUnsignedPartOfI32
                        },
                        gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                        pg_crud_macros_cmn::IncrPrmUndrscr::False,
                        gen_qp_format_with_v_ts(
                            &mb_dims_ies_init_ts,
                            &gen_quotes::dq_ts(&format!(
                                "{{}}(upper({{}}{}) - lower({{}}{}) = ${{}})",
                                pg_type_kind.format_argument(),
                                pg_type_kind.format_argument(),
                            )),
                            &quote::quote! {
                                #mb_extra_prms_ts
                                #col_sc,
                            },
                        ),
                        is_qb_mut_true,
                        quote::quote! {
                            #mb_dims_qb_ts
                            #qb_one_v_ts
                        },
                    )
                };
                let gen_eq_oprtr_qp_ts =
                    |mb_dims_ies_init_ts: &dyn quote::ToTokens, format_ts: &dyn quote::ToTokens| {
                        quote::quote! {
                            #mb_dims_ies_init_ts
                            let oprtr = <T as #import::PgTypeEqOprtr>::oprtr(&#self_sc.#v_sc);
                            let oprtr_query_str = oprtr.to_query_str();
                            Ok(#import::QpFragment::try_from(format!(
                                #format_ts,
                                #self_oprtr_to_qp_ts
                                #col_sc,
                                match oprtr {
                                    #import::EqOprtr::Eq => {
                                        #v_match_incr_checked_add_one_init_ts
                                        format!("{oprtr_query_str} ${v}")
                                    },
                                    #import::EqOprtr::IsNull => oprtr_query_str.to_string(),
                                }
                            ))?)
                        }
                    };
                let gen_eq_oprtr_qb_ts = |ts: &dyn quote::ToTokens| {
                    quote::quote! {
                        #ts
                        if matches!(&<T as #import::PgTypeEqOprtr>::oprtr(&#self_sc.#v_sc), #import::EqOprtr::Eq)
                            && let Err(#er_sc) = #query_sc.as_mut().try_bind(#self_sc.#v_sc)
                        {
                            return Err(match #import::PgQueryBindEr::try_from(#er_sc.to_string()) {
                                Ok(v) => v,
                                Err(bind_er) => #import::PgQueryBindEr::from(bind_er),
                            });
                        }
                        Ok(#query_sc)
                    }
                };
                match &flt {
                    pg_crud_macros_cmn::flts::PgTypeFlt::Eq { .. } => {
                        let (
                            mb_dims_dcl_ts,
                            mb_dims_dflt_init_ts,
                            mb_dims_ies_init_ts,
                            _,
                            _,
                            mb_dims_qb_ts,
                        ) = gen_pg_type_dims_helpers_pg_type(&pg_type_ptrn_stdrt);
                        (
                            Generic::True {
                                mb_extra_traits_ts: Some(
                                    quote::quote! {#sqlx_type_pg_encode_ts + #import::PgTypeEqOprtr},
                                ),
                            },
                            gen_mb_dims_dcl_pub_v_t_ts(&mb_dims_dcl_ts),
                            gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                            pg_crud_macros_cmn::IncrPrmUndrscr::False,
                            gen_eq_oprtr_qp_ts(&mb_dims_ies_init_ts, &quote::quote! {"{}({} {})"}),
                            is_qb_mut_true,
                            gen_eq_oprtr_qb_ts(&mb_dims_qb_ts),
                        )
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::GreaterThan { .. } => {
                        gen_greater_than_ts(&pg_type_ptrn_stdrt)
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::Btwn { .. } => gen_btwn_ts(&pg_type_ptrn_stdrt),
                    pg_crud_macros_cmn::flts::PgTypeFlt::In { .. } => gen_in_ts(&pg_type_ptrn_stdrt),
                    pg_crud_macros_cmn::flts::PgTypeFlt::Rgx => gen_rgx_ts(&pg_type_ptrn_stdrt),
                    pg_crud_macros_cmn::flts::PgTypeFlt::Before { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"<")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::CrntDate => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"= current_date")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::GreaterThanCrntDate => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"> current_date")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::CrntTimestamp => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"= current_timestamp")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::GreaterThanCrntTimestamp => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"> current_timestamp")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::CrntTime => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"= current_time")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::GreaterThanCrntTime => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"> current_time")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::EqToEncodedStringRepresentation => {
                        gen_eq_to_encoded_string_representation_ts(&pg_type_ptrn_stdrt)
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::FindRangesWithinGivenRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"<@")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::FindRangesThatFullyContainTheGivenRange {
                        ..
                    } => gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"@>"),
                    pg_crud_macros_cmn::flts::PgTypeFlt::StrictlyToLeftOfRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"&<")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::StrictlyToRightOfRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"&>")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::IncludedLowerBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_stdrt, "lower", "=")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::ExcludedUpperBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_stdrt, "upper", "=")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::GreaterThanIncludedLowerBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_stdrt, "lower", ">")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::GreaterThanExcludedUpperBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_stdrt, "upper", ">")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::OverlapWithRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"&&")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::AdjacentWithRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"-|-")
                    }
                    pg_crud_macros_cmn::flts::PgTypeFlt::RangeLen => {
                        gen_range_len_ts(&pg_type_ptrn_stdrt)
                    }
                }
            };
            let struct_ts = gen_struct_ts(false, &generic, &ident, &struct_extra_fields_ts);
            let impl_dflt_some_one_el_ts = gen_impl_dflt_some_one_el_ts(
                &generic,
                &ident,
                &impl_dflt_some_one_el_extra_fields_ts,
            );
            let impl_pg_type_wh_flt_ts = gen_impl_pg_type_wh_flt_ts(
                &generic,
                &ident,
                &incr_prm_undrscr,
                &pg_crud_macros_cmn::AddOprtrUndrscr::False,
                &qp_ts,
                &is_qb_mut,
                &qb_ts,
            );
            let gend = quote::quote! {
                #struct_ts
                #impl_dflt_some_one_el_ts
                #impl_pg_type_wh_flt_ts
            };
            gend
        };
        let flt_arr_ts = <pg_crud_macros_cmn::flts::PgTypeFlt as strum::IntoEnumIterator>::iter()
            .map(|el| gen_flts_ts(&el))
            .collect::<Vec<_>>();
        let gend = quote::quote! {#(#flt_arr_ts)*};
        macros_helpers::ts_writer::mb_write_ts_into_file(
            gen_wh_flts_config.pg_types_write_into_file,
            "gen_wh_flts_pg_types",
            macros_helpers::ts_writer::ProcMacro2TsRef::from(&gend),
            &macros_helpers::ts_writer::FormatWithCargofmt::True,
        );
        gend
    };
    let imports_ts = quote::quote! {
        #[allow(clippy::wildcard_imports)]
        use super::*;
    };
    let gend = pg_crud_macros_cmn::ts_helpers::gen_mod_with_pub_use_ts(
        &quote::format_ident!("gen_wh_flts_mod"),
        &pg_crud_macros_cmn::GeneratedRustTsVec::from(vec![
            macros_helpers::generated_rust_ts::GeneratedRustTs::from(imports_ts),
            macros_helpers::generated_rust_ts::GeneratedRustTs::from(pg_type_ts),
        ]),
    );
    macros_helpers::ts_writer::mb_write_ts_into_file(
        gen_wh_flts_config.whole_write_into_file,
        "gen_wh_flts",
        macros_helpers::ts_writer::ProcMacro2TsRef::from(gend.as_ref()),
        &macros_helpers::ts_writer::FormatWithCargofmt::True,
    );
    ProcMacro2GenWhFltsTs::from(proc_macro2::TokenStream::from(gend))
}
