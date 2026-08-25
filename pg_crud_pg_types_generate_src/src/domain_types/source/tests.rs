#[test]
fn model_can_be_parsed_and_validated_without_emitting_source() {
    let input = quote::quote! {{
        "pg_table_cols_write_into_file": "False",
        "whole_write_into_file": "False",
        "variant": {"Subset": ["I16AsInt2", "StringAsText"]}
    }};
    let parsed = super::parse_generate_pg_types(
        macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(&input),
    )
    .expect(
        "35a0f719 model_can_be_parsed_and_validated_without_emitting_source invariant must hold",
    );
    let built = super::build_generate_pg_types(parsed).expect(
        "3c8d514f model_can_be_parsed_and_validated_without_emitting_source invariant must hold",
    );
    let validated = super::validate_generate_pg_types(built).expect(
        "b24816de model_can_be_parsed_and_validated_without_emitting_source invariant must hold",
    );
    assert_eq!(usize::from(validated.entry_count()), 2usize);
}

#[test]
fn malformed_config_is_a_typed_parse_error() {
    let input = quote::quote! {{"variant": "MissingFields"}};
    assert!(matches!(
        super::parse_generate_pg_types(
            macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(&input),
        ),
        Err(super::GeneratePgTypesPipelineError::Parse(_error))
    ));
}

#[test]
fn generated_type_list_deserialization_rejects_too_many_entries() {
    let serialized = serde_json::to_string(&vec![
        super::PgType::I16AsInt2;
        super::GENERATE_PG_TYPES_MAX_LEN
            + constants_usize::ONE
    ])
    .expect(
        "7cd2e0af generated_type_list_deserialization_rejects_too_many_entries invariant must hold",
    );
    let _error = serde_json::from_str::<super::GeneratePgTypes>(&serialized)
        .expect_err(constants_str::VALUE_28B750CB);
}
