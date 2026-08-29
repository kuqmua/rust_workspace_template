pub(crate) fn static_schema_texts(
    values: crate::db_static_schema_texts::DbStaticSchemaTexts,
) -> Result<
    crate::db_schema_texts::DbSchemaTexts,
    crate::db_schema_conformance_error::DbSchemaConformanceError,
> {
    Vec::from(values)
        .into_iter()
        .map(crate::static_schema_text::static_schema_text)
        .collect::<Result<Vec<_>, _>>()
        .map(Into::into)
}
