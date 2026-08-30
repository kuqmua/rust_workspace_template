#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::DerefInner, newtype::FromInner)]
pub(super) struct OpenApiSchemaReferencesBTreeSet(
    std::collections::BTreeSet<crate::open_api_contract_text::OpenApiContractText>,
);
impl OpenApiSchemaReferencesBTreeSet {
    pub(super) fn validate<Document>(
        &self,
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
        let schemas = document_value
            .pointer(constants_str::catalog::COMPONENTS_SCHEMAS_ALT)
            .and_then(serde_json::Value::as_object)
            .ok_or(crate::open_api_validation_error::OpenApiValidationError::MissingSchemas)?;
        self.0.iter().try_for_each(|reference| {
            if schemas.contains_key(reference.as_ref()) {
                Ok(())
            } else {
                Err(
                    crate::open_api_validation_error::OpenApiValidationError::MissingSchemaReference(
                        reference.clone(),
                    ),
                )
            }
        })
    }
}
