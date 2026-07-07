pub use gen_quotes::dq_ts;
pub use macros_helpers::{DSerdeDeserialize, DTsBuilder};
pub use macros_helpers::{
    FormatWithCargofmt, ShouldWriteTsIntoFile, gen_if_write_is_err_ts, mb_write_ts_into_file,
};
pub use naming::{ColSc, ErSc, IncrSc, PubSc, QuerySc, SelfSc, VSc, prm::PgTypeWhSelfUcc};
pub use optml::Optml;
pub use panic_loc::panic_loc;
pub use pg_crud_macros_cmn::{
    AddOprtrUndrscr, ColPrmUndrscr, Import, IncrPrmUndrscr, IsQbMut, PgTypeFlt,
    gen_impl_dflt_some_one_el_ts, gen_match_ok_assign_or_return_err_ts,
    gen_match_ok_or_return_err_ts, impl_pg_type_wh_flt_for_ident_ts,
};
pub use quote::{ToTokens, quote};
pub use serde_json::from_str;
pub use std::fmt::Display;
pub use strum::IntoEnumIterator;
pub use token_patterns::{PgCrudCmnDfltSomeOneEl, PgCrudCmnDfltSomeOneElCall};
type Ts2 = proc_macro2::TokenStream;
#[must_use]
pub fn gen_wh_flts(input_ts: &Ts2) -> Ts2 {
    #[derive(Clone, Optml)]
    enum Generic {
        False,
        True { mb_extra_traits_ts: Option<Ts2> },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Clone, Optml)]
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
    #[derive(Debug, serde::Deserialize, Optml)]
    struct GenWhFltsConfig {
        pg_types_write_into_file: ShouldWriteTsIntoFile,
        whole_write_into_file: ShouldWriteTsIntoFile,
    }
    panic_loc();
    let gen_wh_flts_config = match from_str::<GenWhFltsConfig>(&input_ts.to_string()) {
        Ok(v) => v,
        Err(er) => {
            let msg = format!("failed to parse GenWhFltsConfig: {er}");
            return quote! { compile_error!(#msg); };
        }
    };
    let import = Import::PgCrudCmn;
    let t_ts = quote! {T};
    let t_ann_generic_ts = quote! {<#t_ts>};
    let proc_macro2_ts_new = Ts2::new();
    let pub_v_t_ts = quote! {pub #VSc: T};
    let v_dflt_some_one_el_ts = quote! {
        #VSc: #PgCrudCmnDfltSomeOneElCall
    };
    let gen_struct_ts = |flt_init_with_try_new_result_is_ok: bool,
                         generic: &Generic,
                         ident: &dyn ToTokens,
                         struct_extra_fields_ts: &dyn ToTokens| {
        let mb_pub_ts: &dyn ToTokens = if flt_init_with_try_new_result_is_ok {
            &proc_macro2_ts_new
        } else {
            &PubSc
        };
        DTsBuilder::new()
            .make_pub()
            .d_debug()
            .d_clone()
            .d_partial_eq()
            .d_serde_serialize()
            .d_serde_deserialize_if(if flt_init_with_try_new_result_is_ok {
                DSerdeDeserialize::False
            } else {
                DSerdeDeserialize::True
            })
            .d_schemars_json_schema()
            .build_struct(
                &Ts2::new(),
                &ident,
                &match &generic {
                    Generic::False => proc_macro2_ts_new.clone(),
                    Generic::True { mb_extra_traits_ts } => mb_extra_traits_ts
                        .as_ref()
                        .map_or_else(|| quote! {<#t_ts>}, |v| quote! {<#t_ts: #v>}),
                },
                &quote::quote! {{
                    #mb_pub_ts oprtr: #import::Oprtr,
                    #struct_extra_fields_ts
                }},
            )
    };
    let gen_impl_dflt_some_one_el_ts = |generic: &Generic,
                                        ident: &dyn ToTokens,
                                        ts: &dyn ToTokens| {
        gen_impl_dflt_some_one_el_ts(
            &match &generic {
                Generic::False => Ts2::new(),
                Generic::True { mb_extra_traits_ts } => mb_extra_traits_ts.as_ref().map_or_else(
                    || quote! {<T: #PgCrudCmnDfltSomeOneEl>},
                    |v| quote! {<T: #v + #PgCrudCmnDfltSomeOneEl>},
                ),
            },
            &Import::PgCrudCmn,
            &ident,
            match &generic {
                Generic::False => &proc_macro2_ts_new,
                Generic::True { .. } => &t_ann_generic_ts,
            },
            &quote! {
                Self {
                    oprtr: #PgCrudCmnDfltSomeOneElCall,
                    #ts
                }
            },
        )
    };
    let gen_impl_pg_type_wh_flt_ts = |generic: &Generic,
                                      ident: &dyn ToTokens,
                                      incr_prm_undrscr: &IncrPrmUndrscr,
                                      add_oprtr_undrscr: &AddOprtrUndrscr,
                                      qp_ts: &dyn ToTokens,
                                      is_qb_mut: &IsQbMut,
                                      qb_ts: &dyn ToTokens| {
        impl_pg_type_wh_flt_for_ident_ts(
            &{
                let mb_t_extra_traits_for_pg_type_wh_flt_ts: &dyn ToTokens = match &generic {
                    Generic::False => &proc_macro2_ts_new,
                    Generic::True { mb_extra_traits_ts } => {
                        let send_and_lt_ts = quote! {Send + 'lt};
                        let ts = mb_extra_traits_ts.as_ref().map_or_else(
                            || send_and_lt_ts.clone(),
                            |v| quote! {#v + #send_and_lt_ts},
                        );
                        &quote! {, T: #ts}
                    }
                };
                quote! {<'lt #mb_t_extra_traits_for_pg_type_wh_flt_ts>}
            },
            &ident,
            &match &generic {
                Generic::False => &proc_macro2_ts_new,
                Generic::True { .. } => &t_ann_generic_ts,
            },
            incr_prm_undrscr,
            &ColPrmUndrscr::False,
            add_oprtr_undrscr,
            &qp_ts,
            is_qb_mut,
            &qb_ts,
            &Import::PgCrudCmn,
        )
    };
    let add_rgx_case_and_v_dcl_ts = |ts: &dyn ToTokens| {
        quote! {
            #ts
            pub rgx_case: RgxCase,
            pub #VSc: RgxRgx
        }
    };
    let add_rgx_case_and_v_dflt_init_ts = |ts: &dyn ToTokens| {
        quote! {
            #ts
            rgx_case: #PgCrudCmnDfltSomeOneElCall,
            #v_dflt_some_one_el_ts
        }
    };
    let gen_match_incr_checked_add_one_init_ts = |ts: &dyn ToTokens| {
        let match_ts = gen_match_ok_or_return_err_ts(
            &quote! {#import::incr_checked_add_one_returning_incr(#IncrSc)},
            &quote! {v_25d59e01},
        );
        quote! {
            let #ts = #match_ts;
        }
    };
    let v_match_incr_checked_add_one_init_ts = gen_match_incr_checked_add_one_init_ts(&VSc);
    let self_oprtr_to_qp_ts = quote! {&#SelfSc.oprtr.to_qp(add_oprtr),};
    let gen_rgx_qp_format_ts =
        |v: &dyn Display, mb_dims_ies_init_ts: &dyn ToTokens, mb_extra_prms_ts: &dyn ToTokens| {
            let format_ts = dq_ts(&v);
            quote! {
                #mb_dims_ies_init_ts
                #v_match_incr_checked_add_one_init_ts
                Ok(format!(
                    #format_ts,
                    #self_oprtr_to_qp_ts
                    #ColSc,
                    #mb_extra_prms_ts
                    #SelfSc.rgx_case.postgreql_syntax(),
                    #VSc
                ))
            }
        };
    let if_let_err_query_try_bind_self_v_to_string_ts = quote! {
        if let Err(#ErSc) = #QuerySc.try_bind(#SelfSc.#VSc.to_string()) {
            return Err(#ErSc.to_string());
        }
        Ok(#QuerySc)
    };
    let if_let_err_query_try_bind_self_v_ts = quote! {
        if let Err(#ErSc) = #QuerySc.try_bind(#SelfSc.#VSc) {
            return Err(#ErSc.to_string());
        }
    };
    let qb_one_v_ts = quote! {
        #if_let_err_query_try_bind_self_v_ts
        Ok(#QuerySc)
    };
    let generic_false = Generic::False;
    let generic_true_debug_partial_eq_partial_ord_clone_type_encode = Generic::True {
        mb_extra_traits_ts: Some(quote! {
            std::fmt::Debug
            + PartialEq
            + PartialOrd
            + Clone
            + sqlx::Type<sqlx::Postgres>
            + for<'__> sqlx::Encode<'__, sqlx::Postgres>
        }),
    };
    let pub_v_btwn_t_ts = quote! {pub #VSc: Btwn<T>};
    let gen_match_qb_ts = |field_ts: &dyn ToTokens| {
        gen_match_ok_assign_or_return_err_ts(
            &quote! {#field_ts.qb(#QuerySc)},
            &QuerySc,
            &quote! {v_f6d31bdd},
        )
    };
    let query_self_v_qb_ts = {
        let ts = gen_match_qb_ts(&quote! {#SelfSc.#VSc});
        quote! {
            #ts
            Ok(#QuerySc)
        }
    };
    let pg_type_ptrn_stdrt = PgTypePtrn::Stdrt;
    let gen_ident_match_field_fn_ok_v_return_err_ts =
        |ident_ts: &dyn ToTokens, field_ts: &dyn ToTokens, fn_ts: &dyn ToTokens| {
            let match_ts = gen_match_ok_or_return_err_ts(
                &quote! {self.#field_ts.#fn_ts(#IncrSc, #ColSc, add_oprtr)},
                &quote! {v_0a22ee9a},
            );
            quote! {
                let #ident_ts = #match_ts;
            }
        };
    let v_match_self_v_qp_init_ts =
        gen_ident_match_field_fn_ok_v_return_err_ts(&VSc, &VSc, &quote! {qp});
    let gen_mb_dims_dcl_pub_v_t_ts = |ts: &dyn ToTokens| {
        quote! {
            #ts
            #pub_v_t_ts
        }
    };
    let gen_mb_dims_dflt_init_v_dflt_ts = |ts: &dyn ToTokens| {
        quote! {
            #ts
            #v_dflt_some_one_el_ts
        }
    };
    let gen_two_ts = |mb_dims_qb_ts: &dyn ToTokens, trailing_ts: &dyn ToTokens| {
        quote! {
            #mb_dims_qb_ts
            #trailing_ts
        }
    };
    let is_qb_mut_true = IsQbMut::True;
    let is_qb_mut_false = IsQbMut::False;
    let gen_qp_format_with_v_ts =
        |mb_dims_ies_init_ts: &dyn ToTokens,
         format_ts: &dyn ToTokens,
         mb_extra_prms_ts: &dyn ToTokens| {
            quote! {
                #mb_dims_ies_init_ts
                #v_match_incr_checked_add_one_init_ts
                Ok(format!(
                    #format_ts,
                    #self_oprtr_to_qp_ts
                    #ColSc,
                    #mb_extra_prms_ts
                    #VSc
                ))
            }
        };
    let gen_pg_type_dims_helpers = |pg_type_ptrn: &PgTypePtrn| match pg_type_ptrn {
        PgTypePtrn::Stdrt => (
            Ts2::new(),
            Ts2::new(),
            Ts2::new(),
            PgTypeKind::Stdrt,
            Ts2::new(),
            Ts2::new(),
        ),
    };
    let pg_type_ts = {
        let gen_flts_ts = |flt: &PgTypeFlt| {
            let ident = PgTypeWhSelfUcc::from_display(&flt);
            let (
                generic,
                struct_extra_fields_ts,
                impl_dflt_some_one_el_extra_fields_ts,
                incr_prm_undrscr,
                qp_ts,
                is_qb_mut,
                qb_ts,
            ) = {
                let sqlx_type_pg_encode_ts = quote! {sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>};
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
                            IncrPrmUndrscr::False,
                            gen_qp_format_with_v_ts(
                                &mb_dims_ies_init_ts,
                                &dq_ts(&gen_format_h_str(&pg_type_kind)),
                                &mb_extra_prms_ts,
                            ),
                            is_qb_mut_true,
                            gen_two_ts(&mb_dims_qb_ts, &qb_one_v_ts),
                        )
                    };
                let gen_oprtr_cmp_flt_ts = |pg_type_ptrn: &PgTypePtrn, oprtr: &dyn Display| {
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
                        quote! {
                            #mb_dims_dcl_ts
                            #pub_v_btwn_t_ts
                        },
                        gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                        IncrPrmUndrscr::False,
                        {
                            let format_ts = dq_ts(&format!(
                                "{{}}({{}}{} {{}})",
                                pg_type_kind.format_argument()
                            ));
                            quote! {
                                #mb_dims_ies_init_ts
                                #v_match_self_v_qp_init_ts
                                Ok(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #ColSc,
                                    #mb_extra_prms_ts
                                    #VSc
                                ))
                            }
                        },
                        is_qb_mut_true,
                        quote! {
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
                                quote! {std::fmt::Debug + PartialEq + Clone + #sqlx_type_pg_encode_ts},
                            ),
                        },
                        quote! {
                            #mb_dims_dcl_ts
                            pub #VSc: PgTypeNotEmptyUnqVec<T>
                        },
                        gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                        IncrPrmUndrscr::False,
                        {
                            let format_ts = dq_ts(&format!(
                                "{{}}({{}}{} in ({{}}))",
                                pg_type_kind.format_argument()
                            ));
                            let if_write_is_err_ts = gen_if_write_is_err_ts(
                                &quote! {acc, "${v_daedba9c},"},
                                &quote! {return Err(#import::QpEr::WriteIntoBuffer { loc: loc_lib::loc!() });},
                            );
                            quote! {
                                #mb_dims_ies_init_ts
                                let #VSc = {
                                    let mut acc = String::default();
                                    for _ in #SelfSc.#VSc.to_vec() {
                                        match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                                            Ok(v_daedba9c) => {
                                                #if_write_is_err_ts
                                            },
                                            Err(#ErSc) => {
                                                return Err(#ErSc);
                                            },
                                        }
                                    }
                                    let _: Option<char> = acc.pop();
                                    acc
                                };
                                Ok(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #ColSc,
                                    #mb_extra_prms_ts
                                    #VSc
                                ))
                            }
                        },
                        is_qb_mut_true,
                        quote! {
                            #mb_dims_qb_ts
                            for el in #SelfSc.#VSc.into_vec() {
                                if let Err(#ErSc) = #QuerySc.try_bind(el) {
                                    return Err(#ErSc.to_string());
                                }
                            }
                            Ok(#QuerySc)
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
                        IncrPrmUndrscr::False,
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
                let gen_pg_syntax_flt_ts = |pg_type_ptrn: &PgTypePtrn, pg_syntax: &dyn Display| {
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
                        IncrPrmUndrscr::True,
                        {
                            let format_ts = dq_ts(&format!(
                                "{{}}({{}}{} {pg_syntax})",
                                pg_type_kind.format_argument()
                            ));
                            quote! {
                                #mb_dims_ies_init_ts
                                Ok(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #ColSc,
                                    #mb_extra_prms_ts
                                ))
                            }
                        },
                        is_qb_mut_false,
                        quote! {
                            #mb_dims_qb_ts
                            Ok(#QuerySc)
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
                        quote! {
                            #mb_dims_dcl_ts
                            pub encode_format: EncodeFormat,
                            pub encoded_string_representation: String,
                        },
                        quote! {
                            #mb_dims_dflt_init_ts
                            encode_format: #PgCrudCmnDfltSomeOneElCall,
                            encoded_string_representation: String::default()
                        },
                        IncrPrmUndrscr::False,
                        {
                            let format_ts = dq_ts(&format!(
                                "{{}}(encode({{}}{}, '{{}}') = ${{}})",
                                pg_type_kind.format_argument()
                            ));
                            quote! {
                                #mb_dims_ies_init_ts
                                #v_match_incr_checked_add_one_init_ts
                                Ok(format!(
                                    #format_ts,
                                    #self_oprtr_to_qp_ts
                                    #ColSc,
                                    #mb_extra_prms_ts
                                    &#SelfSc.encode_format,
                                    #VSc
                                ))
                            }
                        },
                        is_qb_mut_true,
                        quote! {
                            #mb_dims_qb_ts
                            if let Err(#ErSc) = #QuerySc.try_bind(self.encoded_string_representation) {
                                return Err(#ErSc.to_string());
                            }
                            Ok(#QuerySc)
                        },
                    )
                };
                let gen_range_bound_cmp_flt_ts =
                    |pg_type_ptrn: &PgTypePtrn, bound_fn: &str, oprtr: &str| {
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
                        quote! {
                            #mb_dims_dcl_ts
                            pub #VSc: #import::NotZeroUnsignedPartOfI32
                        },
                        gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                        IncrPrmUndrscr::False,
                        gen_qp_format_with_v_ts(
                            &mb_dims_ies_init_ts,
                            &dq_ts(&format!(
                                "{{}}(upper({{}}{}) - lower({{}}{}) = ${{}})",
                                pg_type_kind.format_argument(),
                                pg_type_kind.format_argument(),
                            )),
                            &quote! {
                                #mb_extra_prms_ts
                                #ColSc,
                            },
                        ),
                        is_qb_mut_true,
                        quote! {
                            #mb_dims_qb_ts
                            #qb_one_v_ts
                        },
                    )
                };
                let gen_eq_oprtr_qp_ts =
                    |mb_dims_ies_init_ts: &dyn ToTokens, format_ts: &dyn ToTokens| {
                        quote! {
                            #mb_dims_ies_init_ts
                            let oprtr = <T as #import::PgTypeEqOprtr>::oprtr(&#SelfSc.#VSc);
                            let oprtr_query_str = oprtr.to_query_str();
                            Ok(format!(
                                #format_ts,
                                #self_oprtr_to_qp_ts
                                #ColSc,
                                match oprtr {
                                    #import::EqOprtr::Eq => {
                                        #v_match_incr_checked_add_one_init_ts
                                        format!("{oprtr_query_str} ${v}")
                                    },
                                    #import::EqOprtr::IsNull => oprtr_query_str.to_owned(),
                                }
                            ))
                        }
                    };
                let gen_eq_oprtr_qb_ts = |ts: &dyn ToTokens| {
                    quote! {
                        #ts
                        if matches!(&<T as #import::PgTypeEqOprtr>::oprtr(&#SelfSc.#VSc), #import::EqOprtr::Eq)
                            && let Err(#ErSc) = #QuerySc.try_bind(#SelfSc.#VSc)
                        {
                            return Err(#ErSc.to_string());
                        }
                        Ok(#QuerySc)
                    }
                };
                match &flt {
                    PgTypeFlt::Eq { .. } => {
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
                                    quote! {#sqlx_type_pg_encode_ts + #import::PgTypeEqOprtr},
                                ),
                            },
                            gen_mb_dims_dcl_pub_v_t_ts(&mb_dims_dcl_ts),
                            gen_mb_dims_dflt_init_v_dflt_ts(&mb_dims_dflt_init_ts),
                            IncrPrmUndrscr::False,
                            gen_eq_oprtr_qp_ts(&mb_dims_ies_init_ts, &quote! {"{}({} {})"}),
                            is_qb_mut_true,
                            gen_eq_oprtr_qb_ts(&mb_dims_qb_ts),
                        )
                    }
                    PgTypeFlt::GreaterThan { .. } => gen_greater_than_ts(&pg_type_ptrn_stdrt),
                    PgTypeFlt::Btwn { .. } => gen_btwn_ts(&pg_type_ptrn_stdrt),
                    PgTypeFlt::In { .. } => gen_in_ts(&pg_type_ptrn_stdrt),
                    PgTypeFlt::Rgx => gen_rgx_ts(&pg_type_ptrn_stdrt),
                    PgTypeFlt::Before { .. } => gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"<"),
                    PgTypeFlt::CrntDate => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"= current_date")
                    }
                    PgTypeFlt::GreaterThanCrntDate => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"> current_date")
                    }
                    PgTypeFlt::CrntTimestamp => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"= current_timestamp")
                    }
                    PgTypeFlt::GreaterThanCrntTimestamp => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"> current_timestamp")
                    }
                    PgTypeFlt::CrntTime => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"= current_time")
                    }
                    PgTypeFlt::GreaterThanCrntTime => {
                        gen_pg_syntax_flt_ts(&pg_type_ptrn_stdrt, &"> current_time")
                    }
                    PgTypeFlt::EqToEncodedStringRepresentation => {
                        gen_eq_to_encoded_string_representation_ts(&pg_type_ptrn_stdrt)
                    }
                    PgTypeFlt::FindRangesWithinGivenRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"<@")
                    }
                    PgTypeFlt::FindRangesThatFullyContainTheGivenRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"@>")
                    }
                    PgTypeFlt::StrictlyToLeftOfRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"&<")
                    }
                    PgTypeFlt::StrictlyToRightOfRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"&>")
                    }
                    PgTypeFlt::IncludedLowerBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_stdrt, "lower", "=")
                    }
                    PgTypeFlt::ExcludedUpperBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_stdrt, "upper", "=")
                    }
                    PgTypeFlt::GreaterThanIncludedLowerBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_stdrt, "lower", ">")
                    }
                    PgTypeFlt::GreaterThanExcludedUpperBound { .. } => {
                        gen_range_bound_cmp_flt_ts(&pg_type_ptrn_stdrt, "upper", ">")
                    }
                    PgTypeFlt::OverlapWithRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"&&")
                    }
                    PgTypeFlt::AdjacentWithRange { .. } => {
                        gen_oprtr_cmp_flt_ts(&pg_type_ptrn_stdrt, &"-|-")
                    }
                    PgTypeFlt::RangeLen => gen_range_len_ts(&pg_type_ptrn_stdrt),
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
                &AddOprtrUndrscr::False,
                &qp_ts,
                &is_qb_mut,
                &qb_ts,
            );
            let gend = quote! {
                #struct_ts
                #impl_dflt_some_one_el_ts
                #impl_pg_type_wh_flt_ts
            };
            gend
        };
        let flt_arr_ts = PgTypeFlt::iter()
            .map(|el| gen_flts_ts(&el))
            .collect::<Vec<_>>();
        let gend = quote! {#(#flt_arr_ts)*};
        mb_write_ts_into_file(
            gen_wh_flts_config.pg_types_write_into_file,
            "gen_wh_flts_pg_types",
            &gend,
            &FormatWithCargofmt::True,
        );
        gend
    };
    let imports_ts = quote! {
        #[allow(clippy::wildcard_imports)]
        use super::*;
    };
    let gend = pg_crud_macros_cmn::gen_mod_with_pub_use_ts(
        &quote::format_ident!("gen_wh_flts_mod"),
        &[imports_ts, pg_type_ts],
    );
    mb_write_ts_into_file(
        gen_wh_flts_config.whole_write_into_file,
        "gen_wh_flts",
        &gend,
        &FormatWithCargofmt::True,
    );
    gend
}
