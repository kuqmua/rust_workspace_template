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
        let mut accumulated = quote::quote! { existing_prefix:: };
        quote::ToTokens::to_tokens(&actual, &mut accumulated);
        assert_eq!(
            accumulated.to_string(),
            quote::quote! { existing_prefix::#expected }.to_string()
        );
    }
    #[test]
    fn test_token_pattern_struct_outputs_expected_tokens() {
        assert_tokens_eq(super::SqlxAcquire, quote::quote! {sqlx::Acquire});
        assert_tokens_eq(
            super::AxumExtractRejectionJsonRejection,
            quote::quote! {axum::extract::rejection::JsonRejection},
        );
        assert_tokens_eq(
            super::AxumResponseIntoResponse,
            quote::quote! {axum::response::IntoResponse},
        );
        assert_tokens_eq(super::ReqwestError, quote::quote! {reqwest::Error});
        assert_tokens_eq(
            super::ReqwestHeaderHeaderMap,
            quote::quote! {reqwest::header::HeaderMap},
        );
        assert_tokens_eq(super::HttpStatusCode, quote::quote! {http::StatusCode});
        assert_tokens_eq(super::SqlxRow, quote::quote! {sqlx::Row});
        assert_tokens_eq(super::SerdeSerialize, quote::quote! {serde::Serialize});
        assert_tokens_eq(super::SerdeDeserialize, quote::quote! {serde::Deserialize});
        assert_tokens_eq(super::UtoipaToSchema, quote::quote! {utoipa::ToSchema});
        assert_tokens_eq(
            super::SchemarsJsonSchema,
            quote::quote! {schemars::JsonSchema},
        );
        assert_tokens_eq(
            super::LocationLibLocation,
            quote::quote! {location_lib::location::Location},
        );
        assert_tokens_eq(super::ThiserrorError, quote::quote! {thiserror::Error});
        assert_tokens_eq(super::Char, quote::quote! {char});
        assert_tokens_eq(super::RefStr, quote::quote! {&str});
        assert_tokens_eq(super::StringTokenStream, quote::quote! {String});
        assert_tokens_eq(
            super::DeriveDebug,
            quote::quote! {#[derive(Debug, OptimalMemoryLayout)]},
        );
        assert_tokens_eq(
            super::DeriveDebugThiserrorLocation,
            quote::quote! {#[derive(Debug, thiserror::Error, proc_macro_location_derive_location::Location, OptimalMemoryLayout)]},
        );
        assert_tokens_eq(
            super::DeriveDebugUtoipaToSchema,
            quote::quote! {#[derive(Debug, utoipa::ToSchema, OptimalMemoryLayout)]},
        );
        assert_tokens_eq(
            super::DeriveDebugSerdeSerializeSerdeDeserialize,
            quote::quote! {#[derive(Debug, serde::Serialize, serde::Deserialize, OptimalMemoryLayout)]},
        );
        assert_tokens_eq(
            super::DeriveDebugSerdeSerializeSerdeDeserializeUtoipaToSchema,
            quote::quote! {#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, OptimalMemoryLayout)]},
        );
        assert_tokens_eq(
            super::StrSqlxColumnIndex,
            quote::quote! {&'lt str: sqlx::ColumnIndex<R>,},
        );
        assert_tokens_eq(
            super::SqlxDecodeDecodeDatabase,
            quote::quote! {sqlx::decode::Decode<'lt, R::Database>},
        );
        assert_tokens_eq(
            super::SqlxTypesTypeDatabase,
            quote::quote! {sqlx::types::Type<R::Database>},
        );
        assert_tokens_eq(
            super::LocationLibLocationLocation,
            quote::quote! {location_lib::location::Location},
        );
        assert_tokens_eq(
            super::LocationSnakeCaseDoubleDotSpaceLocationLibLocationLocation,
            quote::quote! {location: location_lib::location::Location},
        );
        assert_tokens_eq(
            super::CoreDefault,
            quote::quote! {::core::default::Default::default()},
        );
        assert_tokens_eq(
            super::SqlxTypesTimeTimeMidnight,
            quote::quote! {sqlx::types::time::Time::MIDNIGHT},
        );
        assert_tokens_eq(
            super::SqlxTypesTimeOffsetDateTimeUnixEpoch,
            quote::quote! {sqlx::types::time::OffsetDateTime::UNIX_EPOCH},
        );
        assert_tokens_eq(super::Error0, quote::quote! {error_0});
        assert_tokens_eq(super::Error1, quote::quote! {error_1});
        assert_tokens_eq(super::Error2, quote::quote! {error_2});
        assert_tokens_eq(super::Error3, quote::quote! {error_3});
        assert_tokens_eq(super::MustUse, quote::quote! {#[must_use]});
        assert_tokens_eq(super::NoneTokenStream, quote::quote! {None});
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
    fn test_cross_crate_default_trait_patterns_keep_module_and_call_paths() {
        assert_tokens_eq(
            super::PgCrudCommonDefaultSomeOneElement,
            quote::quote! {pg_crud_common::default_some_one_element::DefaultSomeOneElement},
        );
        assert_tokens_eq(
            super::PgCrudCommonDefaultSomeOneElementCall,
            quote::quote! {pg_crud_common::default_some_one_element::DefaultSomeOneElement::default_some_one_element()},
        );
        assert_tokens_eq(
            super::CrateAllEnumVariantsArrayDefaultSomeOneElementCall,
            quote::quote! {crate::AllEnumVariantsArrayDefaultSomeOneElement::all_variants_default_some_one_element()},
        );
        assert_tokens_eq(
            super::PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementCall,
            quote::quote! {pg_crud_common::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement::all_variants_default_some_one_element()},
        );
    }

    #[test]
    fn test_maximum_page_size_patterns_select_matching_traits_and_methods() {
        assert_tokens_eq(
            super::CrateDefaultSomeOneElementMaxPageSize,
            quote::quote! {crate::DefaultSomeOneElementMaxPageSize},
        );
        assert_tokens_eq(
            super::CrateDefaultSomeOneElementMaxPageSizeCall,
            quote::quote! {crate::DefaultSomeOneElementMaxPageSize::default_some_one_element_max_page_size()},
        );
        assert_tokens_eq(
            super::PgCrudCommonDefaultSomeOneElementMaxPageSizeCall,
            quote::quote! {pg_crud_common::default_some_one_element_max_page_size::DefaultSomeOneElementMaxPageSize::default_some_one_element_max_page_size()},
        );
        assert_tokens_eq(
            super::CrateAllEnumVariantsArrayDefaultSomeOneElementCallWithMaxPageSize,
            quote::quote! {crate::AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize::all_variants_default_some_one_element_max_page_size()},
        );
        assert_tokens_eq(
            super::PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementCallWithMaxPageSize,
            quote::quote! {pg_crud_common::all_enum_variants_array_default_some_one_element_max_page_size::AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize::all_variants_default_some_one_element_max_page_size()},
        );
    }

    #[test]
    fn test_token_pattern_batch_struct_outputs_expected_tokens() {
        assert_tokens_eq(super::Bool, quote::quote! {bool});
        assert_tokens_eq(super::U8, quote::quote! {u8});
        assert_tokens_eq(super::U16, quote::quote! {u16});
        assert_tokens_eq(super::U32, quote::quote! {u32});
        assert_tokens_eq(super::U64, quote::quote! {u64});
        assert_tokens_eq(super::I8, quote::quote! {i8});
        assert_tokens_eq(super::I16, quote::quote! {i16});
        assert_tokens_eq(super::I32, quote::quote! {i32});
        assert_tokens_eq(super::I64, quote::quote! {i64});
        assert_tokens_eq(super::F32, quote::quote! {f32});
        assert_tokens_eq(super::F64, quote::quote! {f64});
        assert_tokens_eq(super::UuidUuid, quote::quote! {uuid::Uuid});
        assert_tokens_eq(super::StdFmtDisplay, quote::quote! {std::fmt::Display});
    }
    #[test]
    fn test_token_stream_path_function_outputs_expected_tokens() {
        assert_tokens_eq(crate::pg_crud_common(), quote::quote! {pg_crud_common::});
        assert_tokens_eq(crate::crate_path_token_stream(), quote::quote! {crate::});
        assert_tokens_eq(
            crate::default_some_one_element_upper_camel_case(),
            quote::quote! {DefaultSomeOneElement},
        );
        assert_tokens_eq(
            crate::all_variants_default_some_one_element_upper_camel_case(),
            quote::quote! {AllEnumVariantsArrayDefaultSomeOneElement},
        );
        assert_tokens_eq(
            crate::default_some_one_element_max_page_size_upper_camel_case(),
            quote::quote! {DefaultSomeOneElementMaxPageSize},
        );
        assert_tokens_eq(
            crate::all_variants_default_some_one_element_max_page_size_upper_camel_case(),
            quote::quote! {AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize},
        );
    }
    #[test]
    fn test_path_helper_outputs_expected_tokens() {
        assert_tokens_eq(
            crate::path_default_some_one_element_max_page_size_call(),
            quote::quote! {::default_some_one_element_max_page_size()},
        );
        assert_tokens_eq(
            crate::path_all_variants_default_some_one_element_call(),
            quote::quote! {::all_variants_default_some_one_element()},
        );
        assert_tokens_eq(
            crate::path_all_variants_default_some_one_element_max_page_size_call(),
            quote::quote! {::all_variants_default_some_one_element_max_page_size()},
        );
        assert_tokens_eq(
            crate::path_default_some_one_element_call(),
            quote::quote! {::default_some_one_element()},
        );
    }
    #[test]
    fn test_remaining_composite_trait_patterns_preserve_exact_module_paths() {
        assert_tokens_eq(
            super::CrateAllEnumVariantsArrayDefaultSomeOneElement,
            quote::quote! { crate::AllEnumVariantsArrayDefaultSomeOneElement },
        );
        assert_tokens_eq(
            super::PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElement,
            quote::quote! { pg_crud_common::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement },
        );
        assert_tokens_eq(
            super::PgCrudCommonDefaultSomeOneElementMaxPageSize,
            quote::quote! { pg_crud_common::default_some_one_element_max_page_size::DefaultSomeOneElementMaxPageSize },
        );
        assert_tokens_eq(
            super::CrateAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
            quote::quote! { crate::AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize },
        );
        assert_tokens_eq(
            super::PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize,
            quote::quote! { pg_crud_common::all_enum_variants_array_default_some_one_element_max_page_size::AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize },
        );
    }
    #[test]
    fn test_token_attribute_patterns_preserve_predicate_and_specific_lint_reason() {
        let predicate = format!("{}::{}", stringify!(Option), stringify!(is_none));
        assert_tokens_eq(
            super::FieldAttrSerdeSkipSerializingIfOptionalIsNone,
            quote::quote! { #[serde(skip_serializing_if = #predicate)] },
        );
        let reason = stringify!(lib keeps declaration order aligned with generated layout or processing flow);
        assert_tokens_eq(
            super::AllowClippyArbitrarySrcItemOrdering,
            quote::quote! { #[allow(clippy::arbitrary_source_item_ordering, reason = #reason)] },
        );
    }
}
