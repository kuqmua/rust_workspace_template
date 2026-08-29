pub use super::open_api_contract_text::*;
pub use super::open_api_operation_expectation::*;
pub use super::open_api_operation_validation_error::*;
pub use super::open_api_payload_validation_error::*;
pub use super::open_api_response_status::*;
pub use super::open_api_schema_mismatch::*;
pub(crate) use super::open_api_schema_references_b_tree_set::OpenApiSchemaReferencesBTreeSet;
pub use super::open_api_security_expectation::*;
pub use super::open_api_validation_error::*;
pub(crate) use super::openapi_schema_references::openapi_schema_references;
pub use super::runtime_routes_ref::*;
pub use super::serde_json_open_api_serialization_error::*;
pub use super::validate_openapi_contract::*;
pub use super::validate_openapi_json_payload::*;
pub use super::validate_openapi_operations::*;
pub use super::validate_openapi_schema_references::*;
#[cfg(test)]
mod tests {
    #[test]
    fn valid_document_matches_runtime_route_and_references() {
        let document = serde_json::json!({
            constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::GET_LOWERCASE: {
                constants_str::OPERATION_ID_JSON: constants_str::TEST_OPENAPI_OPERATION_ID,
                constants_str::RESPONSES: { constants_str::STATUS_OK: { constants_str::OPENAPI_CONTENT: { constants_str::APPLICATION_JSON: {
                    constants_str::JSON_SCHEMA: { constants_str::DOLLAR_REF: constants_str::TEST_OPENAPI_SCHEMA_REF }
                }}}}
            }}},
            constants_str::COMPONENTS: { constants_str::SCHEMAS: { constants_str::TEST_OPENAPI_SCHEMA: { constants_str::JSON_TYPE: constants_str::OBJECT }}}
        });
        let routes = [frontend_contract::RouteMetadata::new(
            frontend_contract::RouteMethod::Get,
            constants_str::TEST_OPENAPI_OPERATION_ID.into(),
            constants_str::TEST_OPENAPI_PATH.into(),
        )];
        assert!(matches!(
            super::validate_openapi_contract(&document, routes.as_slice().into()),
            Ok(())
        ));
    }

    #[test]
    fn dangling_reference_is_rejected() {
        let document = serde_json::json!({
            constants_str::PATHS: {},
            constants_str::COMPONENTS: { constants_str::SCHEMAS: { constants_str::TEST_OPENAPI_SCHEMA: {
                constants_str::DOLLAR_REF: constants_str::TEST_OPENAPI_MISSING_SCHEMA_REF
            }}}
        });
        assert!(matches!(
            super::validate_openapi_contract(&document, (&[][..]).into()),
            Err(super::OpenApiValidationError::MissingSchemaReference(_))
        ));
        assert!(matches!(
            super::validate_openapi_schema_references(&document),
            Err(super::OpenApiValidationError::MissingSchemaReference(_))
        ));
    }

    #[test]
    fn operation_security_status_and_content_type_are_checked() {
        let document = serde_json::json!({
            constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::GET_LOWERCASE: {
                constants_str::OPERATION_ID_JSON: constants_str::TEST_OPENAPI_OPERATION_ID,
                constants_str::RESPONSES: { constants_str::STATUS_OK: { constants_str::OPENAPI_CONTENT: {
                    constants_str::APPLICATION_JSON: { constants_str::JSON_SCHEMA: { constants_str::JSON_TYPE: constants_str::OBJECT }}
                }}}
            }}}
        });
        let expectation = super::OpenApiOperationExpectation::new(
            frontend_contract::RouteMetadata::new(
                frontend_contract::RouteMethod::Get,
                constants_str::TEST_OPENAPI_OPERATION_ID.into(),
                constants_str::TEST_OPENAPI_PATH.into(),
            ),
            super::OpenApiResponseStatus::try_from(200u16).expect("9f6e9528 operation_security_status_and_content_type_are_checked invariant must hold"),
            constants_str::APPLICATION_JSON.into(),
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
            super::validate_openapi_json_payload(
                &serde_json::json!({constants_str::NAME: constants_str::TEST_OPENAPI_SCHEMA}),
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
