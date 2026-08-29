pub(super) fn openapi_schema_references(
    document: &impl serde::Serialize,
) -> Result<
    crate::open_api_schema_references_b_tree_set::OpenApiSchemaReferencesBTreeSet,
    crate::open_api_validation_error::OpenApiValidationError,
> {
    let document_value = serde_json::to_value(document).map_err(|error| {
        crate::open_api_validation_error::OpenApiValidationError::DocumentSerialization(
            crate::serde_json_open_api_serialization_error::SerdeJsonOpenApiSerializationError::from(error),
        )
    })?;
    let _schemas = document_value
        .pointer(constants_str::catalog::COMPONENTS_SCHEMAS_ALT)
        .and_then(serde_json::Value::as_object)
        .ok_or(crate::open_api_validation_error::OpenApiValidationError::MissingSchemas)?;
    let mut references = std::collections::BTreeSet::new();
    let mut pending = vec![&document_value];
    while let Some(current) = pending.pop() {
        match current {
            serde_json::Value::Array(values) => pending.extend(values),
            serde_json::Value::Object(values) => {
                if let Some(name) = values
                    .get(constants_str::catalog::DOLLAR_REF)
                    .and_then(serde_json::Value::as_str)
                    .and_then(|reference| {
                        reference.strip_prefix(constants_str::catalog::COMPONENTS_SCHEMAS)
                    })
                {
                    let reference = crate::open_api_contract_text::OpenApiContractText::try_from(
                        name.to_owned(),
                    )
                    .map_err(
                        crate::open_api_validation_error::OpenApiValidationError::TextTooLong,
                    )?;
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
    Ok(
        crate::open_api_schema_references_b_tree_set::OpenApiSchemaReferencesBTreeSet::from(
            references,
        ),
    )
}
