const OPENAPI_CONTRACT_TEXT_MAX_LEN: usize = 1_048_576usize;

#[derive(Clone, Debug, Eq, PartialEq, newtype::BoundedString)]
#[bounded_string(max = OPENAPI_CONTRACT_TEXT_MAX_LEN)]
pub struct OpenApiContractText(String);

#[derive(Clone, Copy, Debug, newtype::Display, newtype::FromInner)]
pub struct OpenApiContractTextError(OpenApiContractTextTryFromStringError);

#[derive(Debug, newtype::Display, newtype::ErrorTransparent, newtype::FromInner)]
pub struct SerdeJsonOpenApiSerializationError(serde_json::Error);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct RuntimeRoutesRef<'value_lt>(&'value_lt [crate::RouteMetadata]);

#[derive(Debug, newtype::DebugDisplay, newtype::Error)]
pub enum OpenApiValidationError {
    DocumentSerialization(SerdeJsonOpenApiSerializationError),
    MissingOperationId(OpenApiContractText, OpenApiContractText),
    MissingPaths,
    MissingSchemaReference(OpenApiContractText),
    MissingSchemas,
    OpenApiRouteMissing(OpenApiContractText, OpenApiContractText),
    OperationIdMismatch(
        OpenApiContractText,
        OpenApiContractText,
        OpenApiContractText,
        OpenApiContractText,
    ),
    RuntimeRouteMissing(OpenApiContractText, OpenApiContractText),
    TextTooLong(OpenApiContractTextError),
    UnusedSchema(OpenApiContractText),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::TryFrom)]
#[try_from(error = crate::HttpStatusTryFromU16Error, validator = |value: &u16| {
    if (100u16..1_000u16).contains(value) {
        Ok(())
    } else {
        Err(crate::HttpStatusTryFromU16Error)
    }
})]
pub struct OpenApiResponseStatus(u16);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenApiSecurityExpectation {
    Public,
    Required(crate::ContractStr),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenApiOperationExpectation {
    content_type: crate::ContractStr,
    metadata: crate::RouteMetadata,
    security: OpenApiSecurityExpectation,
    status: OpenApiResponseStatus,
}
impl OpenApiOperationExpectation {
    #[must_use]
    pub const fn new(
        metadata: crate::RouteMetadata,
        status: OpenApiResponseStatus,
        content_type: crate::ContractStr,
        security: OpenApiSecurityExpectation,
    ) -> Self {
        Self {
            content_type,
            metadata,
            security,
            status,
        }
    }
}

#[derive(Debug, newtype::DebugDisplay, newtype::Error)]
pub enum OpenApiOperationValidationError {
    DocumentSerialization(SerdeJsonOpenApiSerializationError),
    MissingContentType,
    MissingOperation,
    MissingResponseSchema,
    MissingResponseStatus,
    SecurityMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenApiSchemaMismatch {
    AdditionalProperty,
    AnyOf,
    Const,
    Enum,
    MissingReference,
    OneOf,
    RequiredProperty,
    Type,
}

#[derive(Debug, newtype::DebugDisplay, newtype::Error)]
pub enum OpenApiPayloadValidationError {
    DocumentSerialization(SerdeJsonOpenApiSerializationError),
    Mismatch(OpenApiSchemaMismatch),
    PayloadSerialization(SerdeJsonOpenApiSerializationError),
    SchemaSerialization(SerdeJsonOpenApiSerializationError),
}

pub fn validate_openapi_contract<Document>(
    document: &Document,
    runtime_routes: RuntimeRoutesRef<'_>,
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
        .pointer(str_constants::COMPONENTS_SCHEMAS_ALT)
        .and_then(serde_json::Value::as_object)
        .ok_or(OpenApiValidationError::MissingSchemas)?;
    let mut references = std::collections::BTreeSet::new();
    let mut pending = vec![&document_value];
    while let Some(current) = pending.pop() {
        match current {
            serde_json::Value::Array(values) => pending.extend(values),
            serde_json::Value::Object(values) => {
                if let Some(name) = values
                    .get(str_constants::DOLLAR_REF)
                    .and_then(serde_json::Value::as_str)
                    .and_then(|reference| reference.strip_prefix(str_constants::COMPONENTS_SCHEMAS))
                {
                    let _inserted: bool = references.insert(name.to_owned());
                }
                pending.extend(values.values());
            }
            serde_json::Value::Bool(_)
            | serde_json::Value::Null
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {}
        }
    }
    references.iter().try_for_each(|reference| {
        if schemas.contains_key(reference) {
            Ok(())
        } else {
            Err(OpenApiValidationError::MissingSchemaReference(
                OpenApiContractText::try_from(reference.clone()).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
            ))
        }
    })?;
    schemas.keys().try_for_each(|name| {
        if references.contains(name) {
            Ok(())
        } else {
            Err(OpenApiValidationError::UnusedSchema(
                OpenApiContractText::try_from(name.clone()).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
            ))
        }
    })?;

    let paths = document_value
        .get(str_constants::PATHS)
        .and_then(serde_json::Value::as_object)
        .ok_or(OpenApiValidationError::MissingPaths)?;
    let mut documented = std::collections::BTreeMap::new();
    paths.iter().try_for_each(|(path, path_item)| {
        path_item
            .as_object()
            .into_iter()
            .flatten()
            .filter(|(method, _)| {
                [
                    str_constants::DELETE,
                    str_constants::GET,
                    str_constants::HEAD,
                    str_constants::OPTIONS,
                    str_constants::PATCH,
                    str_constants::POST,
                    str_constants::PUT,
                    str_constants::TRACE,
                ]
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(method))
            })
            .try_for_each(|(method, operation)| {
                let Some(operation_id) = operation
                    .get(str_constants::OPERATION_ID_JSON)
                    .and_then(serde_json::Value::as_str)
                else {
                    return Err(OpenApiValidationError::MissingOperationId(
                        OpenApiContractText::try_from(method.clone()).map_err(|error| {
                            OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(
                                error,
                            ))
                        })?,
                        OpenApiContractText::try_from(path.clone()).map_err(|error| {
                            OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(
                                error,
                            ))
                        })?,
                    ));
                };
                let _previous = documented.insert(
                    (method.to_ascii_uppercase(), path.clone()),
                    operation_id.to_owned(),
                );
                Ok(())
            })
    })?;
    runtime_routes.0.iter().try_for_each(|route| {
        let method = route.method().as_ref().to_ascii_uppercase();
        let path = route.path().as_ref().to_owned();
        let Some(operation_id) = documented.get(&(method.clone(), path.clone())) else {
            return Err(OpenApiValidationError::RuntimeRouteMissing(
                OpenApiContractText::try_from(method).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
                OpenApiContractText::try_from(path).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
            ));
        };
        if operation_id == route.openapi_operation_id().as_ref() {
            Ok(())
        } else {
            Err(OpenApiValidationError::OperationIdMismatch(
                OpenApiContractText::try_from(method).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
                OpenApiContractText::try_from(path).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
                OpenApiContractText::try_from(route.openapi_operation_id().as_ref().to_owned())
                    .map_err(|error| {
                        OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                    })?,
                OpenApiContractText::try_from(operation_id.clone()).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
            ))
        }
    })?;
    documented.into_iter().try_for_each(|((method, path), _)| {
        if runtime_routes.0.iter().any(|route| {
            route
                .method()
                .as_ref()
                .eq_ignore_ascii_case(method.as_str())
                && route.path().as_ref() == path
        }) {
            Ok(())
        } else {
            Err(OpenApiValidationError::OpenApiRouteMissing(
                OpenApiContractText::try_from(method).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
                OpenApiContractText::try_from(path).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
            ))
        }
    })
}

pub fn validate_openapi_operations<Document>(
    document: &Document,
    expectations: &[OpenApiOperationExpectation],
) -> Result<(), OpenApiOperationValidationError>
where
    Document: serde::Serialize,
{
    let document_value = serde_json::to_value(document).map_err(|error| {
        OpenApiOperationValidationError::DocumentSerialization(
            SerdeJsonOpenApiSerializationError::from(error),
        )
    })?;
    expectations.iter().try_for_each(|expectation| {
        let method = expectation.metadata.method().as_ref().to_ascii_lowercase();
        let operation = document_value
            .get(str_constants::PATHS)
            .and_then(|paths| paths.get(expectation.metadata.path().as_ref()))
            .and_then(|path| path.get(method.as_str()))
            .ok_or(OpenApiOperationValidationError::MissingOperation)?;
        let security_matches = match expectation.security {
            OpenApiSecurityExpectation::Public => operation
                .get(str_constants::SECURITY)
                .is_none_or(|security| security.as_array().is_some_and(Vec::is_empty)),
            OpenApiSecurityExpectation::Required(name) => operation
                .get(str_constants::SECURITY)
                .and_then(serde_json::Value::as_array)
                .is_some_and(|requirements| {
                    requirements.iter().any(|requirement| {
                        requirement
                            .as_object()
                            .is_some_and(|object| object.contains_key(name.as_ref()))
                    })
                }),
        };
        if !security_matches {
            return Err(OpenApiOperationValidationError::SecurityMismatch);
        }
        let status = expectation.status.0.to_string();
        let response = operation
            .get(str_constants::RESPONSES)
            .and_then(|responses| responses.get(status.as_str()))
            .ok_or(OpenApiOperationValidationError::MissingResponseStatus)?;
        let content = response
            .get(str_constants::OPENAPI_CONTENT)
            .and_then(serde_json::Value::as_object)
            .and_then(|content| content.get(expectation.content_type.as_ref()))
            .ok_or(OpenApiOperationValidationError::MissingContentType)?;
        if content.get(str_constants::JSON_SCHEMA).is_none() {
            Err(OpenApiOperationValidationError::MissingResponseSchema)
        } else {
            Ok(())
        }
    })
}

pub fn validate_openapi_json_payload<Payload, Schema, Document>(
    payload: &Payload,
    schema: &Schema,
    document: &Document,
) -> Result<(), OpenApiPayloadValidationError>
where
    Payload: serde::Serialize,
    Schema: serde::Serialize,
    Document: serde::Serialize,
{
    let payload_value = serde_json::to_value(payload).map_err(|error| {
        OpenApiPayloadValidationError::PayloadSerialization(
            SerdeJsonOpenApiSerializationError::from(error),
        )
    })?;
    let schema_value = serde_json::to_value(schema).map_err(|error| {
        OpenApiPayloadValidationError::SchemaSerialization(
            SerdeJsonOpenApiSerializationError::from(error),
        )
    })?;
    let document_value = serde_json::to_value(document).map_err(|error| {
        OpenApiPayloadValidationError::DocumentSerialization(
            SerdeJsonOpenApiSerializationError::from(error),
        )
    })?;

    if let Some(reference) = schema_value
        .get(str_constants::DOLLAR_REF)
        .and_then(serde_json::Value::as_str)
    {
        let referenced_schema = reference
            .strip_prefix(str_constants::COMPONENTS_SCHEMAS)
            .and_then(|name| {
                let schemas = document_value.pointer(str_constants::COMPONENTS_SCHEMAS_ALT)?;
                schemas.get(name)
            })
            .ok_or(OpenApiPayloadValidationError::Mismatch(
                OpenApiSchemaMismatch::MissingReference,
            ))?;
        return validate_openapi_json_payload(&payload_value, referenced_schema, &document_value);
    }
    if payload_value.is_null()
        && schema_value
            .get(str_constants::NULLABLE)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    {
        return Ok(());
    }
    if let Some(all_of) = schema_value
        .get(str_constants::ALL_OF)
        .and_then(serde_json::Value::as_array)
    {
        all_of.iter().try_for_each(|candidate| {
            validate_openapi_json_payload(&payload_value, candidate, &document_value)
        })?;
    }
    if let Some(one_of) = schema_value
        .get(str_constants::ONE_OF)
        .and_then(serde_json::Value::as_array)
    {
        let matches = one_of
            .iter()
            .filter(|candidate| {
                validate_openapi_json_payload(&payload_value, *candidate, &document_value).is_ok()
            })
            .count();
        if matches != 1usize {
            return Err(OpenApiPayloadValidationError::Mismatch(
                OpenApiSchemaMismatch::OneOf,
            ));
        }
    }
    if let Some(any_of) = schema_value
        .get(str_constants::ANY_OF)
        .and_then(serde_json::Value::as_array)
        && !any_of.iter().any(|candidate| {
            validate_openapi_json_payload(&payload_value, candidate, &document_value).is_ok()
        })
    {
        return Err(OpenApiPayloadValidationError::Mismatch(
            OpenApiSchemaMismatch::AnyOf,
        ));
    }
    if let Some(expected_type) = schema_value
        .get(str_constants::JSON_TYPE)
        .and_then(serde_json::Value::as_str)
    {
        let type_matches = match expected_type {
            str_constants::ARRAY => payload_value.is_array(),
            str_constants::BOOLEAN => payload_value.is_boolean(),
            str_constants::INTEGER => {
                payload_value.as_i64().is_some() || payload_value.as_u64().is_some()
            }
            str_constants::JSON_NULL => payload_value.is_null(),
            str_constants::NUMBER => payload_value.is_number(),
            str_constants::OBJECT => payload_value.is_object(),
            str_constants::STRING => payload_value.is_string(),
            _ => false,
        };
        if !type_matches {
            return Err(OpenApiPayloadValidationError::Mismatch(
                OpenApiSchemaMismatch::Type,
            ));
        }
    }
    if schema_value
        .get(str_constants::ENUM)
        .and_then(serde_json::Value::as_array)
        .is_some_and(|values| !values.contains(&payload_value))
    {
        return Err(OpenApiPayloadValidationError::Mismatch(
            OpenApiSchemaMismatch::Enum,
        ));
    }
    if schema_value
        .get(str_constants::CONST)
        .is_some_and(|value| value != &payload_value)
    {
        return Err(OpenApiPayloadValidationError::Mismatch(
            OpenApiSchemaMismatch::Const,
        ));
    }
    if let Some(object) = payload_value.as_object() {
        let properties = schema_value
            .get(str_constants::PROPERTIES)
            .and_then(serde_json::Value::as_object);
        if let Some(required) = schema_value
            .get(str_constants::REQUIRED)
            .and_then(serde_json::Value::as_array)
            && required.iter().any(|field| {
                field
                    .as_str()
                    .is_some_and(|field_name| !object.contains_key(field_name))
            })
        {
            return Err(OpenApiPayloadValidationError::Mismatch(
                OpenApiSchemaMismatch::RequiredProperty,
            ));
        }
        if schema_value
            .get(str_constants::ADDITIONAL_PROPERTIES)
            .and_then(serde_json::Value::as_bool)
            == Some(false)
            && object.keys().any(|field| {
                properties.is_none_or(|defined_properties| !defined_properties.contains_key(field))
            })
        {
            return Err(OpenApiPayloadValidationError::Mismatch(
                OpenApiSchemaMismatch::AdditionalProperty,
            ));
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
            .get(str_constants::ADDITIONAL_PROPERTIES)
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
    if let Some(items_schema) = schema_value.get(str_constants::ITEMS)
        && let Some(items) = payload_value.as_array()
    {
        items.iter().try_for_each(|item| {
            validate_openapi_json_payload(item, items_schema, &document_value)
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn valid_document_matches_runtime_route_and_references() {
        let document = serde_json::json!({
            str_constants::PATHS: { str_constants::TEST_OPENAPI_PATH: { str_constants::GET_LOWERCASE: {
                str_constants::OPERATION_ID_JSON: str_constants::TEST_OPENAPI_OPERATION_ID,
                str_constants::RESPONSES: { str_constants::STATUS_OK: { str_constants::OPENAPI_CONTENT: { str_constants::APPLICATION_JSON: {
                    str_constants::JSON_SCHEMA: { str_constants::DOLLAR_REF: str_constants::TEST_OPENAPI_SCHEMA_REF }
                }}}}
            }}},
            str_constants::COMPONENTS: { str_constants::SCHEMAS: { str_constants::TEST_OPENAPI_SCHEMA: { str_constants::JSON_TYPE: str_constants::OBJECT }}}
        });
        let routes = [crate::RouteMetadata::new(
            crate::RouteMethod::Get,
            str_constants::TEST_OPENAPI_OPERATION_ID.into(),
            str_constants::TEST_OPENAPI_PATH.into(),
        )];
        assert!(matches!(
            super::validate_openapi_contract(&document, routes.as_slice().into()),
            Ok(())
        ));
    }

    #[test]
    fn dangling_reference_is_rejected() {
        let document = serde_json::json!({
            str_constants::PATHS: {},
            str_constants::COMPONENTS: { str_constants::SCHEMAS: { str_constants::TEST_OPENAPI_SCHEMA: {
                str_constants::DOLLAR_REF: str_constants::TEST_OPENAPI_MISSING_SCHEMA_REF
            }}}
        });
        assert!(matches!(
            super::validate_openapi_contract(&document, (&[][..]).into()),
            Err(super::OpenApiValidationError::MissingSchemaReference(_))
        ));
    }

    #[test]
    fn operation_security_status_and_content_type_are_checked() {
        let document = serde_json::json!({
            str_constants::PATHS: { str_constants::TEST_OPENAPI_PATH: { str_constants::GET_LOWERCASE: {
                str_constants::OPERATION_ID_JSON: str_constants::TEST_OPENAPI_OPERATION_ID,
                str_constants::RESPONSES: { str_constants::STATUS_OK: { str_constants::OPENAPI_CONTENT: {
                    str_constants::APPLICATION_JSON: { str_constants::JSON_SCHEMA: { str_constants::JSON_TYPE: str_constants::OBJECT }}
                }}}
            }}}
        });
        let expectation = super::OpenApiOperationExpectation::new(
            crate::RouteMetadata::new(
                crate::RouteMethod::Get,
                str_constants::TEST_OPENAPI_OPERATION_ID.into(),
                str_constants::TEST_OPENAPI_PATH.into(),
            ),
            super::OpenApiResponseStatus::try_from(200u16).expect("9f6e9528"),
            str_constants::APPLICATION_JSON.into(),
            super::OpenApiSecurityExpectation::Public,
        );
        assert!(matches!(
            super::validate_openapi_operations(&document, &[expectation]),
            Ok(())
        ));
    }

    #[test]
    fn payload_schema_checks_required_fields_and_additional_properties() {
        let document = serde_json::json!({
            str_constants::COMPONENTS: { str_constants::SCHEMAS: {}}
        });
        let schema = serde_json::json!({
            str_constants::JSON_TYPE: str_constants::OBJECT,
            str_constants::REQUIRED: [str_constants::NAME],
            str_constants::PROPERTIES: {
                str_constants::NAME: { str_constants::JSON_TYPE: str_constants::STRING }
            },
            str_constants::ADDITIONAL_PROPERTIES: false
        });
        assert!(matches!(
            super::validate_openapi_json_payload(
                &serde_json::json!({str_constants::NAME: str_constants::TEST_OPENAPI_SCHEMA}),
                &schema,
                &document,
            ),
            Ok(())
        ));
        assert!(matches!(
            super::validate_openapi_json_payload(&serde_json::json!({}), &schema, &document),
            Err(super::OpenApiPayloadValidationError::Mismatch(
                super::OpenApiSchemaMismatch::RequiredProperty
            ))
        ));
    }
}
