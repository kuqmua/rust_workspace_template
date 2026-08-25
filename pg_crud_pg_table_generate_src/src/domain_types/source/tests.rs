#[test]
fn validation_rejects_non_struct_input_without_emitting_source() {
    let input = quote::quote! { enum NotATable { Value } };
    let parsed = crate::domain_types::pipeline::parse_generate_pg_table(
        macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(&input),
    )
    .expect(
        "5d4f86a1 validation_rejects_non_struct_input_without_emitting_source invariant must hold",
    );
    assert!(matches!(
        crate::domain_types::pipeline::build_generate_pg_table(parsed),
        Err(crate::domain_types::pipeline::GeneratePgTablePipelineError::Build(_error))
    ));
}

#[test]
fn build_stage_exposes_typed_model_without_emitting_source() {
    let input = quote::quote! { struct Table { id: i64, name: String } };
    let parsed = crate::domain_types::pipeline::parse_generate_pg_table(
        macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(&input),
    )
    .expect("0f8b43d2 build_stage_exposes_typed_model_without_emitting_source invariant must hold");
    let built = crate::domain_types::pipeline::build_generate_pg_table(parsed).expect(
        "a715e9c4 build_stage_exposes_typed_model_without_emitting_source invariant must hold",
    );
    assert_eq!(usize::from(built.model().field_count()), 2usize);
}

#[test]
fn validation_rejects_empty_table_model_without_emitting_source() {
    let input = quote::quote! { struct EmptyTable; };
    let parsed = crate::domain_types::pipeline::parse_generate_pg_table(
        macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(&input),
    )
    .expect(
        "67d029ab validation_rejects_empty_table_model_without_emitting_source invariant must hold",
    );
    let built = crate::domain_types::pipeline::build_generate_pg_table(parsed).expect(
        "c15b8f34 validation_rejects_empty_table_model_without_emitting_source invariant must hold",
    );
    assert!(matches!(
        crate::domain_types::pipeline::validate_generate_pg_table(built),
        Err(crate::domain_types::pipeline::GeneratePgTablePipelineError::Validate(_error))
    ));
}
