pub fn build_generate_pg_types(
    parsed: crate::parsed_generate_pg_types_config::ParsedGeneratePgTypesConfig,
) -> Result<
    crate::built_generate_pg_types_model::BuiltGeneratePgTypesModel,
    crate::generate_pg_types_pipeline_error::GeneratePgTypesPipelineError,
> {
    let entry_count = crate::pg_types_model_entry_count::PgTypesModelEntryCount::from(match parsed
        .get_inner()
        .get_variant()
    {
        crate::generate_pg_types_config_variant::GeneratePgTypesConfigVariant::All => {
            <crate::pg_type_catalog_kind::PgTypeCatalogKind as strum::IntoEnumIterator>::iter()
                .count()
        }
        crate::generate_pg_types_config_variant::GeneratePgTypesConfigVariant::Concrete(
            records,
        ) => records.len(),
        crate::generate_pg_types_config_variant::GeneratePgTypesConfigVariant::Subset(types) => {
            types.len()
        }
    });
    Ok(
        crate::built_generate_pg_types_model::BuiltGeneratePgTypesModel::new(
            crate::generate_pg_types_config::GeneratePgTypesConfig::from(parsed),
            entry_count,
        ),
    )
}
