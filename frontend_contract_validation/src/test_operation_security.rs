#[test]
fn test_operation_security_override_matrix_rejects_malformed_requirements() {
    assert!(crate::open_api_response_status::OpenApiResponseStatus::try_from(200u16).is_ok_and(|status| {
        let metadata = frontend_contract::route_metadata::RouteMetadata::new(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::TEST_OPENAPI_OPERATION_ID.into(),
            constants_str::TEST_OPENAPI_PATH.into(),
        );
        let public = crate::open_api_operation_expectation::OpenApiOperationExpectation::new(
            metadata, status, constants_str::APPLICATION_JSON.into(),
            crate::open_api_security_expectation::OpenApiSecurityExpectation::Public,
        );
        let required = crate::open_api_operation_expectation::OpenApiOperationExpectation::new(
            metadata, status, constants_str::APPLICATION_JSON.into(),
            crate::open_api_security_expectation::OpenApiSecurityExpectation::Required(constants_str::NAME.into()),
        );
        [
            (None, false, true),
            (Some(serde_json::json!([])), true, false),
            (Some(serde_json::json!([{constants_str::NAME: []}])), false, true),
            (Some(serde_json::json!([{constants_str::NAME: []}, {constants_str::NAME: []}])), false, true),
            (Some(serde_json::json!([{constants_str::NAME: []}, {}])), false, false),
            (Some(serde_json::json!([{constants_str::X: []}])), false, false),
            (Some(serde_json::json!([null])), false, false),
            (Some(serde_json::json!({constants_str::NAME: []})), false, false),
            (Some(serde_json::json!(null)), false, false),
        ].into_iter().all(|(security, accepts_public, accepts_required)| {
            let mut operation = serde_json::json!({
                constants_str::OPERATION_ID_JSON: constants_str::TEST_OPENAPI_OPERATION_ID,
                constants_str::RESPONSES: {constants_str::STATUS_OK: {constants_str::OPENAPI_CONTENT: {
                    constants_str::APPLICATION_JSON: {constants_str::JSON_SCHEMA: {}}
                }}},
            });
            if let Some(operation_security) = security
                && !operation.as_object_mut().is_some_and(|object| {
                    object.insert(constants_str::SECURITY.to_owned(), operation_security).is_none()
                })
            {
                return false;
            }
            let document = serde_json::json!({
                constants_str::SECURITY: [{constants_str::NAME: []}],
                constants_str::PATHS: {constants_str::TEST_OPENAPI_PATH: {constants_str::GET_LOWERCASE: operation}},
            });
            [(public, accepts_public), (required, accepts_required)].into_iter().all(|(expectation, accepted)| {
                let result = crate::validate_openapi_operations::validate_openapi_operations(&document, &[expectation]);
                if accepted {
                    matches!(result, Ok(()))
                } else {
                    matches!(result, Err(crate::open_api_operation_validation_error::OpenApiOperationValidationError::SecurityMismatch))
                }
            })
        })
    }));
}
