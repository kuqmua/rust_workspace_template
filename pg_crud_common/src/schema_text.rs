pub(crate) fn schema_text(
    string: String,
) -> Result<
    crate::db_schema_text::DbSchemaText,
    crate::db_schema_conformance_error::DbSchemaConformanceError,
> {
    crate::db_schema_text::DbSchemaText::try_from(string)
        .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)
}
