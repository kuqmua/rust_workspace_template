pub fn parse_generate_where_filters(
    input: super::ProcMacro2GenerateWhereFiltersInput<'_>,
) -> Result<super::ParsedGenerateWhereFiltersConfig, super::GenerateWhereFiltersPipelineError> {
    serde_json::from_str(&input.as_ref().to_string()).map_err(|error| {
        super::GenerateWhereFiltersPipelineError::Parse(
            super::SerdeJsonGenerateWhereFiltersError::from(error),
        )
    })
}
