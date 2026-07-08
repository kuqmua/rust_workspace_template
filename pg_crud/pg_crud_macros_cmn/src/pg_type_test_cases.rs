#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_opt_vec_cr_ts(
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (CrUcc, OptVecCrSc) = (&names.CrUcc, &names.OptVecCrSc);
    quote::quote! {
        fn #OptVecCrSc() -> Option<Vec<#path_ts::#CrUcc>> {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_rd_ids_to_2_dims_vec_rd_inn_ts(
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (RdIdsSc, RdIdsTo2DimsVecRdInnSc, RdIdsUcc, RdInnUcc) = (
        &names.RdIdsSc,
        &names.RdIdsTo2DimsVecRdInnSc,
        &names.RdIdsUcc,
        &names.RdInnUcc,
    );
    quote::quote! {
        fn #RdIdsTo2DimsVecRdInnSc(
            #RdIdsSc: &#path_ts::#RdIdsUcc
        ) -> Vec<Vec<#path_ts::#RdInnUcc>> {
            #ts
        }
    }
    .into()
}
fn gen_rd_inn_into_rd_or_upd_with_new_or_try_new_unwraped_ts(
    method_name_ts: &dyn quote::ToTokens,
    type_ts: &dyn quote::ToTokens,
    path_ts: &dyn quote::ToTokens,
    return_type_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (VSc,) = (&names.VSc,);
    quote::quote! {
        fn #method_name_ts(
            #VSc: #type_ts
        ) -> #path_ts::#return_type_ts {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_upd_to_rd_ids_ts(
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (RdIdsUcc, UpdToRdIdsSc, UpdUcc, VSc) = (
        &names.RdIdsUcc,
        &names.UpdToRdIdsSc,
        &names.UpdUcc,
        &names.VSc,
    );
    quote::quote! {
        fn #UpdToRdIdsSc(
            #VSc: &#path_ts::#UpdUcc
        ) -> #path_ts::#RdIdsUcc {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_rd_ids_to_opt_v_rd_dflt_some_one_el_ts(
    import: crate::Import,
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (RdIdsToOptVRdDfltSomeOneElSc, RdIdsUcc, RdUcc, VSc, VUcc) = (
        &names.RdIdsToOptVRdDfltSomeOneElSc,
        &names.RdIdsUcc,
        &names.RdUcc,
        &names.VSc,
        &names.VUcc,
    );
    quote::quote! {
        fn #RdIdsToOptVRdDfltSomeOneElSc(
            #VSc: &#path_ts::#RdIdsUcc
        ) -> Option<#import::#VUcc<#path_ts::#RdUcc>> {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_previous_rd_and_opt_upd_into_rd_ts(
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (OptUpdSc, PreviousRdAndOptUpdIntoRdSc, RdSc, RdUcc, UpdUcc) = (
        &names.OptUpdSc,
        &names.PreviousRdAndOptUpdIntoRdSc,
        &names.RdSc,
        &names.RdUcc,
        &names.UpdUcc,
    );
    quote::quote! {
        fn #PreviousRdAndOptUpdIntoRdSc(
            #RdSc: #path_ts::#RdUcc,
            #OptUpdSc: Option<#path_ts::#UpdUcc>,
        ) -> #path_ts::#RdUcc {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_rd_ids_and_cr_into_rd_ts(
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (CrSc, CrUcc, RdIdsAndCrIntoRdSc, RdIdsSc, RdIdsUcc, RdUcc) = (
        &names.CrSc,
        &names.CrUcc,
        &names.RdIdsAndCrIntoRdSc,
        &names.RdIdsSc,
        &names.RdIdsUcc,
        &names.RdUcc,
    );
    quote::quote! {
        fn #RdIdsAndCrIntoRdSc(
            #RdIdsSc: #path_ts::#RdIdsUcc,
            #CrSc: #path_ts::#CrUcc
        ) -> #path_ts::#RdUcc {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_rd_ids_and_cr_into_opt_v_rd_ts(
    import: crate::Import,
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (CrSc, CrUcc, RdIdsAndCrIntoOptVRdSc, RdIdsSc, RdIdsUcc, RdUcc, VUcc) = (
        &names.CrSc,
        &names.CrUcc,
        &names.RdIdsAndCrIntoOptVRdSc,
        &names.RdIdsSc,
        &names.RdIdsUcc,
        &names.RdUcc,
        &names.VUcc,
    );
    quote::quote! {
        fn #RdIdsAndCrIntoOptVRdSc(
            #RdIdsSc: #path_ts::#RdIdsUcc,
            #CrSc: #path_ts::#CrUcc
        ) -> Option<#import::#VUcc<#path_ts::#RdUcc>> {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_rd_ids_and_cr_into_tt_ts(
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (CrSc, CrUcc, RdIdsAndCrIntoTtSc, RdIdsSc, RdIdsUcc, TtUcc) = (
        &names.CrSc,
        &names.CrUcc,
        &names.RdIdsAndCrIntoTtSc,
        &names.RdIdsSc,
        &names.RdIdsUcc,
        &names.TtUcc,
    );
    quote::quote! {
        fn #RdIdsAndCrIntoTtSc(
            #RdIdsSc: #path_ts::#RdIdsUcc,
            #CrSc: #path_ts::#CrUcc
        ) -> #path_ts::#TtUcc {
            #ts
        }
    }
    .into()
}
pub fn gen_rd_ids_and_cr_into_wh_eq_ts(
    rd_ids_ts: &dyn quote::ToTokens,
    cr_ts: &dyn quote::ToTokens,
    wh_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (CrSc, RdIdsAndCrIntoWhEqSc, RdIdsSc) =
        (&names.CrSc, &names.RdIdsAndCrIntoWhEqSc, &names.RdIdsSc);
    quote::quote! {
        fn #RdIdsAndCrIntoWhEqSc(
            #RdIdsSc: #rd_ids_ts,
            #CrSc: #cr_ts
        ) -> #wh_ts {
            #ts
        }
    }
    .into()
}
pub fn gen_rd_ids_and_cr_into_vec_wh_eq_using_fields_ts(
    import: &crate::Import,
    rd_ids_ts: &dyn quote::ToTokens,
    cr_ts: &dyn quote::ToTokens,
    wh_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (CrSc, RdIdsAndCrIntoVecWhEqUsingFieldsSc, RdIdsSc) = (
        &names.CrSc,
        &names.RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        &names.RdIdsSc,
    );
    quote::quote! {
        fn #RdIdsAndCrIntoVecWhEqUsingFieldsSc(
            #RdIdsSc: #rd_ids_ts,
            #CrSc: #cr_ts
        ) -> #import::NotEmptyUnqVec<#wh_ts> {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts(
    import: crate::Import,
    rd_ids_ts: &dyn quote::ToTokens,
    cr_ts: &dyn quote::ToTokens,
    wh_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (CrSc, RdIdsAndCrIntoOptVecWhEqToFieldSc, RdIdsSc) = (
        &names.CrSc,
        &names.RdIdsAndCrIntoOptVecWhEqToFieldSc,
        &names.RdIdsSc,
    );
    let return_type_ts =
        crate::gen_opt_type_dcl_ts(&quote::quote! {#import::NotEmptyUnqVec<#wh_ts>});
    quote::quote! {
        fn #RdIdsAndCrIntoOptVecWhEqToFieldSc(
            #RdIdsSc: #rd_ids_ts,
            #CrSc: #cr_ts
        ) -> #return_type_ts {
            #ts
        }
    }
    .into()
}
pub fn gen_impl_pg_type_test_cases_for_ident_ts(
    cfg_ts: &dyn quote::ToTokens,
    import: &crate::Import,
    type_ts: &dyn quote::ToTokens,
    ident: &dyn quote::ToTokens,
    opt_vec_cr_ts: Option<&macros_helpers::GeneratedRustTs>,
    rd_ids_to_2_dims_vec_rd_inn_ts: &dyn quote::ToTokens,
    rd_inn_into_rd_with_new_or_try_new_unwraped_ts: &dyn quote::ToTokens,
    rd_inn_into_upd_with_new_or_try_new_unwraped_ts: &dyn quote::ToTokens,
    upd_to_rd_ids_ts: &dyn quote::ToTokens,
    rd_ids_to_opt_v_rd_dflt_some_one_el_ts: &dyn quote::ToTokens,
    previous_rd_and_opt_upd_into_rd_ts: &dyn quote::ToTokens,
    rd_ids_and_cr_into_rd_ts: &dyn quote::ToTokens,
    rd_ids_and_cr_into_opt_v_rd_ts: &dyn quote::ToTokens,
    rd_ids_and_cr_into_tt_ts: &dyn quote::ToTokens,
    rd_ids_and_cr_into_wh_eq_ts: &dyn quote::ToTokens,
    rd_ids_and_cr_into_vec_wh_eq_using_fields_ts: &dyn quote::ToTokens,
    rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts: Option<&macros_helpers::GeneratedRustTs>,
    pg_type_opt_vec_wh_greater_than_test_ts: Option<&macros_helpers::GeneratedRustTs>,
    rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts: Option<&macros_helpers::GeneratedRustTs>,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (
        AllowClippyArbitrarySrcItemOrdering,
        CrUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        WhUcc,
    ) = (
        &names.AllowClippyArbitrarySrcItemOrdering,
        &names.CrUcc,
        &names.PgTypeOptVecWhGreaterThanTestSc,
        &names.PgTypeTestCasesUcc,
        &names.PgTypeUcc,
        &names.RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        &names.RdIdsSc,
        &names.RdIdsUcc,
        &names.RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        &names.RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        &names.SelUcc,
        &names.SelfUcc,
        &names.TtSc,
        &names.TtUcc,
        &names.WhUcc,
    );
    let self_pg_type_as_pg_type_ts = quote::quote! {<#SelfUcc::#PgTypeUcc as #import::#PgTypeUcc>};
    let self_pg_type_as_pg_type_rd_ids_ts = quote::quote! {#self_pg_type_as_pg_type_ts::#RdIdsUcc};
    let self_pg_type_as_pg_type_cr_ts = quote::quote! {#self_pg_type_as_pg_type_ts::#CrUcc};
    let self_pg_type_as_pg_type_wh_ts = quote::quote! {#self_pg_type_as_pg_type_ts::#WhUcc};
    let ident_sel_ucc = naming::prm::SelfSelUcc::from_tokens(&ident);
    let opt_vec_cr_ts_gnrtd =
        opt_vec_cr_ts.map(|ts| gen_opt_vec_cr_ts(&self_pg_type_as_pg_type_ts, ts));
    let rd_ids_to_2_dims_vec_rd_inn_ts_gnrtd = gen_rd_ids_to_2_dims_vec_rd_inn_ts(
        &self_pg_type_as_pg_type_ts,
        &rd_ids_to_2_dims_vec_rd_inn_ts,
    );
    let rd_inn_into_rd_with_new_or_try_new_unwraped_ts_gnrtd =
        gen_rd_inn_into_rd_or_upd_with_new_or_try_new_unwraped_ts(
            &RdInnIntoRdWithNewOrTryNewUnwrapedSc,
            &type_ts,
            &self_pg_type_as_pg_type_ts,
            &naming::RdUcc,
            &rd_inn_into_rd_with_new_or_try_new_unwraped_ts,
        );
    let rd_inn_into_upd_with_new_or_try_new_unwraped_ts_gnrtd =
        gen_rd_inn_into_rd_or_upd_with_new_or_try_new_unwraped_ts(
            &RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
            &type_ts,
            &self_pg_type_as_pg_type_ts,
            &naming::UpdUcc,
            &rd_inn_into_upd_with_new_or_try_new_unwraped_ts,
        );
    let upd_to_rd_ids_ts_gnrtd =
        gen_upd_to_rd_ids_ts(&self_pg_type_as_pg_type_ts, &upd_to_rd_ids_ts);
    let rd_ids_to_opt_v_rd_dflt_some_one_el_ts_gnrtd = gen_rd_ids_to_opt_v_rd_dflt_some_one_el_ts(
        *import,
        &self_pg_type_as_pg_type_ts,
        &rd_ids_to_opt_v_rd_dflt_some_one_el_ts,
    );
    let previous_rd_and_opt_upd_into_rd_ts_gnrtd = gen_previous_rd_and_opt_upd_into_rd_ts(
        &self_pg_type_as_pg_type_ts,
        &previous_rd_and_opt_upd_into_rd_ts,
    );
    let rd_ids_and_cr_into_rd_ts_gnrtd =
        gen_rd_ids_and_cr_into_rd_ts(&self_pg_type_as_pg_type_ts, &rd_ids_and_cr_into_rd_ts);
    let rd_ids_and_cr_into_opt_v_rd_ts_gnrtd = gen_rd_ids_and_cr_into_opt_v_rd_ts(
        *import,
        &self_pg_type_as_pg_type_ts,
        &rd_ids_and_cr_into_opt_v_rd_ts,
    );
    let rd_ids_and_cr_into_tt_ts_gnrtd =
        gen_rd_ids_and_cr_into_tt_ts(&self_pg_type_as_pg_type_ts, &rd_ids_and_cr_into_tt_ts);
    let rd_ids_and_cr_into_wh_eq_ts_gnrtd = gen_rd_ids_and_cr_into_wh_eq_ts(
        &self_pg_type_as_pg_type_rd_ids_ts,
        &self_pg_type_as_pg_type_cr_ts,
        &self_pg_type_as_pg_type_wh_ts,
        &rd_ids_and_cr_into_wh_eq_ts,
    );
    let rd_ids_and_cr_into_vec_wh_eq_using_fields_ts_gnrtd =
        gen_rd_ids_and_cr_into_vec_wh_eq_using_fields_ts(
            import,
            &self_pg_type_as_pg_type_rd_ids_ts,
            &self_pg_type_as_pg_type_cr_ts,
            &self_pg_type_as_pg_type_wh_ts,
            &rd_ids_and_cr_into_vec_wh_eq_using_fields_ts,
        );
    let rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts_gnrtd =
        rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts.map(|ts| {
            gen_rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts(
                *import,
                &self_pg_type_as_pg_type_rd_ids_ts,
                &self_pg_type_as_pg_type_cr_ts,
                &self_pg_type_as_pg_type_wh_ts,
                ts,
            )
        });
    let pg_type_opt_vec_wh_greater_than_test_ts_gnrtd = pg_type_opt_vec_wh_greater_than_test_ts
        .map(|ts| {
            quote::quote! {
                fn #PgTypeOptVecWhGreaterThanTestSc() -> Option<
                    #import::NotEmptyUnqVec<
                        #import::PgTypeGreaterThanTest<
                            #SelfUcc::#PgTypeUcc
                        >
                    >
                > {
                    #ts
                }
            }
        });
    let rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts_gnrtd =
        rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts.map(|ts| {
            let rd_ids_and_tt_into_pg_type_opt_wh_greater_than_sc =
                RdIdsAndTtIntoPgTypeOptWhGreaterThanSc;
            quote::quote! {
                fn #rd_ids_and_tt_into_pg_type_opt_wh_greater_than_sc(
                    greater_than_vrt: #import::PgTypeGreaterThanVrt,
                    #RdIdsSc: #self_pg_type_as_pg_type_ts::#RdIdsUcc,
                    #TtSc: #self_pg_type_as_pg_type_ts::#TtUcc,
                ) -> Option<#self_pg_type_as_pg_type_ts::#WhUcc> {
                    #ts
                }
            }
        });
    quote::quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #AllowClippyArbitrarySrcItemOrdering
        #cfg_ts
        #[allow(clippy::float_arithmetic)]
        impl #import::#PgTypeTestCasesUcc for #ident {
            type #PgTypeUcc = #SelfUcc;
            type #SelUcc = #ident_sel_ucc;
            #opt_vec_cr_ts_gnrtd
            #rd_ids_to_2_dims_vec_rd_inn_ts_gnrtd
            #rd_inn_into_rd_with_new_or_try_new_unwraped_ts_gnrtd
            #rd_inn_into_upd_with_new_or_try_new_unwraped_ts_gnrtd
            #upd_to_rd_ids_ts_gnrtd
            #rd_ids_to_opt_v_rd_dflt_some_one_el_ts_gnrtd
            #previous_rd_and_opt_upd_into_rd_ts_gnrtd
            #rd_ids_and_cr_into_rd_ts_gnrtd
            #rd_ids_and_cr_into_opt_v_rd_ts_gnrtd
            #rd_ids_and_cr_into_tt_ts_gnrtd
            #rd_ids_and_cr_into_wh_eq_ts_gnrtd
            #rd_ids_and_cr_into_vec_wh_eq_using_fields_ts_gnrtd
            #rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts_gnrtd
            #pg_type_opt_vec_wh_greater_than_test_ts_gnrtd
            #rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts_gnrtd
        }
    }
    .into()
}
