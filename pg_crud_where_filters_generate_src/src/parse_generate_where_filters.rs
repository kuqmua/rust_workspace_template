pub fn parse_generate_where_filters(
    input: crate::source::ProcMacro2GenerateWhereFiltersInput<'_>,
) -> Result<
    crate::source::ParsedGenerateWhereFiltersConfig,
    crate::source::GenerateWhereFiltersPipelineError,
> {
    serde_json::from_str(&input.as_ref().to_string()).map_err(|error| {
        crate::source::GenerateWhereFiltersPipelineError::Parse(
            crate::source::SerdeJsonGenerateWhereFiltersError::from(error),
        )
    })
}
