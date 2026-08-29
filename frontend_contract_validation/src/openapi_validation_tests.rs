#[cfg(test)]
mod tests {
    #[test]
    fn valid_document_matches_runtime_route_and_references() {
        let document = serde_json::json!({
            constants_str::catalog::PATHS: { constants_str::test_fixtures::TEST_OPENAPI_PATH: { constants_str::test_fixtures::GET_LOWERCASE: {
                constants_str::test_fixtures::OPERATION_ID_JSON: constants_str::test_fixtures::TEST_OPENAPI_OPERATION_ID,
                constants_str::catalog::RESPONSES: { constants_str::test_fixtures::STATUS_OK: { constants_str::test_fixtures::OPENAPI_CONTENT: { constants_str::catalog::APPLICATION_JSON: {
                    constants_str::test_fixtures::JSON_SCHEMA: { constants_str::catalog::DOLLAR_REF: constants_str::test_fixtures::TEST_OPENAPI_SCHEMA_REF }
                }}}}
            }}},
            constants_str::catalog::COMPONENTS: { constants_str::catalog::SCHEMAS: { constants_str::test_fixtures::TEST_OPENAPI_SCHEMA: { constants_str::test_fixtures::JSON_TYPE: constants_str::test_fixtures::OBJECT }}}
        });
        let routes = [frontend_contract::route_metadata::RouteMetadata::new(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::test_fixtures::TEST_OPENAPI_OPERATION_ID.into(),
            constants_str::test_fixtures::TEST_OPENAPI_PATH.into(),
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
    fn dangling_reference_is_rejected() {
        let document = serde_json::json!({
            constants_str::catalog::PATHS: {},
            constants_str::catalog::COMPONENTS: { constants_str::catalog::SCHEMAS: { constants_str::test_fixtures::TEST_OPENAPI_SCHEMA: {
                constants_str::catalog::DOLLAR_REF: constants_str::test_fixtures::TEST_OPENAPI_MISSING_SCHEMA_REF
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
    fn operation_security_status_and_content_type_are_checked() {
        let document = serde_json::json!({
            constants_str::catalog::PATHS: { constants_str::test_fixtures::TEST_OPENAPI_PATH: { constants_str::test_fixtures::GET_LOWERCASE: {
                constants_str::test_fixtures::OPERATION_ID_JSON: constants_str::test_fixtures::TEST_OPENAPI_OPERATION_ID,
                constants_str::catalog::RESPONSES: { constants_str::test_fixtures::STATUS_OK: { constants_str::test_fixtures::OPENAPI_CONTENT: {
                    constants_str::catalog::APPLICATION_JSON: { constants_str::test_fixtures::JSON_SCHEMA: { constants_str::test_fixtures::JSON_TYPE: constants_str::test_fixtures::OBJECT }}
                }}}
            }}}
        });
        let expectation = crate::open_api_operation_expectation::OpenApiOperationExpectation::new(
            frontend_contract::route_metadata::RouteMetadata::new(
                frontend_contract::route_method::RouteMethod::Get,
                constants_str::test_fixtures::TEST_OPENAPI_OPERATION_ID.into(),
                constants_str::test_fixtures::TEST_OPENAPI_PATH.into(),
            ),
            crate::open_api_response_status::OpenApiResponseStatus::try_from(200u16).expect("9f6e9528 operation_security_status_and_content_type_are_checked invariant must hold"),
            constants_str::catalog::APPLICATION_JSON.into(),
            crate::open_api_security_expectation::OpenApiSecurityExpectation::Public,
        );
        assert!(matches!(
            crate::validate_openapi_operations::validate_openapi_operations(
                &document,
                &[expectation]
            ),
            Ok(())
        ));
    }

    #[test]
    fn payload_schema_checks_required_fields_and_additional_properties() {
        let document = serde_json::json!({
            constants_str::catalog::COMPONENTS: { constants_str::catalog::SCHEMAS: {}}
        });
        let schema = serde_json::json!({
            constants_str::test_fixtures::JSON_TYPE: constants_str::test_fixtures::OBJECT,
            constants_str::test_fixtures::REQUIRED: [constants_str::catalog::NAME],
            constants_str::catalog::PROPERTIES: {
                constants_str::catalog::NAME: { constants_str::test_fixtures::JSON_TYPE: constants_str::catalog::STRING }
            },
            constants_str::test_fixtures::ADDITIONAL_PROPERTIES: false
        });
        assert!(matches!(
            crate::validate_openapi_json_payload::validate_openapi_json_payload(
                &serde_json::json!({constants_str::catalog::NAME: constants_str::test_fixtures::TEST_OPENAPI_SCHEMA}),
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
