use super::openapi_validation::{
    OpenApiValidationError, SerdeJsonOpenApiSerializationError, openapi_schema_references,
};

pub fn validate_openapi_schema_references<Document>(
    document: &Document,
) -> Result<(), OpenApiValidationError>
where
    Document: serde::Serialize,
{
    let document_value = serde_json::to_value(document).map_err(|error| {
        OpenApiValidationError::DocumentSerialization(SerdeJsonOpenApiSerializationError::from(
            error,
        ))
    })?;
    openapi_schema_references(&document_value)?.validate(&document_value)
}
