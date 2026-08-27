pub fn build_generate_pg_table(
    parsed: super::SynParsedGeneratePgTableInput,
) -> Result<super::SynBuiltGeneratePgTableInput, super::GeneratePgTablePipelineError> {
    let _shape = crate::domain_types::struct_shape::struct_shape(
        workspace_macro_helpers::domain_types::SynDeriveInputRef::from(&parsed.0),
    )
    .map_err(|error| {
        super::GeneratePgTablePipelineError::Build(super::SynGeneratePgTablePipelineError::from(
            error,
        ))
    })?;
    let input = crate::domain_types::table::syn_generate_pg_table_model_input::SynGeneratePgTableModelInput::from(parsed.0);
    let field_count = match &input.data {
        syn::Data::Struct(data) => data.fields.iter().count(),
        syn::Data::Enum(_) | syn::Data::Union(_) => constants_usize::ZERO,
    };
    Ok(super::SynBuiltGeneratePgTableInput::from(
        crate::domain_types::table::GeneratePgTableModel {
            field_count: crate::domain_types::table::GeneratePgTableFieldCount::from(field_count),
            input,
        },
    ))
}
