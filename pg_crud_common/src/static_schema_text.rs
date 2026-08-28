pub(crate) fn static_schema_text(
    value: super::DbStaticSchemaText,
) -> Result<super::DbSchemaText, super::DbSchemaConformanceError> {
    super::DbSchemaText::try_from(value.0.to_owned()).map_err(|error| {
        super::DbSchemaConformanceError::SchemaTextTooLong(super::DbSchemaTextError::from(error))
    })
}
