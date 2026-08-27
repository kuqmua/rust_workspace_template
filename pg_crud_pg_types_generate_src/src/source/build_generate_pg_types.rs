use super::*;

pub fn build_generate_pg_types(
    parsed: ParsedGeneratePgTypesConfig,
) -> Result<BuiltGeneratePgTypesModel, GeneratePgTypesPipelineError> {
    let entry_count = PgTypesModelEntryCount::from(match &parsed.0.variant {
        GeneratePgTypesConfigVariant::All => <PgType as strum::IntoEnumIterator>::iter().count(),
        GeneratePgTypesConfigVariant::Concrete(records) => records.len(),
        GeneratePgTypesConfigVariant::Subset(types) => types.len(),
    });
    Ok(BuiltGeneratePgTypesModel {
        config: parsed.0,
        entry_count,
    })
}
