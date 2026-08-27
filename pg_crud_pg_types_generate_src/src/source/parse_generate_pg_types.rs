use super::*;

pub fn parse_generate_pg_types(
    input: macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef<'_>,
) -> Result<ParsedGeneratePgTypesConfig, GeneratePgTypesPipelineError> {
    serde_json::from_str::<GeneratePgTypesConfig>(&input.as_ref().to_string())
        .map(ParsedGeneratePgTypesConfig)
        .map_err(|error| {
            GeneratePgTypesPipelineError::Parse(SerdeJsonGeneratePgTypesError::from(error))
        })
}
