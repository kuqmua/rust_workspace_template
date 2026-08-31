pub mod proc_macro2_tokens_mut;

token_patterns_macros::tp!(SqlxAcquire, sqlx::Acquire);
token_patterns_macros::tp!(
    AxumExtractRejectionJsonRejection,
    axum::extract::rejection::JsonRejection
);
token_patterns_macros::tp!(AxumResIntoRes, axum::response::IntoResponse);
token_patterns_macros::tp!(ReqwestError, reqwest::Error);
token_patterns_macros::tp!(ReqwestHeaderHeaderMap, reqwest::header::HeaderMap);
token_patterns_macros::tp!(HttpStatusCode, http::StatusCode);
token_patterns_macros::tp!(SqlxRow, sqlx::Row);
token_patterns_macros::tp!(SerdeSerialize, serde::Serialize);
token_patterns_macros::tp!(SerdeDeserialize, serde::Deserialize);
token_patterns_macros::tp!(UtoipaToSchema, utoipa::ToSchema);
token_patterns_macros::tp!(SchemarsJsonSchema, schemars::JsonSchema);
token_patterns_macros::tp!(LocationLibLocation, location_lib::location::Location);
token_patterns_macros::tp!(ThiserrorError, thiserror::Error);
token_patterns_macros::tp!(Char, char);
token_patterns_macros::tp!(RefStr, &str);
token_patterns_macros::tp!(StringTokenStream, String);
token_patterns_macros::tp!(DeriveDebug, #[derive(Debug, OptimalMemoryLayout)]);
token_patterns_macros::tp!(DeriveDebugThiserrorLocation, #[derive(Debug, thiserror::Error, location::Location, OptimalMemoryLayout)]);
token_patterns_macros::tp!(DeriveDebugUtoipaToSchema, #[derive(Debug, utoipa::ToSchema, OptimalMemoryLayout)]);
token_patterns_macros::tp!(DeriveDebugSerdeSerializeSerdeDeserialize, #[derive(Debug, serde::Serialize, serde::Deserialize, OptimalMemoryLayout)]);
token_patterns_macros::tp!(DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema, #[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, OptimalMemoryLayout)]);
token_patterns_macros::tp!(DeriveDebugCloneCopy, #[derive(Debug, Clone, Copy, OptimalMemoryLayout)]);
token_patterns_macros::tp!(StrSqlxColumnIndex, &'lt str: sqlx::ColumnIndex<R>,);
token_patterns_macros::tp!(
    SqlxDecodeDecodeDatabase,
    sqlx::decode::Decode<'lt, R::Database>
);
token_patterns_macros::tp!(SqlxTypesTypeDatabase, sqlx::types::Type<R::Database>);
token_patterns_macros::tp!(
    LocationLibLocationLocation,
    location_lib::location::Location
);
token_patterns_macros::tp!(LocationSnakeCaseDoubleDotSpaceLocationLibLocationLocation, location: location_lib::location::Location);
token_patterns_macros::tp!(CoreDefault, ::core::default::Default::default());
token_patterns_macros::tp!(SqlxTypesTimeTimeMidnight, sqlx::types::time::Time::MIDNIGHT);
token_patterns_macros::tp!(
    SqlxTypesTimeOffsetDateTimeUnixEpoch,
    sqlx::types::time::OffsetDateTime::UNIX_EPOCH
);
token_patterns_macros::tp!(Error0, error_0);
token_patterns_macros::tp!(Error1, error_1);
token_patterns_macros::tp!(Error2, error_2);
token_patterns_macros::tp!(Error3, error_3);
token_patterns_macros::tp!(FieldAttrSerdeSkipSerializingIfOptionalIsNone, #[serde(skip_serializing_if = "Option::is_none")]);
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
token_patterns_macros::tp_parts!(
    CrateDefaultSomeOneElement,
    crate_path_token_stream(),
    default_some_one_element_upper_camel_case()
);
token_patterns_macros::tp_parts!(
    CrateDefaultSomeOneElementCall,
    crate_path_token_stream(),
    default_some_one_element_upper_camel_case(),
    path_default_some_one_element_call()
);
token_patterns_macros::tp_parts!(
    PgCrudCommonDefaultSomeOneElement,
    quote::quote! {pg_crud_common::default_some_one_element::},
    default_some_one_element_upper_camel_case()
);
token_patterns_macros::tp_parts!(
    PgCrudCommonDefaultSomeOneElementCall,
    PgCrudCommonDefaultSomeOneElement,
    path_default_some_one_element_call()
);
token_patterns_macros::tp_parts!(
    CrateAllEnumVariantsArrayDefaultSomeOneElement,
    crate_path_token_stream(),
    all_variants_default_some_one_element_upper_camel_case()
);
token_patterns_macros::tp_parts!(
    CrateAllEnumVariantsArrayDefaultSomeOneElementCall,
    CrateAllEnumVariantsArrayDefaultSomeOneElement,
    path_all_variants_default_some_one_element_call()
);
token_patterns_macros::tp_parts!(
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElement,
    quote::quote! {pg_crud_common::all_enum_variants_array_default_some_one_element::},
    all_variants_default_some_one_element_upper_camel_case()
);
token_patterns_macros::tp_parts!(
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementCall,
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElement,
    path_all_variants_default_some_one_element_call()
);
token_patterns_macros::tp_parts!(
    CrateDefaultSomeOneElementMaxPageSize,
    crate_path_token_stream(),
    default_some_one_element_max_page_size_upper_camel_case()
);
token_patterns_macros::tp_parts!(
    CrateDefaultSomeOneElementMaxPageSizeCall,
    crate_path_token_stream(),
    default_some_one_element_max_page_size_upper_camel_case(),
    path_default_some_one_element_max_page_size_call()
);
token_patterns_macros::tp_parts!(
    PgCrudCommonDefaultSomeOneElementMaxPageSize,
    quote::quote! {pg_crud_common::default_some_one_element_max_page_size::},
    default_some_one_element_max_page_size_upper_camel_case()
);
token_patterns_macros::tp_parts!(
    PgCrudCommonDefaultSomeOneElementMaxPageSizeCall,
    PgCrudCommonDefaultSomeOneElementMaxPageSize,
    path_default_some_one_element_max_page_size_call()
);
token_patterns_macros::tp_parts!(
    CrateAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
    crate_path_token_stream(),
    all_variants_default_some_one_element_max_page_size_upper_camel_case()
);
token_patterns_macros::tp_parts!(
    CrateAllEnumVariantsArrayDefaultSomeOneElementCallWithMaxPageSize,
    CrateAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
    path_all_variants_default_some_one_element_max_page_size_call()
);
token_patterns_macros::tp_parts!(
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
    quote::quote! {pg_crud_common::all_enum_variants_array_default_some_one_element_max_page_size::},
    all_variants_default_some_one_element_max_page_size_upper_camel_case()
);
token_patterns_macros::tp_parts!(
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementCallWithMaxPageSize,
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
    path_all_variants_default_some_one_element_max_page_size_call()
);
token_patterns_macros::tp!(MustUse, #[must_use]);
token_patterns_macros::tp!(AllowClippyArbitrarySrcItemOrdering, #[allow(clippy::arbitrary_source_item_ordering)]);
token_patterns_macros::tp!(NoneTokenStream, None);
token_patterns_macros::ts_path_fn!(
    path_all_variants_default_some_one_element_max_page_size_call,
    ::all_variants_default_some_one_element_max_page_size()
);
token_patterns_macros::ts_path_fn!(
    default_some_one_element_max_page_size_upper_camel_case,
    DefaultSomeOneElementMaxPageSize
);
token_patterns_macros::ts_path_fn!(crate_path_token_stream, crate::);
token_patterns_macros::ts_path_fn!(pg_crud_common, pg_crud_common::);
token_patterns_macros::ts_path_fn!(
    default_some_one_element_upper_camel_case,
    DefaultSomeOneElement
);
token_patterns_macros::ts_path_fn!(
    all_variants_default_some_one_element_upper_camel_case,
    AllEnumVariantsArrayDefaultSomeOneElement
);
token_patterns_macros::ts_path_fn!(
    path_default_some_one_element_call,
    ::default_some_one_element()
);
token_patterns_macros::ts_path_fn!(
    path_default_some_one_element_max_page_size_call,
    ::default_some_one_element_max_page_size()
);
token_patterns_macros::ts_path_fn!(
    all_variants_default_some_one_element_max_page_size_upper_camel_case,
    AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize
);
token_patterns_macros::ts_path_fn!(
    path_all_variants_default_some_one_element_call,
    ::all_variants_default_some_one_element()
);
#[cfg(test)]
mod tests {
    fn assert_tokens_eq(actual: impl quote::ToTokens, expected: impl quote::ToTokens) {
        assert_eq!(
            quote::quote! {#actual}.to_string(),
            quote::quote! {#expected}.to_string()
        );
    }
    #[test]
    fn test_tp_struct_outputs_expected_tokens() {
        assert_tokens_eq(super::SqlxAcquire, quote::quote! {sqlx::Acquire});
        assert_tokens_eq(
            super::DeriveDebugCloneCopy,
            quote::quote! {#[derive(Debug, Clone, Copy, OptimalMemoryLayout)]},
        );
    }
    #[test]
    fn test_tp_parts_struct_outputs_expected_tokens() {
        assert_tokens_eq(
            super::CrateDefaultSomeOneElement,
            quote::quote! {crate::DefaultSomeOneElement},
        );
        assert_tokens_eq(
            super::CrateDefaultSomeOneElementCall,
            quote::quote! {crate::DefaultSomeOneElement::default_some_one_element()},
        );
    }
    #[test]
    fn test_tp_batch_struct_outputs_expected_tokens() {
        assert_tokens_eq(super::Bool, quote::quote! {bool});
    }
    #[test]
    fn test_ts_path_fn_outputs_expected_tokens() {
        assert_tokens_eq(crate::pg_crud_common(), quote::quote! {pg_crud_common::});
    }
    #[test]
    fn test_path_helper_outputs_expected_tokens() {
        assert_tokens_eq(
            crate::path_default_some_one_element_call(),
            quote::quote! {::default_some_one_element()},
        );
    }
}
