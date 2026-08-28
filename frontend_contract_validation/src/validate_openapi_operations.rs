use crate::openapi_validation::{
    OpenApiOperationExpectation, OpenApiOperationValidationError, OpenApiSecurityExpectation,
    SerdeJsonOpenApiSerializationError,
};

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
            .get(constants_str::PATHS)
            .and_then(|paths| paths.get(expectation.metadata.path().as_ref()))
            .and_then(|path| path.get(method.as_str()))
            .ok_or(OpenApiOperationValidationError::MissingOperation)?;
        let security_matches = match expectation.security {
            OpenApiSecurityExpectation::Public => operation
                .get(constants_str::SECURITY)
                .is_none_or(|security| security.as_array().is_some_and(Vec::is_empty)),
            OpenApiSecurityExpectation::Required(name) => operation
                .get(constants_str::SECURITY)
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
            .get(constants_str::RESPONSES)
            .and_then(|responses| responses.get(status.as_str()))
            .ok_or(OpenApiOperationValidationError::MissingResponseStatus)?;
        let content = response
            .get(constants_str::OPENAPI_CONTENT)
            .and_then(serde_json::Value::as_object)
            .and_then(|content| content.get(expectation.content_type.as_ref()))
            .ok_or(OpenApiOperationValidationError::MissingContentType)?;
        if content.get(constants_str::JSON_SCHEMA).is_none() {
            Err(OpenApiOperationValidationError::MissingResponseSchema)
        } else {
            Ok(())
        }
    })
}
