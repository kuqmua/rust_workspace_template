pub(crate) fn schema_texts(
    values: Vec<String>,
) -> Result<
    Vec<crate::db_schema_text::DbSchemaText>,
    crate::db_schema_conformance_error::DbSchemaConformanceError,
> {
    values
        .into_iter()
        .map(crate::schema_text::schema_text)
        .collect()
}
