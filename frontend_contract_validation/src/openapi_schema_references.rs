use crate::openapi_validation::{
    OpenApiContractText, OpenApiSchemaReferencesBTreeSet, OpenApiValidationError,
    SerdeJsonOpenApiSerializationError,
};

pub(super) fn openapi_schema_references(
    document: &impl serde::Serialize,
) -> Result<OpenApiSchemaReferencesBTreeSet, OpenApiValidationError> {
    let document_value = serde_json::to_value(document).map_err(|error| {
        OpenApiValidationError::DocumentSerialization(SerdeJsonOpenApiSerializationError::from(
            error,
        ))
    })?;
    let _schemas = document_value
        .pointer(constants_str::COMPONENTS_SCHEMAS_ALT)
        .and_then(serde_json::Value::as_object)
        .ok_or(OpenApiValidationError::MissingSchemas)?;
    let mut references = std::collections::BTreeSet::new();
    let mut pending = vec![&document_value];
    while let Some(current) = pending.pop() {
        match current {
            serde_json::Value::Array(values) => pending.extend(values),
            serde_json::Value::Object(values) => {
                if let Some(name) = values
                    .get(constants_str::DOLLAR_REF)
                    .and_then(serde_json::Value::as_str)
                    .and_then(|reference| reference.strip_prefix(constants_str::COMPONENTS_SCHEMAS))
                {
                    let reference = OpenApiContractText::try_from(name.to_owned())
                        .map_err(OpenApiValidationError::TextTooLong)?;
                    let _inserted: bool = references.insert(reference);
                }
                pending.extend(values.values());
            }
            serde_json::Value::Bool(_)
            | serde_json::Value::Null
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {}
        }
    }
    Ok(OpenApiSchemaReferencesBTreeSet::from(references))
}
