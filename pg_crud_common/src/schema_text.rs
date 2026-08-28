pub(crate) fn schema_text(
    value: String,
) -> Result<super::DbSchemaText, super::DbSchemaConformanceError> {
    super::DbSchemaText::try_from(value).map_err(|error| {
        super::DbSchemaConformanceError::SchemaTextTooLong(super::DbSchemaTextError::from(error))
    })
}
