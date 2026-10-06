#[test]
fn test_nullable_payload_preserves_type_checks_and_reference_resolution() {
    let document = serde_json::json!({
        constants_str::COMPONENTS: {constants_str::SCHEMAS: {
            constants_str::TEST_OPENAPI_SCHEMA: {
                constants_str::JSON_TYPE: constants_str::STRING,
                constants_str::NULLABLE: true,
            }
        }}
    });
    let nullable_schema = serde_json::json!({
        constants_str::JSON_TYPE: constants_str::STRING,
        constants_str::NULLABLE: true,
    });
    let nonnullable_schema = serde_json::json!({
        constants_str::JSON_TYPE: constants_str::STRING,
        constants_str::NULLABLE: false,
    });
    let invalid_nullable_schema = serde_json::json!({
        constants_str::JSON_TYPE: constants_str::STRING,
        constants_str::NULLABLE: constants_str::X,
    });
    let reference_schema = serde_json::json!({
        constants_str::DOLLAR_REF: constants_str::TEST_OPENAPI_SCHEMA_REF,
    });
    [
        (&nullable_schema, serde_json::json!(null), None),
        (&nullable_schema, serde_json::json!(constants_str::X), None),
        (&nullable_schema, serde_json::json!(false), Some(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Type)),
        (&nonnullable_schema, serde_json::json!(null), Some(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Type)),
        (&invalid_nullable_schema, serde_json::json!(null), Some(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Type)),
        (&reference_schema, serde_json::json!(null), None),
    ].into_iter().fold((), |(), (schema, payload, expected)| {
        let result = crate::validate_openapi_json_payload::validate_openapi_json_payload(&payload, schema, &document);
        match expected {
            None => assert!(matches!(result, Ok(()))),
            Some(expected_mismatch) => assert!(matches!(result, Err(crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(actual)) if actual == expected_mismatch)),
        }
    });
    let missing_reference_schema = serde_json::json!({
        constants_str::DOLLAR_REF: constants_str::TEST_OPENAPI_SCHEMA_REF,
        constants_str::NULLABLE: true,
    });
    assert!(matches!(
        crate::validate_openapi_json_payload::validate_openapi_json_payload(
            &serde_json::json!(null),
            &missing_reference_schema,
            &serde_json::json!({}),
        ),
        Err(
            crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
                crate::open_api_schema_mismatch::OpenApiSchemaMismatch::MissingReference
            )
        )
    ));
}
