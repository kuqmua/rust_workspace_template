pub mod proc_macro2_tokens_mut;

proc_macro_token_patterns_token_pattern::token_pattern!(SqlxAcquire, sqlx::Acquire);
proc_macro_token_patterns_token_pattern::token_pattern!(
    AxumExtractRejectionJsonRejection,
    axum::extract::rejection::JsonRejection
);
proc_macro_token_patterns_token_pattern::token_pattern!(
    AxumResponseIntoResponse,
    axum::response::IntoResponse
);
proc_macro_token_patterns_token_pattern::token_pattern!(ReqwestError, reqwest::Error);
proc_macro_token_patterns_token_pattern::token_pattern!(
    ReqwestHeaderHeaderMap,
    reqwest::header::HeaderMap
);
proc_macro_token_patterns_token_pattern::token_pattern!(HttpStatusCode, http::StatusCode);
proc_macro_token_patterns_token_pattern::token_pattern!(SqlxRow, sqlx::Row);
proc_macro_token_patterns_token_pattern::token_pattern!(SerdeSerialize, serde::Serialize);
proc_macro_token_patterns_token_pattern::token_pattern!(SerdeDeserialize, serde::Deserialize);
proc_macro_token_patterns_token_pattern::token_pattern!(UtoipaToSchema, utoipa::ToSchema);
proc_macro_token_patterns_token_pattern::token_pattern!(SchemarsJsonSchema, schemars::JsonSchema);
proc_macro_token_patterns_token_pattern::token_pattern!(
    LocationLibLocation,
    location_lib::location::Location
);
proc_macro_token_patterns_token_pattern::token_pattern!(ThiserrorError, thiserror::Error);
proc_macro_token_patterns_token_pattern::token_pattern!(Char, char);
proc_macro_token_patterns_token_pattern::token_pattern!(RefStr, &str);
proc_macro_token_patterns_token_pattern::token_pattern!(StringTokenStream, String);
proc_macro_token_patterns_token_pattern::token_pattern!(DeriveDebug, #[derive(Debug, OptimalMemoryLayout)]);
proc_macro_token_patterns_token_pattern::token_pattern!(DeriveDebugThiserrorLocation, #[derive(Debug, thiserror::Error, proc_macro_location_derive_location::Location, OptimalMemoryLayout)]);
proc_macro_token_patterns_token_pattern::token_pattern!(DeriveDebugUtoipaToSchema, #[derive(Debug, utoipa::ToSchema, OptimalMemoryLayout)]);
proc_macro_token_patterns_token_pattern::token_pattern!(DeriveDebugSerdeSerializeSerdeDeserialize, #[derive(Debug, serde::Serialize, serde::Deserialize, OptimalMemoryLayout)]);
proc_macro_token_patterns_token_pattern::token_pattern!(DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema, #[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, OptimalMemoryLayout)]);
proc_macro_token_patterns_token_pattern::token_pattern!(DeriveDebugCloneCopy, #[derive(Debug, Clone, Copy, OptimalMemoryLayout)]);
proc_macro_token_patterns_token_pattern::token_pattern!(StrSqlxColumnIndex, &'lt str: sqlx::ColumnIndex<R>,);
proc_macro_token_patterns_token_pattern::token_pattern!(
    SqlxDecodeDecodeDatabase,
    sqlx::decode::Decode<'lt, R::Database>
);
proc_macro_token_patterns_token_pattern::token_pattern!(
    SqlxTypesTypeDatabase,
    sqlx::types::Type<R::Database>
);
proc_macro_token_patterns_token_pattern::token_pattern!(
    LocationLibLocationLocation,
    location_lib::location::Location
);
proc_macro_token_patterns_token_pattern::token_pattern!(LocationSnakeCaseDoubleDotSpaceLocationLibLocationLocation, location: location_lib::location::Location);
proc_macro_token_patterns_token_pattern::token_pattern!(
    CoreDefault,
    ::core::default::Default::default()
);
proc_macro_token_patterns_token_pattern::token_pattern!(
    SqlxTypesTimeTimeMidnight,
    sqlx::types::time::Time::MIDNIGHT
);
proc_macro_token_patterns_token_pattern::token_pattern!(
    SqlxTypesTimeOffsetDateTimeUnixEpoch,
    sqlx::types::time::OffsetDateTime::UNIX_EPOCH
);
proc_macro_token_patterns_token_pattern::token_pattern!(Error0, error_0);
proc_macro_token_patterns_token_pattern::token_pattern!(Error1, error_1);
proc_macro_token_patterns_token_pattern::token_pattern!(Error2, error_2);
proc_macro_token_patterns_token_pattern::token_pattern!(Error3, error_3);
proc_macro_token_patterns_token_pattern::token_pattern!(FieldAttrSerdeSkipSerializingIfOptionalIsNone, #[serde(skip_serializing_if = "Option::is_none")]);
proc_macro_token_patterns_token_pattern_batch::token_pattern_batch!(
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
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    CrateDefaultSomeOneElement,
    crate_path_token_stream(),
    default_some_one_element_upper_camel_case()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    CrateDefaultSomeOneElementCall,
    crate_path_token_stream(),
    default_some_one_element_upper_camel_case(),
    path_default_some_one_element_call()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    PgCrudCommonDefaultSomeOneElement,
    quote::quote! {pg_crud_common::default_some_one_element::},
    default_some_one_element_upper_camel_case()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    PgCrudCommonDefaultSomeOneElementCall,
    PgCrudCommonDefaultSomeOneElement,
    path_default_some_one_element_call()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    CrateAllEnumVariantsArrayDefaultSomeOneElement,
    crate_path_token_stream(),
    all_variants_default_some_one_element_upper_camel_case()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    CrateAllEnumVariantsArrayDefaultSomeOneElementCall,
    CrateAllEnumVariantsArrayDefaultSomeOneElement,
    path_all_variants_default_some_one_element_call()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElement,
    quote::quote! {pg_crud_common::all_enum_variants_array_default_some_one_element::},
    all_variants_default_some_one_element_upper_camel_case()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementCall,
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElement,
    path_all_variants_default_some_one_element_call()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    CrateDefaultSomeOneElementMaxPageSize,
    crate_path_token_stream(),
    default_some_one_element_max_page_size_upper_camel_case()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    CrateDefaultSomeOneElementMaxPageSizeCall,
    crate_path_token_stream(),
    default_some_one_element_max_page_size_upper_camel_case(),
    path_default_some_one_element_max_page_size_call()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    PgCrudCommonDefaultSomeOneElementMaxPageSize,
    quote::quote! {pg_crud_common::default_some_one_element_max_page_size::},
    default_some_one_element_max_page_size_upper_camel_case()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    PgCrudCommonDefaultSomeOneElementMaxPageSizeCall,
    PgCrudCommonDefaultSomeOneElementMaxPageSize,
    path_default_some_one_element_max_page_size_call()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    CrateAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
    crate_path_token_stream(),
    all_variants_default_some_one_element_max_page_size_upper_camel_case()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    CrateAllEnumVariantsArrayDefaultSomeOneElementCallWithMaxPageSize,
    CrateAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
    path_all_variants_default_some_one_element_max_page_size_call()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
    quote::quote! {pg_crud_common::all_enum_variants_array_default_some_one_element_max_page_size::},
    all_variants_default_some_one_element_max_page_size_upper_camel_case()
);
proc_macro_token_patterns_token_pattern_parts::token_pattern_parts!(
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementCallWithMaxPageSize,
    PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
    path_all_variants_default_some_one_element_max_page_size_call()
);
proc_macro_token_patterns_token_pattern::token_pattern!(MustUse, #[must_use]);
proc_macro_token_patterns_token_pattern::token_pattern!(AllowClippyArbitrarySrcItemOrdering, #[allow(clippy::arbitrary_source_item_ordering, reason = "lib keeps declaration order aligned with generated layout or processing flow")]);
proc_macro_token_patterns_token_pattern::token_pattern!(NoneTokenStream, None);
proc_macro_token_patterns_token_stream_path_function::token_stream_path_function!(
    path_all_variants_default_some_one_element_max_page_size_call,
    ::all_variants_default_some_one_element_max_page_size()
);
proc_macro_token_patterns_token_stream_path_function::token_stream_path_function!(
    default_some_one_element_max_page_size_upper_camel_case,
    DefaultSomeOneElementMaxPageSize
);
proc_macro_token_patterns_token_stream_path_function::token_stream_path_function!(crate_path_token_stream, crate::);
proc_macro_token_patterns_token_stream_path_function::token_stream_path_function!(pg_crud_common, pg_crud_common::);
proc_macro_token_patterns_token_stream_path_function::token_stream_path_function!(
    default_some_one_element_upper_camel_case,
    DefaultSomeOneElement
);
proc_macro_token_patterns_token_stream_path_function::token_stream_path_function!(
    all_variants_default_some_one_element_upper_camel_case,
    AllEnumVariantsArrayDefaultSomeOneElement
);
proc_macro_token_patterns_token_stream_path_function::token_stream_path_function!(
    path_default_some_one_element_call,
    ::default_some_one_element()
);
proc_macro_token_patterns_token_stream_path_function::token_stream_path_function!(
    path_default_some_one_element_max_page_size_call,
    ::default_some_one_element_max_page_size()
);
proc_macro_token_patterns_token_stream_path_function::token_stream_path_function!(
    all_variants_default_some_one_element_max_page_size_upper_camel_case,
    AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize
);
proc_macro_token_patterns_token_stream_path_function::token_stream_path_function!(
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
    fn test_token_pattern_struct_outputs_expected_tokens() {
        assert_tokens_eq(super::SqlxAcquire, quote::quote! {sqlx::Acquire});
        assert_tokens_eq(
            super::DeriveDebugCloneCopy,
            quote::quote! {#[derive(Debug, Clone, Copy, OptimalMemoryLayout)]},
        );
    }
    #[test]
    fn test_token_pattern_parts_struct_outputs_expected_tokens() {
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
    fn test_token_pattern_batch_struct_outputs_expected_tokens() {
        assert_tokens_eq(super::Bool, quote::quote! {bool});
    }
    #[test]
    fn test_token_stream_path_function_outputs_expected_tokens() {
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
