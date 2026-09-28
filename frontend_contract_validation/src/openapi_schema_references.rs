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
        .pointer(constants_str::COMPONENTS_SCHEMAS_ALT)
        .and_then(serde_json::Value::as_object)
        .ok_or(crate::open_api_validation_error::OpenApiValidationError::MissingSchemas)?;
    let mut references = std::collections::BTreeSet::new();
    let mut pending = vec![&document_value];
    while let Some(current) = pending.pop() {
        match current {
            serde_json::Value::Array(values) => pending.extend(values),
            serde_json::Value::Object(values) => {
                if let Some((name, reference)) = values
                    .get(constants_str::DOLLAR_REF)
                    .and_then(serde_json::Value::as_str)
                    .and_then(|reference| {
                        reference
                            .strip_prefix(constants_str::COMPONENTS_SCHEMAS)
                            .map(|name| (name, reference))
                    })
                {
                    let mut decoded_name = name.to_owned();
                    if reference
                        .get(1..)
                        .and_then(|pointer| document_value.pointer(pointer))
                        .is_none()
                    {
                        return Err(crate::open_api_validation_error::OpenApiValidationError::MissingSchemaReference(
                            crate::open_api_contract_text::OpenApiContractText::try_from(decoded_name)
                                .map_err(crate::open_api_validation_error::OpenApiValidationError::TextTooLong)?,
                        ));
                    }
                    if let Some(index) = decoded_name.find('/') {
                        decoded_name.truncate(index);
                    }
                    let mut search_start = 0usize;
                    while let Some(relative_index) = decoded_name
                        .get(search_start..)
                        .and_then(|remaining| remaining.find('~'))
                    {
                        let index = search_start.saturating_add(relative_index);
                        let after_index = index.saturating_add(1);
                        let decoded_character = match decoded_name.as_bytes().get(after_index) {
                            Some(b'0') => '~',
                            Some(b'1') => '/',
                            Some(_) | None => {
                                return Err(crate::open_api_validation_error::OpenApiValidationError::MissingSchemaReference(
                                    crate::open_api_contract_text::OpenApiContractText::try_from(decoded_name)
                                        .map_err(crate::open_api_validation_error::OpenApiValidationError::TextTooLong)?,
                                ));
                            }
                        };
                        let mut buffer = [0u8; 4];
                        decoded_name.replace_range(
                            index..after_index.saturating_add(1),
                            decoded_character.encode_utf8(&mut buffer),
                        );
                        search_start = after_index;
                    }
                    let schema_reference = crate::open_api_contract_text::OpenApiContractText::try_from(
                        decoded_name,
                    )
                    .map_err(
                        crate::open_api_validation_error::OpenApiValidationError::TextTooLong,
                    )?;
                    let _inserted: bool = references.insert(schema_reference);
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
