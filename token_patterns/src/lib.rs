token_patterns_macros::tp!(SqlxAcquire, sqlx::Acquire);
token_patterns_macros::tp!(
    AxumExtractRejectionJsonRejection,
    axum::extract::rejection::JsonRejection
);
token_patterns_macros::tp!(AxumResIntoRes, axum::response::IntoResponse);
token_patterns_macros::tp!(ReqwestEr, reqwest::Error);
token_patterns_macros::tp!(ReqwestHeaderHeaderMap, reqwest::header::HeaderMap);
token_patterns_macros::tp!(HttpStatusCode, http::StatusCode);
token_patterns_macros::tp!(SqlxRow, sqlx::Row);
token_patterns_macros::tp!(SerdeSerialize, serde::Serialize);
token_patterns_macros::tp!(SerdeDeserialize, serde::Deserialize);
token_patterns_macros::tp!(UtoipaToSchema, utoipa::ToSchema);
token_patterns_macros::tp!(SchemarsJsonSchema, schemars::JsonSchema);
token_patterns_macros::tp!(LocLibLoc, loc_lib::Location);
token_patterns_macros::tp!(ThiserrorError, thiserror::Error);
token_patterns_macros::tp!(Char, char);
token_patterns_macros::tp!(RefStr, &str);
token_patterns_macros::tp!(StringTs, String);
token_patterns_macros::tp!(DeriveDebug, #[derive(Debug, Optml)]);
token_patterns_macros::tp!(DeriveDebugThiserrorLoc, #[derive(Debug, thiserror::Error, loc_lib::Location, Optml)]);
token_patterns_macros::tp!(DeriveDebugUtoipaToSchema, #[derive(Debug, utoipa::ToSchema, Optml)]);
token_patterns_macros::tp!(DeriveDebugSerdeSerializeSerdeDeserialize, #[derive(Debug, serde::Serialize, serde::Deserialize, Optml)]);
token_patterns_macros::tp!(DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema, #[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Optml)]);
token_patterns_macros::tp!(DeriveDebugCloneCopy, #[derive(Debug, Clone, Copy, Optml)]);
token_patterns_macros::tp!(StrSqlxColIndex, &'lt str: sqlx::ColumnIndex<R>,);
token_patterns_macros::tp!(
    SqlxDecodeDecodeDatabase,
    sqlx::decode::Decode<'lt, R::Database>
);
token_patterns_macros::tp!(SqlxTypesTypeDatabase, sqlx::types::Type<R::Database>);
token_patterns_macros::tp!(LocLibLocLoc, loc_lib::loc::Loc);
token_patterns_macros::tp!(LocScDoubleDotSpaceLocLibLocLoc, loc: loc_lib::loc::Loc);
token_patterns_macros::tp!(CoreDefault, ::core::default::Default::default());
token_patterns_macros::tp!(SqlxTypesTimeTimeMidnight, sqlx::types::time::Time::MIDNIGHT);
token_patterns_macros::tp!(
    SqlxTypesTimeOffsetDateTimeUnixEpoch,
    sqlx::types::time::OffsetDateTime::UNIX_EPOCH
);
token_patterns_macros::tp!(Er0, er_0);
token_patterns_macros::tp!(Er1, er_1);
token_patterns_macros::tp!(Er2, er_2);
token_patterns_macros::tp!(Er3, er_3);
token_patterns_macros::tp!(FieldAttrSerdeSkipSerializingIfOptIsNone, #[serde(skip_serializing_if = "Option::is_none")]);
token_patterns_macros::tp_batch!(
    (Bool, bool),
    (U8, u8),
    (U16, u16),
    (U32, u32),
    (U64, u64),
    (I8, i8),
    (I16, i16),
    (I32, i32),
    (I64, i64),
    (F32, f32),
    (F64, f64),
    (UuidUuid, uuid::Uuid),
    (StdFmtDisplay, std::fmt::Display)
);
token_patterns_macros::tp_parts!(CrateDfltSomeOneEl, crate_path_ts(), dflt_some_one_el_ucc());
token_patterns_macros::tp_parts!(
    CrateDfltSomeOneElCall,
    crate_path_ts(),
    dflt_some_one_el_ucc(),
    path_dflt_some_one_el_call()
);
token_patterns_macros::tp_parts!(PgCrudDfltSomeOneEl, pg_crud(), dflt_some_one_el_ucc());
token_patterns_macros::tp_parts!(
    PgCrudDfltSomeOneElCall,
    PgCrudDfltSomeOneEl,
    path_dflt_some_one_el_call()
);
token_patterns_macros::tp_parts!(
    PgCrudCmnDfltSomeOneEl,
    pg_crud_cmn(),
    dflt_some_one_el_ucc()
);
token_patterns_macros::tp_parts!(
    PgCrudCmnDfltSomeOneElCall,
    PgCrudCmnDfltSomeOneEl,
    path_dflt_some_one_el_call()
);
token_patterns_macros::tp_parts!(
    CrateAllEnumVrtsArrDfltSomeOneEl,
    crate_path_ts(),
    all_vrts_dflt_some_one_el_ucc()
);
token_patterns_macros::tp_parts!(
    CrateAllEnumVrtsArrDfltSomeOneElCall,
    CrateAllEnumVrtsArrDfltSomeOneEl,
    path_all_vrts_dflt_some_one_el_call()
);
token_patterns_macros::tp_parts!(
    PgCrudAllEnumVrtsArrDfltSomeOneEl,
    pg_crud(),
    all_vrts_dflt_some_one_el_ucc()
);
token_patterns_macros::tp_parts!(
    PgCrudAllEnumVrtsArrDfltSomeOneElCall,
    PgCrudAllEnumVrtsArrDfltSomeOneEl,
    path_all_vrts_dflt_some_one_el_call()
);
token_patterns_macros::tp_parts!(
    PgCrudCmnAllEnumVrtsArrDfltSomeOneEl,
    pg_crud_cmn(),
    all_vrts_dflt_some_one_el_ucc()
);
token_patterns_macros::tp_parts!(
    PgCrudCmnAllEnumVrtsArrDfltSomeOneElCall,
    PgCrudCmnAllEnumVrtsArrDfltSomeOneEl,
    path_all_vrts_dflt_some_one_el_call()
);
token_patterns_macros::tp_parts!(
    CrateDfltSomeOneElMaxPageSize,
    crate_path_ts(),
    dflt_some_one_el_max_page_size_ucc()
);
token_patterns_macros::tp_parts!(
    CrateDfltSomeOneElMaxPageSizeCall,
    crate_path_ts(),
    dflt_some_one_el_max_page_size_ucc(),
    path_dflt_some_one_el_max_page_size_call()
);
token_patterns_macros::tp_parts!(
    PgCrudDfltSomeOneElMaxPageSize,
    pg_crud(),
    dflt_some_one_el_max_page_size_ucc()
);
token_patterns_macros::tp_parts!(
    PgCrudDfltSomeOneElMaxPageSizeCall,
    PgCrudDfltSomeOneElMaxPageSize,
    path_dflt_some_one_el_max_page_size_call()
);
token_patterns_macros::tp_parts!(
    PgCrudCmnDfltSomeOneElMaxPageSize,
    pg_crud_cmn(),
    dflt_some_one_el_max_page_size_ucc()
);
token_patterns_macros::tp_parts!(
    PgCrudCmnDfltSomeOneElMaxPageSizeCall,
    PgCrudCmnDfltSomeOneElMaxPageSize,
    path_dflt_some_one_el_max_page_size_call()
);
token_patterns_macros::tp_parts!(
    CrateAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    crate_path_ts(),
    all_vrts_dflt_some_one_el_max_page_size_ucc()
);
token_patterns_macros::tp_parts!(
    CrateAllEnumVrtsArrDfltSomeOneElCallWithMaxPageSize,
    CrateAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    path_all_vrts_dflt_some_one_el_max_page_size_call()
);
token_patterns_macros::tp_parts!(
    PgCrudAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    pg_crud(),
    all_vrts_dflt_some_one_el_max_page_size_ucc()
);
token_patterns_macros::tp_parts!(
    PgCrudAllEnumVrtsArrDfltSomeOneElCallWithMaxPageSize,
    PgCrudAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    path_all_vrts_dflt_some_one_el_max_page_size_call()
);
token_patterns_macros::tp_parts!(
    PgCrudCmnAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    pg_crud_cmn(),
    all_vrts_dflt_some_one_el_max_page_size_ucc()
);
token_patterns_macros::tp_parts!(
    PgCrudCmnAllEnumVrtsArrDfltSomeOneElCallWithMaxPageSize,
    PgCrudCmnAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    path_all_vrts_dflt_some_one_el_max_page_size_call()
);
token_patterns_macros::tp!(MustUse, #[must_use]);
token_patterns_macros::tp!(AllowClippyArbitrarySrcItemOrdering, #[allow(clippy::arbitrary_source_item_ordering)]);
token_patterns_macros::tp!(NoneTs, None);
token_patterns_macros::ts_path_fn!(
    path_all_vrts_dflt_some_one_el_max_page_size_call,
    ::all_vrts_dflt_some_one_el_max_page_size()
);
token_patterns_macros::ts_path_fn!(dflt_some_one_el_max_page_size_ucc, DfltSomeOneElMaxPageSize);
token_patterns_macros::ts_path_fn!(crate_path_ts, crate::);
token_patterns_macros::ts_path_fn!(pg_crud, pg_crud::);
token_patterns_macros::ts_path_fn!(pg_crud_cmn, pg_crud_cmn::);
token_patterns_macros::ts_path_fn!(dflt_some_one_el_ucc, DfltSomeOneEl);
token_patterns_macros::ts_path_fn!(all_vrts_dflt_some_one_el_ucc, AllEnumVrtsArrDfltSomeOneEl);
token_patterns_macros::ts_path_fn!(path_dflt_some_one_el_call, ::dflt_some_one_el());
token_patterns_macros::ts_path_fn!(
    path_dflt_some_one_el_max_page_size_call,
    ::dflt_some_one_el_max_page_size()
);
token_patterns_macros::ts_path_fn!(
    all_vrts_dflt_some_one_el_max_page_size_ucc,
    AllEnumVrtsArrDfltSomeOneElMaxPageSize
);
token_patterns_macros::ts_path_fn!(
    path_all_vrts_dflt_some_one_el_call,
    ::all_vrts_dflt_some_one_el()
);
struct TokensMut<'tokens_lt>(&'tokens_lt mut proc_macro2::TokenStream);
fn append_tokens(tokens: &mut TokensMut<'_>, part: impl quote::ToTokens) {
    part.to_tokens(&mut *tokens.0);
}
#[cfg(test)]
mod tests {
    fn assert_tokens_eq(actual: impl quote::ToTokens, expected: impl quote::ToTokens) {
        assert_eq!(
            quote::quote! {#actual}.to_string(),
            quote::quote! {#expected}.to_string()
        );
    }
    #[test]
    fn tp_struct_outputs_expected_tokens() {
        assert_tokens_eq(super::SqlxAcquire, quote::quote! {sqlx::Acquire});
        assert_tokens_eq(
            super::DeriveDebugCloneCopy,
            quote::quote! {#[derive(Debug, Clone, Copy, Optml)]},
        );
    }
    #[test]
    fn tp_parts_struct_outputs_expected_tokens() {
        assert_tokens_eq(
            super::CrateDfltSomeOneEl,
            quote::quote! {crate::DfltSomeOneEl},
        );
        assert_tokens_eq(
            super::CrateDfltSomeOneElCall,
            quote::quote! {crate::DfltSomeOneEl::dflt_some_one_el()},
        );
    }
    #[test]
    fn tp_batch_struct_outputs_expected_tokens() {
        assert_tokens_eq(super::Bool, quote::quote! {bool});
    }
    #[test]
    fn ts_path_fn_outputs_expected_tokens() {
        assert_tokens_eq(super::pg_crud(), quote::quote! {pg_crud::});
        assert_tokens_eq(super::pg_crud_cmn(), quote::quote! {pg_crud_cmn::});
    }
    #[test]
    fn path_helper_outputs_expected_tokens() {
        assert_tokens_eq(
            super::path_dflt_some_one_el_call(),
            quote::quote! {::dflt_some_one_el()},
        );
    }
}
