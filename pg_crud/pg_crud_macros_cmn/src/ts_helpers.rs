#[must_use]
pub fn parse_strs_to_ts2_vec(
    v: crate::ParseTsStrings,
    uuid: crate::ParseErIdRef<'_>,
) -> crate::GeneratedRustTsVec {
    v.into_vec()
        .into_iter()
        .map(|el| parse_ts_or_compile_error(crate::ParseTsTextRef::from(el.as_str()), uuid))
        .collect::<crate::GeneratedRustTsVec>()
}
#[must_use]
pub fn gen_mod_with_pub_use_ts(
    mod_name: &dyn quote::ToTokens,
    content_ts: &crate::GeneratedRustTsVec,
) -> macros_helpers::GeneratedRustTs {
    quote::quote! {
        #[allow(unused_qualifications)]
        #[allow(unused_variables)]
        #[allow(clippy::absolute_paths)]
        #[allow(clippy::arbitrary_source_item_ordering)]
        mod #mod_name {
            #content_ts
        }
        pub use #mod_name::*;
    }
    .into()
}
#[must_use]
pub fn cmn_d_ts_builder() -> macros_helpers::DTsBuilder {
    macros_helpers::DTsBuilder::new()
        .make_pub()
        .d_debug()
        .d_clone()
        .d_partial_eq()
        .d_serde_serialize()
        .d_serde_deserialize()
}
#[must_use]
pub fn serde_er_enum_d_ts_builder() -> macros_helpers::DTsBuilder {
    macros_helpers::DTsBuilder::new()
        .make_pub()
        .d_debug()
        .d_serde_serialize()
        .d_serde_deserialize()
        .d_thiserror_error()
        .d_loc_lib_location()
}
#[must_use]
pub fn er_enum_d_ts_builder() -> macros_helpers::DTsBuilder {
    macros_helpers::DTsBuilder::new()
        .make_pub()
        .d_debug()
        .d_thiserror_error()
        .d_loc_lib_location()
}
#[must_use]
pub fn gen_match_ok_assign_or_return_err_ts(
    expr_ts: &dyn quote::ToTokens,
    assign_target_ts: &dyn quote::ToTokens,
    ok_v_ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (ErSc,) = (&names.ErSc,);
    quote::quote! {
        match #expr_ts {
            Ok(#ok_v_ts) => {
                #assign_target_ts = #ok_v_ts;
            }
            Err(#ErSc) => {
                return Err(#ErSc);
            }
        }
    }
    .into()
}
#[must_use]
pub fn gen_match_ok_or_return_err_ts(
    expr_ts: &dyn quote::ToTokens,
    ok_v_ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (ErSc,) = (&names.ErSc,);
    quote::quote! {
        match #expr_ts {
            Ok(#ok_v_ts) => #ok_v_ts,
            Err(#ErSc) => {
                return Err(#ErSc);
            }
        }
    }
    .into()
}
#[must_use]
pub fn gen_match_not_empty_unq_vec_try_new_some_or_none_ts(
    import: &crate::Import,
    expr_ts: &dyn quote::ToTokens,
    ok_v_ts: &dyn quote::ToTokens,
    panic_uuid: crate::PanicUuidRef<'_>,
) -> macros_helpers::GeneratedRustTs {
    let panic_uuid_ts = gen_quotes::dq_ts(panic_uuid.as_ref());
    quote::quote! {
        match #expr_ts {
            Ok(#ok_v_ts) => Some(#ok_v_ts),
            Err(er) => match er {
                #import::NotEmptyUnqVecTryNewEr::IsEmpty {..} => None,
                #import::NotEmptyUnqVecTryNewEr::NotUnq {..} => panic!(#panic_uuid_ts)
            }
        }
    }
    .into()
}
#[must_use]
pub fn gen_if_let_some_match_ok_assign_query_or_return_err_ts(
    expr_ts: &dyn quote::ToTokens,
    some_v_ts: &dyn quote::ToTokens,
    ok_v_ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (QuerySc, VSc) = (&names.QuerySc, &names.VSc);
    let match_ts = gen_match_ok_assign_or_return_err_ts(expr_ts, &QuerySc, ok_v_ts);
    quote::quote! {
        if let Some(#some_v_ts) = &#VSc.0 {
            #match_ts
        }
        Ok(#QuerySc)
    }
    .into()
}
pub(crate) fn parse_ts_or_compile_error(
    v: crate::ParseTsTextRef<'_>,
    er_id: crate::ParseErIdRef<'_>,
) -> macros_helpers::GeneratedRustTs {
    match v.as_ref().parse::<proc_macro2::TokenStream>() {
        Ok(parsed_ts) => parsed_ts.into(),
        Err(er) => {
            let msg = format!("{}: {er}", er_id.as_ref());
            quote::quote! {compile_error!(#msg);}.into()
        }
    }
}
