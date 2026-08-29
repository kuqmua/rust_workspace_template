pub fn validate_openapi_json_payload<Payload, Schema, Document>(
    payload: &Payload,
    schema: &Schema,
    document: &Document,
) -> Result<(), crate::open_api_payload_validation_error::OpenApiPayloadValidationError>
where
    Payload: serde::Serialize,
    Schema: serde::Serialize,
    Document: serde::Serialize,
{
    let payload_value = serde_json::to_value(payload).map_err(|error| {
        crate::open_api_payload_validation_error::OpenApiPayloadValidationError::PayloadSerialization(
            crate::serde_json_open_api_serialization_error::SerdeJsonOpenApiSerializationError::from(error),
        )
    })?;
    let schema_value = serde_json::to_value(schema).map_err(|error| {
        crate::open_api_payload_validation_error::OpenApiPayloadValidationError::SchemaSerialization(
            crate::serde_json_open_api_serialization_error::SerdeJsonOpenApiSerializationError::from(error),
        )
    })?;
    let document_value = serde_json::to_value(document).map_err(|error| {
        crate::open_api_payload_validation_error::OpenApiPayloadValidationError::DocumentSerialization(
            crate::serde_json_open_api_serialization_error::SerdeJsonOpenApiSerializationError::from(error),
        )
    })?;

    if let Some(reference) = schema_value
        .get(constants_str::catalog::DOLLAR_REF)
        .and_then(serde_json::Value::as_str)
    {
        let referenced_schema = reference
            .strip_prefix(constants_str::catalog::COMPONENTS_SCHEMAS)
            .and_then(|name| {
                let schemas =
                    document_value.pointer(constants_str::catalog::COMPONENTS_SCHEMAS_ALT)?;
                schemas.get(name)
            })
            .ok_or(
                crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
                    crate::open_api_schema_mismatch::OpenApiSchemaMismatch::MissingReference,
                ),
            )?;
        return validate_openapi_json_payload(&payload_value, referenced_schema, &document_value);
    }
    if payload_value.is_null()
        && schema_value
            .get(constants_str::catalog::NULLABLE)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    {
        return Ok(());
    }
    if let Some(all_of) = schema_value
        .get(constants_str::test_fixtures::ALL_OF)
        .and_then(serde_json::Value::as_array)
    {
        all_of.iter().try_for_each(|candidate| {
            validate_openapi_json_payload(&payload_value, candidate, &document_value)
        })?;
    }
    if let Some(one_of) = schema_value
        .get(constants_str::test_fixtures::ONE_OF)
        .and_then(serde_json::Value::as_array)
    {
        let matches = one_of
            .iter()
            .filter(|candidate| {
                validate_openapi_json_payload(&payload_value, *candidate, &document_value).is_ok()
            })
            .count();
        if matches != constants_usize::ONE {
            return Err(
                crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
                    crate::open_api_schema_mismatch::OpenApiSchemaMismatch::OneOf,
                ),
            );
        }
    }
    if let Some(any_of) = schema_value
        .get(constants_str::test_fixtures::ANY_OF)
        .and_then(serde_json::Value::as_array)
        && !any_of.iter().any(|candidate| {
            validate_openapi_json_payload(&payload_value, candidate, &document_value).is_ok()
        })
    {
        return Err(
            crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
                crate::open_api_schema_mismatch::OpenApiSchemaMismatch::AnyOf,
            ),
        );
    }
    if let Some(expected_type) = schema_value
        .get(constants_str::test_fixtures::JSON_TYPE)
        .and_then(serde_json::Value::as_str)
    {
        let type_matches = match expected_type {
            constants_str::test_fixtures::ARRAY => payload_value.is_array(),
            constants_str::test_fixtures::BOOLEAN => payload_value.is_boolean(),
            constants_str::test_fixtures::INTEGER => {
                payload_value.as_i64().is_some() || payload_value.as_u64().is_some()
            }
            constants_str::test_fixtures::JSON_NULL => payload_value.is_null(),
            constants_str::test_fixtures::NUMBER => payload_value.is_number(),
            constants_str::test_fixtures::OBJECT => payload_value.is_object(),
            constants_str::catalog::STRING => payload_value.is_string(),
            _ => false,
        };
        if !type_matches {
            return Err(
                crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
                    crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Type,
                ),
            );
        }
    }
    if schema_value
        .get(constants_str::catalog::ENUM)
        .and_then(serde_json::Value::as_array)
        .is_some_and(|values| !values.contains(&payload_value))
    {
        return Err(
            crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
                crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Enum,
            ),
        );
    }
    if schema_value
        .get(constants_str::test_fixtures::CONST)
        .is_some_and(|value| value != &payload_value)
    {
        return Err(
            crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
                crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Const,
            ),
        );
    }
    if let Some(object) = payload_value.as_object() {
        let properties = schema_value
            .get(constants_str::catalog::PROPERTIES)
            .and_then(serde_json::Value::as_object);
        if let Some(required) = schema_value
            .get(constants_str::test_fixtures::REQUIRED)
            .and_then(serde_json::Value::as_array)
            && required.iter().any(|field| {
                field
                    .as_str()
                    .is_some_and(|field_name| !object.contains_key(field_name))
            })
        {
            return Err(
                crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
                    crate::open_api_schema_mismatch::OpenApiSchemaMismatch::RequiredProperty,
                ),
            );
        }
        if schema_value
            .get(constants_str::test_fixtures::ADDITIONAL_PROPERTIES)
            .and_then(serde_json::Value::as_bool)
            == Some(false)
            && object.keys().any(|field| {
                properties.is_none_or(|defined_properties| !defined_properties.contains_key(field))
            })
        {
            return Err(
                crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
                    crate::open_api_schema_mismatch::OpenApiSchemaMismatch::AdditionalProperty,
                ),
            );
        }
        if let Some(defined_properties) = properties {
            defined_properties
                .iter()
                .try_for_each(|(field, field_schema)| {
                    object.get(field).map_or(Ok(()), |field_value| {
                        validate_openapi_json_payload(field_value, field_schema, &document_value)
                    })
                })?;
        }
        if let Some(additional_schema) = schema_value
            .get(constants_str::test_fixtures::ADDITIONAL_PROPERTIES)
            .and_then(serde_json::Value::as_object)
        {
            object
                .iter()
                .filter(|(field, _)| {
                    properties.is_none_or(|defined| !defined.contains_key(field.as_str()))
                })
                .try_for_each(|(_, field_value)| {
                    validate_openapi_json_payload(field_value, additional_schema, &document_value)
                })?;
        }
    }
    if let Some(items_schema) = schema_value.get(constants_str::test_fixtures::ITEMS)
        && let Some(items) = payload_value.as_array()
    {
        items.iter().try_for_each(|item| {
            validate_openapi_json_payload(item, items_schema, &document_value)
        })?;
    }
    Ok(())
}
