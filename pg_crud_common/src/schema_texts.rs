pub(crate) fn schema_texts(
    values: Vec<String>,
) -> Result<Vec<super::DbSchemaText>, super::DbSchemaConformanceError> {
    values.into_iter().map(super::schema_text).collect()
}
