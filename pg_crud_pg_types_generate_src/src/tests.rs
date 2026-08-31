#[test]
fn model_can_be_parsed_and_validated_without_emitting_source() {
    let input = quote::quote! {{
        "pg_table_cols_write_into_file": "False",
        "whole_write_into_file": "False",
        "variant": {"Subset": ["I16AsInt2", "StringAsText"]}
    }};
    let parsed = crate::parse_generate_pg_types::parse_generate_pg_types(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
    )
    .expect(
        "35a0f719 model_can_be_parsed_and_validated_without_emitting_source invariant must hold",
    );
    let built = crate::build_generate_pg_types::build_generate_pg_types(parsed).expect(
        "3c8d514f model_can_be_parsed_and_validated_without_emitting_source invariant must hold",
    );
    let validated = crate::validate_generate_pg_types::validate_generate_pg_types(built).expect(
        "b24816de model_can_be_parsed_and_validated_without_emitting_source invariant must hold",
    );
    assert_eq!(usize::from(validated.entry_count()), 2usize);
}

#[test]
fn malformed_config_is_a_typed_parse_error() {
    let input = quote::quote! {{"variant": "MissingFields"}};
    assert!(matches!(
        crate::parse_generate_pg_types::parse_generate_pg_types(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        ),
        Err(crate::generate_pg_types_pipeline_error::GeneratePgTypesPipelineError::Parse(_error))
    ));
}

#[test]
fn generated_type_list_deserialization_rejects_too_many_entries() {
    let serialized = serde_json::to_string(&vec![
        crate::pg_type_catalog_kind::PgTypeCatalogKind::I16AsInt2;
        crate::generate_pg_types_max_len::GENERATE_PG_TYPES_MAX_LEN
            + constants_usize::ONE
    ])
    .expect(
        "7cd2e0af generated_type_list_deserialization_rejects_too_many_entries invariant must hold",
    );
    let _error = serde_json::from_str::<crate::generate_pg_types::GeneratePgTypes>(&serialized)
        .expect_err(constants_str::VALUE_28B750CB);
}
