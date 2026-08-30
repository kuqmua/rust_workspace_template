pub fn validate_openapi_operations<Document>(
    document: &Document,
    expectations: &[crate::open_api_operation_expectation::OpenApiOperationExpectation],
) -> Result<(), crate::open_api_operation_validation_error::OpenApiOperationValidationError>
where
    Document: serde::Serialize,
{
    let document_value = serde_json::to_value(document).map_err(|error| {
        crate::open_api_operation_validation_error::OpenApiOperationValidationError::DocumentSerialization(
            crate::serde_json_open_api_serialization_error::SerdeJsonOpenApiSerializationError::from(error),
        )
    })?;
    expectations.iter().try_for_each(|expectation| {
        let (content_type, metadata, security, response_status) = expectation.parts();
        let method = metadata.method().as_ref().to_ascii_lowercase();
        let operation = document_value
            .get(constants_str::catalog::PATHS)
            .and_then(|paths| paths.get(metadata.path().as_ref()))
            .and_then(|path| path.get(method.as_str()))
            .ok_or(crate::open_api_operation_validation_error::OpenApiOperationValidationError::MissingOperation)?;
        let security_matches = match security {
            crate::open_api_security_expectation::OpenApiSecurityExpectation::Public => operation
                .get(constants_str::test_fixtures::SECURITY)
                .is_none_or(|security_value| {
                    security_value.as_array().is_some_and(Vec::is_empty)
                }),
            crate::open_api_security_expectation::OpenApiSecurityExpectation::Required(name) => operation
                .get(constants_str::test_fixtures::SECURITY)
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
            return Err(
                crate::open_api_operation_validation_error::OpenApiOperationValidationError::SecurityMismatch,
            );
        }
        let status = response_status.to_string();
        let response = operation
            .get(constants_str::catalog::RESPONSES)
            .and_then(|responses| responses.get(status.as_str()))
            .ok_or(
                crate::open_api_operation_validation_error::OpenApiOperationValidationError::MissingResponseStatus,
            )?;
        let content = response
            .get(constants_str::test_fixtures::OPENAPI_CONTENT)
            .and_then(serde_json::Value::as_object)
            .and_then(|content| content.get(content_type.as_ref()))
            .ok_or(
                crate::open_api_operation_validation_error::OpenApiOperationValidationError::MissingContentType,
            )?;
        if content.get(constants_str::test_fixtures::JSON_SCHEMA).is_none() {
            Err(crate::open_api_operation_validation_error::OpenApiOperationValidationError::MissingResponseSchema)
        } else {
            Ok(())
        }
    })
}
