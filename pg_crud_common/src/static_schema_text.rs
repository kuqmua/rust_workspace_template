pub(crate) fn static_schema_text(
    value: crate::db_static_schema_text::DbStaticSchemaText,
) -> Result<
    crate::db_schema_text::DbSchemaText,
    crate::db_schema_conformance_error::DbSchemaConformanceError,
> {
    crate::db_schema_text::DbSchemaText::try_from(value.0.to_owned())
        .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)
}
