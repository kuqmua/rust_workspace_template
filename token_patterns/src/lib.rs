pub mod proc_macro2_tokens_mut;

proc_macro_token_patterns_tp::tp!(SqlxAcquire, sqlx::Acquire);
proc_macro_token_patterns_tp::tp!(
    AxumExtractRejectionJsonRejection,
    axum::extract::rejection::JsonRejection
);
proc_macro_token_patterns_tp::tp!(AxumResponseIntoResponse, axum::response::IntoResponse);
proc_macro_token_patterns_tp::tp!(ReqwestError, reqwest::Error);
proc_macro_token_patterns_tp::tp!(ReqwestHeaderHeaderMap, reqwest::header::HeaderMap);
proc_macro_token_patterns_tp::tp!(HttpStatusCode, http::StatusCode);
proc_macro_token_patterns_tp::tp!(SqlxRow, sqlx::Row);
proc_macro_token_patterns_tp::tp!(SerdeSerialize, serde::Serialize);
proc_macro_token_patterns_tp::tp!(SerdeDeserialize, serde::Deserialize);
proc_macro_token_patterns_tp::tp!(UtoipaToSchema, utoipa::ToSchema);
proc_macro_token_patterns_tp::tp!(SchemarsJsonSchema, schemars::JsonSchema);
proc_macro_token_patterns_tp::tp!(LocationLibLocation, location_lib::location::Location);
proc_macro_token_patterns_tp::tp!(ThiserrorError, thiserror::Error);
proc_macro_token_patterns_tp::tp!(Char, char);
proc_macro_token_patterns_tp::tp!(RefStr, &str);
proc_macro_token_patterns_tp::tp!(StringTokenStream, String);
proc_macro_token_patterns_tp::tp!(DeriveDebug, #[derive(Debug, OptimalMemoryLayout)]);
proc_macro_token_patterns_tp::tp!(DeriveDebugThiserrorLocation, #[derive(Debug, thiserror::Error, proc_macro_location_derive_location::Location, OptimalMemoryLayout)]);
proc_macro_token_patterns_tp::tp!(DeriveDebugUtoipaToSchema, #[derive(Debug, utoipa::ToSchema, OptimalMemoryLayout)]);
proc_macro_token_patterns_tp::tp!(DeriveDebugSerdeSerializeSerdeDeserialize, #[derive(Debug, serde::Serialize, serde::Deserialize, OptimalMemoryLayout)]);
proc_macro_token_patterns_tp::tp!(DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema, #[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, OptimalMemoryLayout)]);
proc_macro_token_patterns_tp::tp!(DeriveDebugCloneCopy, #[derive(Debug, Clone, Copy, OptimalMemoryLayout)]);
proc_macro_token_patterns_tp::tp!(StrSqlxColumnIndex, &'lt str: sqlx::ColumnIndex<R>,);
proc_macro_token_patterns_tp::tp!(
    SqlxDecodeDecodeDatabase,
    sqlx::decode::Decode<'lt, R::Database>
);
proc_macro_token_patterns_tp::tp!(SqlxTypesTypeDatabase, sqlx::types::Type<R::Database>);
proc_macro_token_patterns_tp::tp!(
    LocationLibLocationLocation,
    location_lib::location::Location
);
proc_macro_token_patterns_tp::tp!(LocationSnakeCaseDoubleDotSpaceLocationLibLocationLocation, location: location_lib::location::Location);
proc_macro_token_patterns_tp::tp!(CoreDefault, ::core::default::Default::default());
proc_macro_token_patterns_tp::tp!(SqlxTypesTimeTimeMidnight, sqlx::types::time::Time::MIDNIGHT);
proc_macro_token_patterns_tp::tp!(
    SqlxTypesTimeOffsetDateTimeUnixEpoch,
    sqlx::types::time::OffsetDateTime::UNIX_EPOCH
);
proc_macro_token_patterns_tp::tp!(Error0, error_0);
proc_macro_token_patterns_tp::tp!(Error1, error_1);
proc_macro_token_patterns_tp::tp!(Error2, error_2);
proc_macro_token_patterns_tp::tp!(Error3, error_3);
proc_macro_token_patterns_tp::tp!(FieldAttrSerdeSkipSerializingIfOptionalIsNone, #[serde(skip_serializing_if = "Option::is_none")]);
proc_macro_token_patterns_tp_batch::tp_batch!(
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
proc_macro_token_patterns_tp_parts::tp_parts!(
    CrateDefaultSomeOneElement,
    crate_path_token_stream(),
    default_some_one_element_upper_camel_case()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    CrateDefaultSomeOneElementCall,
    crate_path_token_stream(),
    default_some_one_element_upper_camel_case(),
    path_default_some_one_element_call()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    PgCrudCommonDefaultSomeOneElement,
    quote::quote! {pg_crud_common::default_some_one_element::},
    default_some_one_element_upper_camel_case()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    PgCrudCommonDefaultSomeOneElementCall,
    PgCrudCommonDefaultSomeOneElement,
    path_default_some_one_element_call()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    CrateAllEnumVariantsArrayDefaultSomeOneElement,
    crate_path_token_stream(),
    all_variants_default_some_one_element_upper_camel_case()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    CrateAllEnumVariantsArrayDefaultSomeOneElementCall,
    CrateAllEnumVariantsArrayDefaultSomeOneElement,
    path_all_variants_default_some_one_element_call()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElement,
    quote::quote! {pg_crud_common::all_enum_variants_array_default_some_one_element::},
    all_variants_default_some_one_element_upper_camel_case()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementCall,
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElement,
    path_all_variants_default_some_one_element_call()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    CrateDefaultSomeOneElementMaxPageSize,
    crate_path_token_stream(),
    default_some_one_element_max_page_size_upper_camel_case()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    CrateDefaultSomeOneElementMaxPageSizeCall,
    crate_path_token_stream(),
    default_some_one_element_max_page_size_upper_camel_case(),
    path_default_some_one_element_max_page_size_call()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    PgCrudCommonDefaultSomeOneElementMaxPageSize,
    quote::quote! {pg_crud_common::default_some_one_element_max_page_size::},
    default_some_one_element_max_page_size_upper_camel_case()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    PgCrudCommonDefaultSomeOneElementMaxPageSizeCall,
    PgCrudCommonDefaultSomeOneElementMaxPageSize,
    path_default_some_one_element_max_page_size_call()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    CrateAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
    crate_path_token_stream(),
    all_variants_default_some_one_element_max_page_size_upper_camel_case()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    CrateAllEnumVariantsArrayDefaultSomeOneElementCallWithMaxPageSize,
    CrateAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
    path_all_variants_default_some_one_element_max_page_size_call()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
    quote::quote! {pg_crud_common::all_enum_variants_array_default_some_one_element_max_page_size::},
    all_variants_default_some_one_element_max_page_size_upper_camel_case()
);
proc_macro_token_patterns_tp_parts::tp_parts!(
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementCallWithMaxPageSize,
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
    path_all_variants_default_some_one_element_max_page_size_call()
);
proc_macro_token_patterns_tp::tp!(MustUse, #[must_use]);
proc_macro_token_patterns_tp::tp!(AllowClippyArbitrarySrcItemOrdering, #[allow(clippy::arbitrary_source_item_ordering, reason = "lib keeps declaration order aligned with generated layout or processing flow")]);
proc_macro_token_patterns_tp::tp!(NoneTokenStream, None);
proc_macro_token_patterns_ts_path_fn::ts_path_fn!(
    path_all_variants_default_some_one_element_max_page_size_call,
    ::all_variants_default_some_one_element_max_page_size()
);
proc_macro_token_patterns_ts_path_fn::ts_path_fn!(
    default_some_one_element_max_page_size_upper_camel_case,
    DefaultSomeOneElementMaxPageSize
);
proc_macro_token_patterns_ts_path_fn::ts_path_fn!(crate_path_token_stream, crate::);
proc_macro_token_patterns_ts_path_fn::ts_path_fn!(pg_crud_common, pg_crud_common::);
proc_macro_token_patterns_ts_path_fn::ts_path_fn!(
    default_some_one_element_upper_camel_case,
    DefaultSomeOneElement
);
proc_macro_token_patterns_ts_path_fn::ts_path_fn!(
    all_variants_default_some_one_element_upper_camel_case,
    AllEnumVariantsArrayDefaultSomeOneElement
);
proc_macro_token_patterns_ts_path_fn::ts_path_fn!(
    path_default_some_one_element_call,
    ::default_some_one_element()
);
proc_macro_token_patterns_ts_path_fn::ts_path_fn!(
    path_default_some_one_element_max_page_size_call,
    ::default_some_one_element_max_page_size()
);
proc_macro_token_patterns_ts_path_fn::ts_path_fn!(
    all_variants_default_some_one_element_max_page_size_upper_camel_case,
    AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize
);
proc_macro_token_patterns_ts_path_fn::ts_path_fn!(
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
