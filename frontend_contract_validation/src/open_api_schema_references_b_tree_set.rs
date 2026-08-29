#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::openapi_validation::{
    OpenApiContractText, OpenApiValidationError, SerdeJsonOpenApiSerializationError,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct OpenApiSchemaReferencesBTreeSet(
    pub(super) std::collections::BTreeSet<OpenApiContractText>,
);
impl OpenApiSchemaReferencesBTreeSet {
    pub(super) fn validate<Document>(
        &self,
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
        let schemas = document_value
            .pointer(constants_str::COMPONENTS_SCHEMAS_ALT)
            .and_then(serde_json::Value::as_object)
            .ok_or(OpenApiValidationError::MissingSchemas)?;
        self.0.iter().try_for_each(|reference| {
            if schemas.contains_key(reference.as_ref()) {
                Ok(())
            } else {
                Err(OpenApiValidationError::MissingSchemaReference(
                    reference.clone(),
                ))
            }
        })
    }
}
