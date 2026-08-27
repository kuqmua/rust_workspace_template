use super::*;

pub fn validate_generate_pg_types(
    built: BuiltGeneratePgTypesModel,
) -> Result<ValidatedGeneratePgTypesConfig, GeneratePgTypesPipelineError> {
    Ok(ValidatedGeneratePgTypesConfig {
        config: built.config,
        entry_count: built.entry_count,
    })
}
