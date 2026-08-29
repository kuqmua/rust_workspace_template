pub fn validate_openapi_schema_references<Document>(
    document: &Document,
) -> Result<(), crate::open_api_validation_error::OpenApiValidationError>
where
    Document: serde::Serialize,
{
    let document_value = serde_json::to_value(document).map_err(|error| {
        crate::open_api_validation_error::OpenApiValidationError::DocumentSerialization(
            crate::serde_json_open_api_serialization_error::SerdeJsonOpenApiSerializationError::from(error),
        )
    })?;
    crate::openapi_schema_references::openapi_schema_references(&document_value)?
        .validate(&document_value)
}
