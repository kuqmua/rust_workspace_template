#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_document_matches_runtime_route_and_references() {
        let document = serde_json::json!({
            constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::GET_LOWERCASE: {
                constants_str::OPERATION_ID_JSON: constants_str::TEST_OPENAPI_OPERATION_ID,
                constants_str::RESPONSES: { constants_str::STATUS_OK: { constants_str::OPENAPI_CONTENT: { constants_str::APPLICATION_JSON: {
                    constants_str::JSON_SCHEMA: { constants_str::DOLLAR_REF: constants_str::TEST_OPENAPI_SCHEMA_REF }
                }}}}
            }}},
            constants_str::COMPONENTS: { constants_str::SCHEMAS: { constants_str::TEST_OPENAPI_SCHEMA: { constants_str::JSON_TYPE: constants_str::OBJECT }}}
        });
        let routes = [frontend_contract::route_metadata::RouteMetadata::new(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::TEST_OPENAPI_OPERATION_ID.into(),
            constants_str::TEST_OPENAPI_PATH.into(),
        )];
        assert!(matches!(
            crate::validate_openapi_contract::validate_openapi_contract(
                &document,
                routes.as_slice().into()
            ),
            Ok(())
        ));
    }

    #[test]
    fn test_dangling_reference_is_rejected() {
        let document = serde_json::json!({
            constants_str::PATHS: {},
            constants_str::COMPONENTS: { constants_str::SCHEMAS: { constants_str::TEST_OPENAPI_SCHEMA: {
                constants_str::DOLLAR_REF: constants_str::TEST_OPENAPI_MISSING_SCHEMA_REF
            }}}
        });
        assert!(matches!(
            crate::validate_openapi_contract::validate_openapi_contract(
                &document,
                (&[][..]).into()
            ),
            Err(
                crate::open_api_validation_error::OpenApiValidationError::MissingSchemaReference(_)
            )
        ));
        assert!(matches!(
            crate::validate_openapi_schema_references::validate_openapi_schema_references(
                &document
            ),
            Err(
                crate::open_api_validation_error::OpenApiValidationError::MissingSchemaReference(_)
            )
        ));
    }

    #[test]
    fn test_operation_security_status_and_content_type_are_checked() {
        let document = serde_json::json!({
            constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::GET_LOWERCASE: {
                constants_str::OPERATION_ID_JSON: constants_str::TEST_OPENAPI_OPERATION_ID,
                constants_str::RESPONSES: { constants_str::STATUS_OK: { constants_str::OPENAPI_CONTENT: {
                    constants_str::APPLICATION_JSON: { constants_str::JSON_SCHEMA: { constants_str::JSON_TYPE: constants_str::OBJECT }}
                }}}
            }}}
        });
        let expectation = crate::open_api_operation_expectation::OpenApiOperationExpectation::new(
            frontend_contract::route_metadata::RouteMetadata::new(
                frontend_contract::route_method::RouteMethod::Get,
                constants_str::TEST_OPENAPI_OPERATION_ID.into(),
                constants_str::TEST_OPENAPI_PATH.into(),
            ),
            crate::open_api_response_status::OpenApiResponseStatus::try_from(200u16).expect("9f6e9528 operation_security_status_and_content_type_are_checked invariant must hold"),
            constants_str::APPLICATION_JSON.into(),
            crate::open_api_security_expectation::OpenApiSecurityExpectation::Public,
        );
        let (_, metadata, _, status) = expectation.parts();
        assert_eq!(metadata.path().as_ref(), constants_str::TEST_OPENAPI_PATH);
        assert_eq!(*status, 200u16);
        assert!(matches!(
            crate::validate_openapi_operations::validate_openapi_operations(
                &document,
                &[expectation]
            ),
            Ok(())
        ));
    }

    #[test]
    fn test_payload_schema_checks_required_fields_and_additional_properties() {
        let document = serde_json::json!({
            constants_str::COMPONENTS: { constants_str::SCHEMAS: {}}
        });
        let schema = serde_json::json!({
            constants_str::JSON_TYPE: constants_str::OBJECT,
            constants_str::REQUIRED: [constants_str::NAME],
            constants_str::PROPERTIES: {
                constants_str::NAME: { constants_str::JSON_TYPE: constants_str::STRING }
            },
            constants_str::ADDITIONAL_PROPERTIES: false
        });
        assert!(matches!(
            crate::validate_openapi_json_payload::validate_openapi_json_payload(
                &serde_json::json!({constants_str::NAME: constants_str::TEST_OPENAPI_SCHEMA}),
                &schema,
                &document,
            ),
            Ok(())
        ));
        assert!(matches!(
            crate::validate_openapi_json_payload::validate_openapi_json_payload(
                &serde_json::json!({}),
                &schema,
                &document
            ),
            Err(
                crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
                    crate::open_api_schema_mismatch::OpenApiSchemaMismatch::RequiredProperty
                )
            )
        ));
    }
}
