pub(crate) fn schema_text(
    value: String,
) -> Result<super::DbSchemaText, super::DbSchemaConformanceError> {
    super::DbSchemaText::try_from(value).map_err(super::DbSchemaConformanceError::SchemaTextTooLong)
}
