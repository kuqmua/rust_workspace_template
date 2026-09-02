pub fn parse_generate_where_filters(
    proc_macro2_generate_where_filters_input: crate::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput<'_>,
) -> Result<
    crate::parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig,
    crate::generate_where_filters_pipeline_error::GenerateWhereFiltersPipelineError,
> {
    serde_json::from_str(&proc_macro2_generate_where_filters_input.as_ref().to_string()).map_err(|error| {
        crate::generate_where_filters_pipeline_error::GenerateWhereFiltersPipelineError::Parse(
            crate::serde_json_generate_where_filters_error::SerdeJsonGenerateWhereFiltersError::from(error),
        )
    })
}
