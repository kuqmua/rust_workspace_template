#[test]
fn test_validation_rejects_non_struct_input_without_emitting_source() {
    let input = quote::quote! { enum NotATable { Value } };
    let parsed = crate::parse_generate_pg_table::parse_generate_pg_table(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
    )
    .expect(constants_str::DIAGNOSTIC_5D4F86A1);
    assert!(matches!(
        crate::build_generate_pg_table::build_generate_pg_table(parsed),
        Err(crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError::Build(_error))
    ));
}

#[test]
fn test_build_stage_exposes_typed_model_without_emitting_source() {
    let input = quote::quote! { struct Table { id: i64, name: String } };
    let parsed = crate::parse_generate_pg_table::parse_generate_pg_table(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
    )
    .expect(constants_str::DIAGNOSTIC_0F8B43D2);
    let built = crate::build_generate_pg_table::build_generate_pg_table(parsed)
        .expect(constants_str::DIAGNOSTIC_A715E9C4);
    assert_eq!(usize::from(built.model().field_count()), 2usize);
}

#[test]
fn test_validation_rejects_empty_table_model_without_emitting_source() {
    let input = quote::quote! { struct EmptyTable; };
    let parsed = crate::parse_generate_pg_table::parse_generate_pg_table(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
    )
    .expect(constants_str::DIAGNOSTIC_67D029AB);
    let built = crate::build_generate_pg_table::build_generate_pg_table(parsed)
        .expect(constants_str::DIAGNOSTIC_C15B8F34);
    assert!(matches!(
        crate::validate_generate_pg_table::validate_generate_pg_table(built),
        Err(
            crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError::Validate(_error)
        )
    ));
}
