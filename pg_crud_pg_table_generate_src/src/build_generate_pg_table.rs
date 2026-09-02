pub fn build_generate_pg_table(
    syn_parsed_generate_pg_table_input: crate::syn_parsed_generate_pg_table_input::SynParsedGeneratePgTableInput,
) -> Result<
    crate::syn_built_generate_pg_table_input::SynBuiltGeneratePgTableInput,
    crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError,
> {
    let _shape = crate::struct_shape::struct_shape(
        workspace_macro_helpers::syn_derive_input_ref::SynDeriveInputRef::from(
            syn_parsed_generate_pg_table_input.get_inner(),
        ),
    )
    .map_err(|error| {
        crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError::Build(
            crate::syn_generate_pg_table_pipeline_error::SynGeneratePgTablePipelineError::from(
                error,
            ),
        )
    })?;
    let input = crate::syn_generate_pg_table_model_input::SynGeneratePgTableModelInput::from(
        syn::DeriveInput::from(syn_parsed_generate_pg_table_input),
    );
    let field_count = match &input.data {
        syn::Data::Struct(data) => data.fields.iter().count(),
        syn::Data::Enum(_) | syn::Data::Union(_) => constants_usize::ZERO,
    };
    Ok(
        crate::syn_built_generate_pg_table_input::SynBuiltGeneratePgTableInput::from(
            crate::generate_pg_table_model::GeneratePgTableModel::new(
                crate::generate_pg_table_field_count::GeneratePgTableFieldCount::from(field_count),
                input,
            ),
        ),
    )
}
