#[cfg(test)]
mod tests {
    #[test]
    fn test_payload_additional_property_policies_reject_unknown_fields_and_skip_scalars() {
        let document = serde_json::json!({});
        assert!([
            (serde_json::json!({ constants_str::ADDITIONAL_PROPERTIES: false }), serde_json::json!({ constants_str::NAME: 1u8 }), false),
            (serde_json::json!({ constants_str::PROPERTIES: { constants_str::NAME: {} }, constants_str::ADDITIONAL_PROPERTIES: false }), serde_json::json!({ constants_str::NAME: 1u8, constants_str::X: true }), false),
            (serde_json::json!({ constants_str::ADDITIONAL_PROPERTIES: { constants_str::JSON_TYPE: constants_str::STRING } }), serde_json::json!(1u8), true),
            (serde_json::json!({ constants_str::ADDITIONAL_PROPERTIES: { constants_str::JSON_TYPE: constants_str::STRING } }), serde_json::json!(null), true),
        ].into_iter().all(|(schema, payload, accepted)| {
            let result = crate::validate_openapi_json_payload::validate_openapi_json_payload(&payload, &schema, &document);
            if accepted { matches!(result, Ok(())) } else { matches!(result, Err(crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::AdditionalProperty))) }
        }));
    }

    #[test]
    fn test_payload_missing_schema_references_report_exact_mismatch() {
        let payload = serde_json::json!(null);
        assert!([
            (serde_json::json!({ constants_str::DOLLAR_REF: constants_str::TEST_OPENAPI_SCHEMA_REF }), serde_json::json!({})),
            (serde_json::json!({ constants_str::DOLLAR_REF: constants_str::TEST_OPENAPI_SCHEMA_REF }), serde_json::json!({ constants_str::COMPONENTS: { constants_str::SCHEMAS: {} } })),
            (serde_json::json!({ constants_str::DOLLAR_REF: constants_str::TEST_OPENAPI_SCHEMA_REF }), serde_json::json!({ constants_str::COMPONENTS: { constants_str::SCHEMAS: null } })),
            (serde_json::json!({ constants_str::DOLLAR_REF: constants_str::TEST_OPENAPI_PATH }), serde_json::json!({ constants_str::COMPONENTS: { constants_str::SCHEMAS: { constants_str::TEST_OPENAPI_SCHEMA: {} } } })),
        ].into_iter().all(|(schema, document)| matches!(crate::validate_openapi_json_payload::validate_openapi_json_payload(&payload, &schema, &document), Err(crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::MissingReference)))));
    }

    #[test]
    fn test_payload_null_and_numeric_types_preserve_json_kind_and_unsigned_range() {
        let document = serde_json::json!({});
        assert!([
            (constants_str::JSON_NULL, serde_json::json!(null), true),
            (constants_str::JSON_NULL, serde_json::json!(false), false),
            (constants_str::NUMBER, serde_json::json!(1u8), true),
            (constants_str::NUMBER, serde_json::json!(1.5f64), true),
            (constants_str::NUMBER, serde_json::json!(u64::MAX), true),
            (constants_str::NUMBER, serde_json::json!(constants_str::X), false),
            (constants_str::INTEGER, serde_json::json!(u64::MAX), true),
            (constants_str::INTEGER, serde_json::json!(i64::MIN), true),
            (constants_str::INTEGER, serde_json::json!(1.5f64), false),
            (constants_str::INTEGER, serde_json::json!(null), false),
            (constants_str::X, serde_json::json!(null), false),
        ].into_iter().all(|(kind, payload, accepted)| {
            let schema = serde_json::json!({ constants_str::JSON_TYPE: kind });
            let result = crate::validate_openapi_json_payload::validate_openapi_json_payload(&payload, &schema, &document);
            if accepted { matches!(result, Ok(())) } else { matches!(result, Err(crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Type))) }
        }));
    }

    #[test]
    fn test_payload_additional_property_schema_skips_declared_fields_and_propagates_mismatch() {
        let document = serde_json::json!({});
        assert!([true, false].into_iter().all(|declared_properties| {
            let schema = if declared_properties {
                serde_json::json!({ constants_str::JSON_TYPE: constants_str::OBJECT, constants_str::PROPERTIES: { constants_str::NAME: { constants_str::JSON_TYPE: constants_str::INTEGER } }, constants_str::ADDITIONAL_PROPERTIES: { constants_str::JSON_TYPE: constants_str::STRING } })
            } else {
                serde_json::json!({ constants_str::JSON_TYPE: constants_str::OBJECT, constants_str::ADDITIONAL_PROPERTIES: { constants_str::JSON_TYPE: constants_str::STRING } })
            };
            let payload = if declared_properties {
                serde_json::json!({ constants_str::NAME: 1u8, constants_str::ITEMS: constants_str::TEST_JSON_FIRST, constants_str::X: constants_str::TEST_JSON_SECOND })
            } else {
                serde_json::json!({ constants_str::ITEMS: constants_str::TEST_JSON_FIRST, constants_str::X: constants_str::TEST_JSON_SECOND })
            };
            assert!(matches!(crate::validate_openapi_json_payload::validate_openapi_json_payload(&payload, &schema, &document), Ok(())));
            let invalid_payload = if declared_properties {
                serde_json::json!({ constants_str::NAME: 1u8, constants_str::ITEMS: constants_str::TEST_JSON_FIRST, constants_str::X: false })
            } else {
                serde_json::json!({ constants_str::ITEMS: constants_str::TEST_JSON_FIRST, constants_str::X: false })
            };
            matches!(crate::validate_openapi_json_payload::validate_openapi_json_payload(&invalid_payload, &schema, &document), Err(crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Type)))
        }));
    }

    #[test]
    fn test_openapi_operation_serialization_failure_preserves_original_diagnostic() {
        let document = std::collections::BTreeMap::from([([1u8, 2u8], 3u8)]);
        let expected_diagnostic = serde_json::to_value(&document)
            .err()
            .map(|error| error.to_string());
        assert!(expected_diagnostic.is_some());
        assert!(crate::validate_openapi_operations::validate_openapi_operations(&document, &[]).is_err_and(|error| {
            let crate::open_api_operation_validation_error::OpenApiOperationValidationError::DocumentSerialization(source) = error else { return false; };
            Some(source.to_string()) == expected_diagnostic
        }));
    }

    #[test]
    fn test_openapi_operation_missing_response_parts_report_exact_validation_stage() {
        let status_result =
            crate::open_api_response_status::OpenApiResponseStatus::try_from(200u16);
        assert!(status_result.as_ref().err().is_none());
        let Ok(status) = status_result else {
            return;
        };
        let expectation = crate::open_api_operation_expectation::OpenApiOperationExpectation::new(
            frontend_contract::route_metadata::RouteMetadata::new(
                frontend_contract::route_method::RouteMethod::Get,
                constants_str::TEST_OPENAPI_OPERATION_ID.into(),
                constants_str::TEST_OPENAPI_PATH.into(),
            ),
            status,
            constants_str::APPLICATION_JSON.into(),
            crate::open_api_security_expectation::OpenApiSecurityExpectation::Public,
        );
        assert!([
            (serde_json::json!({}), crate::open_api_operation_validation_error::OpenApiOperationValidationError::MissingOperation),
            (serde_json::json!({ constants_str::PATHS: {} }), crate::open_api_operation_validation_error::OpenApiOperationValidationError::MissingOperation),
            (serde_json::json!({ constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: {} } }), crate::open_api_operation_validation_error::OpenApiOperationValidationError::MissingOperation),
            (serde_json::json!({ constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::GET_LOWERCASE: {} } } }), crate::open_api_operation_validation_error::OpenApiOperationValidationError::MissingResponseStatus),
            (serde_json::json!({ constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::GET_LOWERCASE: { constants_str::RESPONSES: {} } } } }), crate::open_api_operation_validation_error::OpenApiOperationValidationError::MissingResponseStatus),
            (serde_json::json!({ constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::GET_LOWERCASE: { constants_str::RESPONSES: { constants_str::STATUS_OK: {} } } } } }), crate::open_api_operation_validation_error::OpenApiOperationValidationError::MissingContentType),
            (serde_json::json!({ constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::GET_LOWERCASE: { constants_str::RESPONSES: { constants_str::STATUS_OK: { constants_str::OPENAPI_CONTENT: null } } } } } }), crate::open_api_operation_validation_error::OpenApiOperationValidationError::MissingContentType),
            (serde_json::json!({ constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::GET_LOWERCASE: { constants_str::RESPONSES: { constants_str::STATUS_OK: { constants_str::OPENAPI_CONTENT: {} } } } } } }), crate::open_api_operation_validation_error::OpenApiOperationValidationError::MissingContentType),
            (serde_json::json!({ constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::GET_LOWERCASE: { constants_str::RESPONSES: { constants_str::STATUS_OK: { constants_str::OPENAPI_CONTENT: { constants_str::APPLICATION_JSON: {} } } } } } } }), crate::open_api_operation_validation_error::OpenApiOperationValidationError::MissingResponseSchema),
        ].into_iter().all(|(document, expected)| crate::validate_openapi_operations::validate_openapi_operations(&document, &[expectation]).is_err_and(|error| std::mem::discriminant(&error) == std::mem::discriminant(&expected))));
    }

    #[test]
    fn test_openapi_internal_reference_operations_preserve_serialization_diagnostics() {
        let document = std::collections::BTreeMap::from([([1u8, 2u8], 3u8)]);
        let expected_diagnostic = serde_json::to_value(&document)
            .err()
            .map(|error| error.to_string());
        assert!(expected_diagnostic.is_some());
        let references =
            crate::open_api_schema_references_b_tree_set::OpenApiSchemaReferencesBTreeSet::from(
                std::collections::BTreeSet::new(),
            );
        assert!(
            [
                crate::openapi_schema_references::openapi_schema_references(&document).map(|_| ()),
                references.validate(&document),
            ]
            .into_iter()
            .all(|result| result.is_err_and(|error| {
                let crate::open_api_validation_error::OpenApiValidationError::DocumentSerialization(
                    source,
                ) = error
                else {
                    return false;
                };
                Some(source.to_string()) == expected_diagnostic
            }))
        );
    }

    #[test]
    fn test_openapi_reference_set_validation_checks_document_and_exact_schema_names() {
        let name_result = crate::open_api_contract_text::OpenApiContractText::try_from(
            constants_str::TEST_OPENAPI_SCHEMA.to_owned(),
        );
        assert!(name_result.is_ok());
        let Ok(schema_name) = name_result else {
            return;
        };
        let references =
            crate::open_api_schema_references_b_tree_set::OpenApiSchemaReferencesBTreeSet::from(
                std::collections::BTreeSet::from([schema_name]),
            );
        assert!(
            references
                .validate(&serde_json::json!({}))
                .is_err_and(|error| matches!(
                    error,
                    crate::open_api_validation_error::OpenApiValidationError::MissingSchemas
                ))
        );
        assert!(references.validate(&serde_json::json!({ constants_str::COMPONENTS: { constants_str::SCHEMAS: {} } })).is_err_and(|error| matches!(error, crate::open_api_validation_error::OpenApiValidationError::MissingSchemaReference(name) if name.as_ref() == constants_str::TEST_OPENAPI_SCHEMA)));
        assert!(matches!(references.validate(&serde_json::json!({ constants_str::COMPONENTS: { constants_str::SCHEMAS: { constants_str::TEST_OPENAPI_SCHEMA: {} } } })), Ok(())));
    }

    #[test]
    fn test_openapi_reference_invalid_escape_names_are_rejected_even_when_pointer_resolves() {
        assert!([Some('2'), None].into_iter().all(|suffix| {
            let invalid_name = constants_str::TEST_OPENAPI_SCHEMA.chars().chain(std::iter::once('~')).chain(suffix).collect::<String>();
            let reference = [constants_str::COMPONENTS_SCHEMAS, invalid_name.as_str()].concat();
            let document = serde_json::json!({
                constants_str::DOLLAR_REF: reference,
                constants_str::COMPONENTS: { constants_str::SCHEMAS: { (invalid_name.as_str()): {} } }
            });
            crate::validate_openapi_schema_references::validate_openapi_schema_references(&document)
                .is_err_and(|error| matches!(error, crate::open_api_validation_error::OpenApiValidationError::MissingSchemaReference(name) if name.as_ref() == invalid_name))
        }));
    }

    #[test]
    fn test_openapi_reference_oversized_names_retain_missing_existing_and_escape_failures() {
        let oversized_name = constants_str::X.repeat(constants_usize::VALUE_1_048_576 + 1usize);
        let invalid_oversized_name = oversized_name.chars().chain(['~', '2']).collect::<String>();
        assert!([
            (oversized_name.as_str(), false),
            (oversized_name.as_str(), true),
            (invalid_oversized_name.as_str(), true),
        ].into_iter().all(|(name, schema_present)| {
            let expected_diagnostic = crate::open_api_contract_text::OpenApiContractText::try_from(name.to_owned()).err().map(|error| error.to_string());
            assert!(expected_diagnostic.is_some());
            let schemas = if schema_present { serde_json::json!({ (name): {} }) } else { serde_json::json!({}) };
            let reference = [constants_str::COMPONENTS_SCHEMAS, name].concat();
            let document = serde_json::json!({ constants_str::DOLLAR_REF: reference, constants_str::COMPONENTS: { constants_str::SCHEMAS: schemas } });
            crate::validate_openapi_schema_references::validate_openapi_schema_references(&document).is_err_and(|error| {
                let crate::open_api_validation_error::OpenApiValidationError::TextTooLong(source) = error else { return false; };
                Some(source.to_string()) == expected_diagnostic
            })
        }));
    }

    #[test]
    fn test_openapi_runtime_oversized_error_fields_preserve_conversion_failures() {
        const FIXTURE_BYTES: &[u8; constants_usize::VALUE_1_048_576 + 1usize] =
            &[1u8; constants_usize::VALUE_1_048_576 + 1usize];
        let static_text_result = std::str::from_utf8(FIXTURE_BYTES);
        assert!(
            static_text_result
                .as_ref()
                .is_ok_and(|text| text.len() == constants_usize::VALUE_1_048_576 + 1usize)
        );
        let Ok(static_text) = static_text_result else {
            return;
        };
        let expected_diagnostic =
            crate::open_api_contract_text::OpenApiContractText::try_from(static_text.to_owned())
                .err()
                .map(|error| error.to_string());
        assert!(expected_diagnostic.is_some());
        let oversized_path_route = frontend_contract::route_metadata::RouteMetadata::new(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::TEST_OPENAPI_OPERATION_ID.into(),
            static_text.into(),
        );
        let oversized_identifier_route = frontend_contract::route_metadata::RouteMetadata::new(
            frontend_contract::route_method::RouteMethod::Get,
            static_text.into(),
            constants_str::TEST_OPENAPI_PATH.into(),
        );
        let document_for_route =
            |path_contract_str: frontend_contract::contract_str::ContractStr,
             operation_id_contract_str: frontend_contract::contract_str::ContractStr| {
                serde_json::json!({
                    constants_str::PATHS: { (path_contract_str.as_ref()): { constants_str::GET_LOWERCASE: { constants_str::OPERATION_ID_JSON: operation_id_contract_str.as_ref() } } },
                    constants_str::COMPONENTS: { constants_str::SCHEMAS: {} }
                })
            };
        assert!([
            (document_for_route(static_text.into(), constants_str::TEST_OPENAPI_OPERATION_ID.into()), [oversized_path_route, oversized_path_route], 2usize),
            (serde_json::json!({ constants_str::PATHS: {}, constants_str::COMPONENTS: { constants_str::SCHEMAS: {} } }), [oversized_path_route, oversized_path_route], 1usize),
            (document_for_route(static_text.into(), constants_str::X.into()), [oversized_path_route, oversized_path_route], 1usize),
            (document_for_route(constants_str::TEST_OPENAPI_PATH.into(), constants_str::TEST_OPENAPI_OPERATION_ID.into()), [oversized_identifier_route, oversized_identifier_route], 1usize),
        ].into_iter().all(|(document, routes, route_count)| {
            let Some(selected_routes) = routes.get(..route_count) else { return false; };
            crate::validate_openapi_contract::validate_openapi_contract(&document, selected_routes.into()).is_err_and(|error| {
                let crate::open_api_validation_error::OpenApiValidationError::TextTooLong(source) = error else { return false; };
                Some(source.to_string()) == expected_diagnostic
            })
        }));
    }

    #[test]
    fn test_openapi_route_document_oversized_error_fields_retain_validation_diagnostics() {
        let oversized = constants_str::X.repeat(constants_usize::VALUE_1_048_576 + 1usize);
        let expected_diagnostic =
            crate::open_api_contract_text::OpenApiContractText::try_from(oversized.clone())
                .err()
                .map(|error| error.to_string());
        assert!(expected_diagnostic.is_some());
        let document_from_paths = |paths| {
            serde_json::json!({
                constants_str::PATHS: paths,
                constants_str::COMPONENTS: { constants_str::SCHEMAS: {} }
            })
        };
        let route = frontend_contract::route_metadata::RouteMetadata::new(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::TEST_OPENAPI_OPERATION_ID.into(),
            constants_str::TEST_OPENAPI_PATH.into(),
        );
        let runtime_routes = [route];
        assert!([
            (serde_json::json!({ constants_str::PATHS: {}, constants_str::COMPONENTS: { constants_str::SCHEMAS: { (oversized.as_str()): {} } } }), false),
            (document_from_paths(serde_json::json!({ (oversized.as_str()): null })), false),
            (document_from_paths(serde_json::json!({ (oversized.as_str()): { constants_str::GET_LOWERCASE: {} } })), false),
            (document_from_paths(serde_json::json!({ (oversized.as_str()): {
                constants_str::GET: { constants_str::OPERATION_ID_JSON: constants_str::TEST_OPENAPI_OPERATION_ID },
                constants_str::GET_LOWERCASE: { constants_str::OPERATION_ID_JSON: constants_str::TEST_OPENAPI_OPERATION_ID }
            } })), false),
            (document_from_paths(serde_json::json!({ (oversized.as_str()): { constants_str::GET_LOWERCASE: { constants_str::OPERATION_ID_JSON: constants_str::TEST_OPENAPI_OPERATION_ID } } })), false),
            (document_from_paths(serde_json::json!({ constants_str::TEST_OPENAPI_PATH: { constants_str::GET_LOWERCASE: { constants_str::OPERATION_ID_JSON: oversized.as_str() } } })), true),
        ].into_iter().all(|(document, runtime_route_enabled)| {
            let routes = if runtime_route_enabled { runtime_routes.as_slice() } else { &[][..] };
            crate::validate_openapi_contract::validate_openapi_contract(&document, routes.into()).is_err_and(|error| {
                let crate::open_api_validation_error::OpenApiValidationError::TextTooLong(source) = error else { return false; };
                Some(source.to_string()) == expected_diagnostic
            })
        }));
    }

    #[test]
    fn test_openapi_route_catalog_reports_missing_sections_and_unused_schema_name() {
        let missing_paths =
            serde_json::json!({ constants_str::COMPONENTS: { constants_str::SCHEMAS: {} } });
        assert!(matches!(
            crate::validate_openapi_contract::validate_openapi_contract(
                &missing_paths,
                (&[][..]).into()
            ),
            Err(crate::open_api_validation_error::OpenApiValidationError::MissingPaths)
        ));
        let missing_schemas = serde_json::json!({ constants_str::PATHS: {} });
        assert!(matches!(
            crate::validate_openapi_contract::validate_openapi_contract(
                &missing_schemas,
                (&[][..]).into()
            ),
            Err(crate::open_api_validation_error::OpenApiValidationError::MissingSchemas)
        ));
        let unused_schema = serde_json::json!({
            constants_str::PATHS: {},
            constants_str::COMPONENTS: { constants_str::SCHEMAS: { constants_str::TEST_OPENAPI_SCHEMA: {} } }
        });
        assert!(crate::validate_openapi_contract::validate_openapi_contract(&unused_schema, (&[][..]).into()).is_err_and(|error| matches!(error, crate::open_api_validation_error::OpenApiValidationError::UnusedSchema(name) if name.as_ref() == constants_str::TEST_OPENAPI_SCHEMA)));
    }

    #[test]
    fn test_openapi_route_catalog_mismatches_preserve_method_path_and_identifiers() {
        let document_for_identifier = |operation_id| {
            serde_json::json!({
                constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::GET_LOWERCASE: { constants_str::OPERATION_ID_JSON: operation_id } } },
                constants_str::COMPONENTS: { constants_str::SCHEMAS: {} }
            })
        };
        let route = frontend_contract::route_metadata::RouteMetadata::new(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::TEST_OPENAPI_OPERATION_ID.into(),
            constants_str::TEST_OPENAPI_PATH.into(),
        );
        let undocumented = serde_json::json!({ constants_str::PATHS: {}, constants_str::COMPONENTS: { constants_str::SCHEMAS: {} } });
        assert!(crate::validate_openapi_contract::validate_openapi_contract(&undocumented, [route].as_slice().into()).is_err_and(|error| matches!(error,
            crate::open_api_validation_error::OpenApiValidationError::RuntimeRouteMissing(method, path)
            if method.as_ref() == constants_str::GET && path.as_ref() == constants_str::TEST_OPENAPI_PATH
        )));
        let mismatched = document_for_identifier(constants_str::X);
        assert!(crate::validate_openapi_contract::validate_openapi_contract(&mismatched, [route].as_slice().into()).is_err_and(|error| matches!(error,
            crate::open_api_validation_error::OpenApiValidationError::OperationIdMismatch(method, path, expected, observed)
            if method.as_ref() == constants_str::GET && path.as_ref() == constants_str::TEST_OPENAPI_PATH
                && expected.as_ref() == constants_str::TEST_OPENAPI_OPERATION_ID && observed.as_ref() == constants_str::X
        )));
        let undocumented_runtime =
            document_for_identifier(constants_str::TEST_OPENAPI_OPERATION_ID);
        assert!(crate::validate_openapi_contract::validate_openapi_contract(&undocumented_runtime, (&[][..]).into()).is_err_and(|error| matches!(error,
            crate::open_api_validation_error::OpenApiValidationError::OpenApiRouteMissing(method, path)
            if method.as_ref() == constants_str::GET && path.as_ref() == constants_str::TEST_OPENAPI_PATH
        )));
    }

    #[test]
    fn test_openapi_route_contract_preserves_document_serialization_diagnostic() {
        let document = std::collections::BTreeMap::from([([1u8, 2u8], 3u8)]);
        let expected_diagnostic = serde_json::to_value(&document)
            .err()
            .map(|error| error.to_string());
        assert!(expected_diagnostic.is_some());
        assert!(
            crate::validate_openapi_contract::validate_openapi_contract(
                &document,
                (&[][..]).into()
            )
            .is_err_and(|error| {
                let crate::open_api_validation_error::OpenApiValidationError::DocumentSerialization(
                    source,
                ) = error
                else {
                    return false;
                };
                Some(source.to_string()) == expected_diagnostic
            })
        );
    }

    #[test]
    fn test_openapi_route_contract_missing_operation_identifiers_preserve_method_and_path() {
        assert!([
            serde_json::json!({}),
            serde_json::json!({ constants_str::OPERATION_ID_JSON: null }),
            serde_json::json!({ constants_str::OPERATION_ID_JSON: 1u8 }),
            serde_json::json!(null),
        ].into_iter().all(|operation| {
            let document = serde_json::json!({
                constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::GET_LOWERCASE: operation } },
                constants_str::COMPONENTS: { constants_str::SCHEMAS: {} }
            });
            crate::validate_openapi_contract::validate_openapi_contract(&document, (&[][..]).into())
                .is_err_and(|error| matches!(error,
                    crate::open_api_validation_error::OpenApiValidationError::MissingOperationId(method, path)
                    if method.as_ref() == constants_str::GET_LOWERCASE && path.as_ref() == constants_str::TEST_OPENAPI_PATH
                ))
        }));
    }

    fn test_drop_nested_json_values<Values>(values: Values)
    where
        Values: IntoIterator<Item = serde_json::Value>,
    {
        let mut pending = values.into_iter().collect::<Vec<serde_json::Value>>();
        while let Some(value) = pending.pop() {
            match value {
                serde_json::Value::Array(items) => pending.extend(items),
                serde_json::Value::Object(fields) => {
                    pending.extend(fields.into_iter().map(|(_, child)| child));
                }
                serde_json::Value::Bool(_)
                | serde_json::Value::Null
                | serde_json::Value::Number(_)
                | serde_json::Value::String(_) => {}
            }
        }
    }

    #[test]
    fn test_deep_inline_schema_and_document_are_borrowed_without_serialization() {
        let compose = |leaf| {
            (0usize..8_192usize).fold(leaf, |child, _index| {
                serde_json::Value::Object(
                    std::iter::once((
                        constants_str::ALL_OF.to_owned(),
                        serde_json::Value::Array(vec![child]),
                    ))
                    .collect(),
                )
            })
        };
        let schema =
            compose(serde_json::json!({ constants_str::JSON_TYPE: constants_str::INTEGER }));
        let document_schema =
            compose(serde_json::json!({ constants_str::JSON_TYPE: constants_str::INTEGER }));
        let document = serde_json::Value::Object(
            std::iter::once((
                constants_str::COMPONENTS.to_owned(),
                serde_json::Value::Object(
                    std::iter::once((
                        constants_str::SCHEMAS.to_owned(),
                        serde_json::Value::Object(
                            std::iter::once((
                                constants_str::TEST_OPENAPI_SCHEMA.to_owned(),
                                document_schema,
                            ))
                            .collect(),
                        ),
                    ))
                    .collect(),
                ),
            ))
            .collect(),
        );
        let outcome = crate::validate_openapi_json_payload::validate_openapi_json_payload(
            &serde_json::Value::from(0u8),
            &schema,
            &document,
        );
        test_drop_nested_json_values([schema, document]);
        assert!(matches!(outcome, Ok(())));
    }

    #[test]
    fn test_deep_enum_and_const_comparisons_do_not_recurse() {
        let nested_value = |leaf| {
            (0usize..8_192usize).fold(serde_json::Value::from(leaf), |child, _index| {
                serde_json::Value::Array(vec![child])
            })
        };
        let payload = nested_value(0u8);
        let constant_schema = serde_json::Value::Object(
            std::iter::once((constants_str::CONST.to_owned(), nested_value(0u8))).collect(),
        );
        let mismatch_schema = serde_json::Value::Object(
            std::iter::once((constants_str::CONST.to_owned(), nested_value(1u8))).collect(),
        );
        let enum_schema = serde_json::Value::Object(
            std::iter::once((
                constants_str::ENUM.to_owned(),
                serde_json::Value::Array(vec![nested_value(0u8)]),
            ))
            .collect(),
        );
        let valid_constant = crate::validate_openapi_json_payload::validate_openapi_json_payload(
            &payload,
            &constant_schema,
            &serde_json::Value::Null,
        );
        let invalid_constant = crate::validate_openapi_json_payload::validate_openapi_json_payload(
            &payload,
            &mismatch_schema,
            &serde_json::Value::Null,
        );
        let valid_enum = crate::validate_openapi_json_payload::validate_openapi_json_payload(
            &payload,
            &enum_schema,
            &serde_json::Value::Null,
        );
        test_drop_nested_json_values([payload, constant_schema, mismatch_schema, enum_schema]);
        assert!(matches!(valid_constant, Ok(())));
        assert!(matches!(
            invalid_constant,
            Err(
                crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
                    crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Const
                )
            )
        ));
        assert!(matches!(valid_enum, Ok(())));
    }

    #[test]
    fn test_iterative_json_comparison_preserves_structural_equality() {
        let object = serde_json::json!({ constants_str::NAME: [0u8, 1u8] });
        let cases = [
            (object.clone(), object.clone()),
            (
                object.clone(),
                serde_json::json!({ constants_str::NAME: [0u8, 2u8] }),
            ),
            (
                object.clone(),
                serde_json::json!({ constants_str::ITEMS: [0u8, 1u8] }),
            ),
            (object, serde_json::json!({})),
            (serde_json::json!([0u8]), serde_json::json!([0u8, 1u8])),
            (serde_json::json!([0u8, 1u8]), serde_json::json!([1u8, 0u8])),
            (serde_json::json!(0u8), serde_json::json!(false)),
        ];
        assert!(cases.into_iter().all(|(payload, candidate)| {
            let equal = payload == candidate;
            [constants_str::CONST, constants_str::ENUM]
                .into_iter()
                .all(|keyword| {
                    let value = if keyword == constants_str::ENUM {
                        serde_json::Value::Array(vec![candidate.clone()])
                    } else {
                        candidate.clone()
                    };
                    let schema = serde_json::Value::Object(
                        std::iter::once((keyword.to_owned(), value)).collect(),
                    );
                    crate::validate_openapi_json_payload::validate_openapi_json_payload(
                        &payload,
                        &schema,
                        &serde_json::Value::Null,
                    )
                    .is_ok()
                        == equal
                })
        }));
    }

    #[test]
    fn test_schema_reference_cycles_return_typed_mismatches() {
        let reference = serde_json::json!({
            constants_str::DOLLAR_REF: constants_str::TEST_OPENAPI_SCHEMA_REF
        });
        assert!([reference.clone(), serde_json::json!({ constants_str::ALL_OF: [reference] })]
            .into_iter()
            .all(|schema| {
                let document = serde_json::json!({
                    constants_str::COMPONENTS: { constants_str::SCHEMAS: {
                        constants_str::TEST_OPENAPI_SCHEMA: schema
                    }}
                });
                matches!(
                    crate::validate_openapi_json_payload::validate_openapi_json_payload(
                        &serde_json::Value::from(0u8), &reference, &document,
                    ),
                    Err(crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
                        crate::open_api_schema_mismatch::OpenApiSchemaMismatch::ReferenceCycle
                    ))
                )
            }));
    }

    #[test]
    fn test_long_schema_reference_and_composition_chains_do_not_recurse() {
        let name = |bounded_len: bounded_types::bounded_len::BoundedLen| {
            let mut value = constants_str::TEST_OPENAPI_SCHEMA.to_owned();
            value.push_str(bounded_len.get().to_string().as_str());
            crate::open_api_contract_text::OpenApiContractText::try_from(value)
                .expect(constants_str::DIAGNOSTIC_E10EC642)
        };
        let reference = |bounded_len: bounded_types::bounded_len::BoundedLen| {
            let mut value = constants_str::COMPONENTS_SCHEMAS.to_owned();
            value.push_str(name(bounded_len).as_ref());
            serde_json::json!({ constants_str::DOLLAR_REF: value })
        };
        assert!(
            [
                None,
                Some(constants_str::ALL_OF),
                Some(constants_str::ONE_OF),
                Some(constants_str::ANY_OF),
            ]
            .into_iter()
            .all(|composition| {
                let schemas = (0usize..4_096usize)
                    .map(|index| {
                        let schema = if index == 4_095usize {
                            serde_json::json!({ constants_str::JSON_TYPE: constants_str::INTEGER })
                        } else {
                            let next = reference(bounded_types::bounded_len::BoundedLen::from(
                                index.saturating_add(constants_usize::ONE),
                            ));
                            match composition {
                                Some(key) => serde_json::json!({ key: [next] }),
                                None => next,
                            }
                        };
                        (
                            name(bounded_types::bounded_len::BoundedLen::from(index))
                                .as_ref()
                                .to_owned(),
                            schema,
                        )
                    })
                    .collect::<serde_json::Map<String, serde_json::Value>>();
                let document = serde_json::json!({
                    constants_str::COMPONENTS: { constants_str::SCHEMAS: schemas }
                });
                matches!(
                    crate::validate_openapi_json_payload::validate_openapi_json_payload(
                        &serde_json::Value::from(0u8),
                        &reference(bounded_types::bounded_len::BoundedLen::from(0usize)),
                        &document,
                    ),
                    Ok(())
                )
            })
        );
    }

    #[test]
    fn test_recursive_schema_validates_children_and_independent_branches() {
        let reference = serde_json::json!({
            constants_str::DOLLAR_REF: constants_str::TEST_OPENAPI_SCHEMA_REF
        });
        let schema = serde_json::json!({
            constants_str::JSON_TYPE: constants_str::OBJECT,
            constants_str::PROPERTIES: {
                constants_str::ITEMS: {
                    constants_str::JSON_TYPE: constants_str::ARRAY,
                    constants_str::ITEMS: reference
                }
            }
        });
        let document = serde_json::json!({
            constants_str::COMPONENTS: { constants_str::SCHEMAS: {
                constants_str::TEST_OPENAPI_SCHEMA: schema
            }}
        });
        let payload = serde_json::json!({ constants_str::ITEMS: [
            {}, { constants_str::ITEMS: [{}] }
        ] });
        let repeated_branches = serde_json::json!({ constants_str::ALL_OF: [
            reference, reference
        ] });
        assert!(matches!(
            crate::validate_openapi_json_payload::validate_openapi_json_payload(
                &payload,
                &repeated_branches,
                &document,
            ),
            Ok(())
        ));
        assert!(matches!(
            crate::validate_openapi_json_payload::validate_openapi_json_payload(
                &serde_json::json!({ constants_str::ITEMS: [0u8] }),
                &reference,
                &document,
            ),
            Err(
                crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
                    crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Type
                )
            )
        ));
    }

    #[test]
    fn test_composition_results_preserve_branch_count_and_failure_semantics() {
        let integer_schema =
            serde_json::json!({ constants_str::JSON_TYPE: constants_str::INTEGER });
        let boolean_schema =
            serde_json::json!({ constants_str::JSON_TYPE: constants_str::BOOLEAN });
        let reference = serde_json::json!({ constants_str::DOLLAR_REF: constants_str::TEST_OPENAPI_SCHEMA_REF });
        let document = serde_json::json!({ constants_str::COMPONENTS: { constants_str::SCHEMAS: {
            constants_str::TEST_OPENAPI_SCHEMA: { constants_str::ALL_OF: [reference] }
        }}});
        assert!([
            (serde_json::json!({ constants_str::ALL_OF: [integer_schema, boolean_schema] }), Some(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Type)),
            (serde_json::json!({ constants_str::ONE_OF: [integer_schema, integer_schema] }), Some(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::OneOf)),
            (serde_json::json!({ constants_str::ONE_OF: [boolean_schema, integer_schema] }), None),
            (serde_json::json!({ constants_str::ANY_OF: [boolean_schema, integer_schema] }), None),
            (serde_json::json!({ constants_str::ANY_OF: [boolean_schema] }), Some(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::AnyOf)),
            (serde_json::json!({ constants_str::ALL_OF: [] }), None),
            (serde_json::json!({ constants_str::ONE_OF: [] }), Some(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::OneOf)),
            (serde_json::json!({ constants_str::ANY_OF: [] }), Some(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::AnyOf)),
            (serde_json::json!({ constants_str::ONE_OF: [reference, integer_schema] }), None),
            (serde_json::json!({ constants_str::ANY_OF: [reference, integer_schema] }), None),
            (serde_json::json!({ constants_str::ALL_OF: [reference, integer_schema] }), Some(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::ReferenceCycle)),
        ].into_iter().all(|(schema, expected)| {
            match (crate::validate_openapi_json_payload::validate_openapi_json_payload(&serde_json::Value::from(0u8), &schema, &document), expected) {
                (Ok(()), None) => true,
                (Err(crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(actual)), Some(expected_mismatch)) => actual == expected_mismatch,
                _ => false,
            }
        }));
    }

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
    fn test_duplicate_normalized_http_method_is_rejected() {
        let document = serde_json::json!({
            constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: {
                constants_str::GET: { constants_str::OPERATION_ID_JSON: constants_str::TEST_OPENAPI_OPERATION_ID },
                constants_str::GET_LOWERCASE: { constants_str::OPERATION_ID_JSON: constants_str::TEST_OPENAPI_OPERATION_ID }
            } },
            constants_str::COMPONENTS: { constants_str::SCHEMAS: {} }
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
            Err(crate::open_api_validation_error::OpenApiValidationError::DuplicateOperation(_, _))
        ));
    }

    #[test]
    fn test_duplicate_runtime_route_is_rejected() {
        let document = serde_json::json!({
            constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: {
                constants_str::GET_LOWERCASE: {
                    constants_str::OPERATION_ID_JSON: constants_str::TEST_OPENAPI_OPERATION_ID
                }
            } },
            constants_str::COMPONENTS: { constants_str::SCHEMAS: {} }
        });
        let route = frontend_contract::route_metadata::RouteMetadata::new(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::TEST_OPENAPI_OPERATION_ID.into(),
            constants_str::TEST_OPENAPI_PATH.into(),
        );
        assert!(matches!(
            crate::validate_openapi_contract::validate_openapi_contract(
                &document,
                [route, route].as_slice().into()
            ),
            Err(crate::open_api_validation_error::OpenApiValidationError::DuplicateOperation(_, _))
        ));
    }

    #[test]
    fn test_non_object_path_item_is_rejected() {
        let document = serde_json::json!({
            constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: serde_json::Value::Null },
            constants_str::COMPONENTS: { constants_str::SCHEMAS: {} }
        });
        assert!(matches!(
            crate::validate_openapi_contract::validate_openapi_contract(
                &document,
                (&[][..]).into()
            ),
            Err(crate::open_api_validation_error::OpenApiValidationError::InvalidPathItem(_))
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
    fn test_nested_component_schema_reference_is_valid() {
        let nested_reference = [
            constants_str::TEST_OPENAPI_SCHEMA_REF,
            constants_str::TEST_OPENAPI_PATH,
        ]
        .concat();
        let document = serde_json::json!({
            constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::DOLLAR_REF: nested_reference } },
            constants_str::COMPONENTS: { constants_str::SCHEMAS: { constants_str::TEST_OPENAPI_SCHEMA: {
                constants_str::ITEMS: { constants_str::JSON_TYPE: constants_str::OBJECT }
            } } }
        });
        assert!(matches!(
            crate::validate_openapi_schema_references::validate_openapi_schema_references(
                &document
            ),
            Ok(())
        ));
    }

    #[test]
    fn test_escaped_component_schema_name_is_valid() {
        let escaped_slash: String = ['~', '1'].into_iter().collect();
        let reference = [
            constants_str::COMPONENTS_SCHEMAS.to_owned(),
            escaped_slash,
            constants_str::ITEMS.to_owned(),
        ]
        .concat();
        let document = serde_json::json!({
            constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: { constants_str::DOLLAR_REF: reference } },
            constants_str::COMPONENTS: { constants_str::SCHEMAS: { constants_str::TEST_OPENAPI_PATH: {
                constants_str::JSON_TYPE: constants_str::OBJECT
            } } }
        });
        assert!(matches!(
            crate::validate_openapi_contract::validate_openapi_contract(
                &document,
                (&[][..]).into()
            ),
            Ok(())
        ));
    }

    #[test]
    fn test_operation_security_status_and_content_type_are_checked() {
        let mut document = serde_json::json!({
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
            crate::open_api_response_status::OpenApiResponseStatus::try_from(200u16)
                .expect(constants_str::DIAGNOSTIC_9F6E9528),
            constants_str::APPLICATION_JSON.into(),
            crate::open_api_security_expectation::OpenApiSecurityExpectation::Public,
        );
        assert_eq!(
            expectation.metadata().path().as_ref(),
            constants_str::TEST_OPENAPI_PATH
        );
        assert_eq!(*expectation.status(), 200u16);
        assert!(matches!(
            crate::validate_openapi_operations::validate_openapi_operations(
                &document,
                &[expectation]
            ),
            Ok(())
        ));
        assert!(document.as_object_mut().is_some_and(|object| {
            object
                .insert(
                    constants_str::SECURITY.to_owned(),
                    serde_json::json!([{ constants_str::NAME: [] }]),
                )
                .is_none()
        }));
        assert!(matches!(
            crate::validate_openapi_operations::validate_openapi_operations(
                &document,
                &[expectation]
            ),
            Err(crate::open_api_operation_validation_error::OpenApiOperationValidationError::SecurityMismatch)
        ));
        let required_expectation =
            crate::open_api_operation_expectation::OpenApiOperationExpectation::new(
                expectation.metadata(),
                expectation.status(),
                expectation.content_type(),
                crate::open_api_security_expectation::OpenApiSecurityExpectation::Required(
                    constants_str::NAME.into(),
                ),
            );
        assert!(matches!(
            crate::validate_openapi_operations::validate_openapi_operations(
                &document,
                &[required_expectation]
            ),
            Ok(())
        ));
        assert!(document.as_object_mut().is_some_and(|object| {
            object
                .insert(
                    constants_str::SECURITY.to_owned(),
                    serde_json::json!([{ constants_str::NAME: [] }, {}]),
                )
                .is_some()
        }));
        assert!(matches!(
            crate::validate_openapi_operations::validate_openapi_operations(
                &document,
                &[required_expectation]
            ),
            Err(crate::open_api_operation_validation_error::OpenApiOperationValidationError::SecurityMismatch)
        ));
        assert!(
            document
                .get_mut(constants_str::PATHS)
                .and_then(|paths| paths.get_mut(constants_str::TEST_OPENAPI_PATH))
                .and_then(|path| path.get_mut(constants_str::GET_LOWERCASE))
                .and_then(serde_json::Value::as_object_mut)
                .is_some_and(|operation| {
                    operation
                        .insert(constants_str::SECURITY.to_owned(), serde_json::json!([]))
                        .is_none()
                })
        );
        assert!(matches!(
            crate::validate_openapi_operations::validate_openapi_operations(
                &document,
                &[expectation]
            ),
            Ok(())
        ));
        assert!(matches!(
            crate::validate_openapi_operations::validate_openapi_operations(
                &document,
                &[required_expectation]
            ),
            Err(crate::open_api_operation_validation_error::OpenApiOperationValidationError::SecurityMismatch)
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
    #[test]
    fn test_schema_reference_validation_requires_object_schema_catalog() {
        [
            serde_json::json!({}),
            serde_json::json!({constants_str::COMPONENTS: {}}),
            serde_json::json!({constants_str::COMPONENTS: {constants_str::SCHEMAS: null}}),
            serde_json::json!({constants_str::COMPONENTS: {constants_str::SCHEMAS: []}}),
        ]
        .into_iter()
        .fold((), |(), document| {
            assert!(matches!(
                crate::validate_openapi_schema_references::validate_openapi_schema_references(
                    &document
                ),
                Err(crate::open_api_validation_error::OpenApiValidationError::MissingSchemas)
            ));
        });
    }

    #[test]
    fn test_schema_reference_validation_reports_document_serialization_error() {
        let document = std::collections::BTreeMap::from([([1u8, 2u8], 3u8)]);
        let expected_diagnostic = serde_json::to_value(&document)
            .err()
            .map(|error| error.to_string());
        assert!(expected_diagnostic.is_some());
        assert!(matches!(
            crate::validate_openapi_schema_references::validate_openapi_schema_references(
                &document
            ),
            Err(crate::open_api_validation_error::OpenApiValidationError::DocumentSerialization(source))
                if Some(source.to_string()) == expected_diagnostic
        ));
    }

    #[test]
    fn test_openapi_method_catalog_preserves_case_matching_and_ignored_path_metadata() {
        let connect = constants_str::CONNECT.to_ascii_uppercase();
        assert!([
            frontend_contract::route_method::RouteMethod::Connect,
            frontend_contract::route_method::RouteMethod::Delete,
            frontend_contract::route_method::RouteMethod::Get,
            frontend_contract::route_method::RouteMethod::Head,
            frontend_contract::route_method::RouteMethod::Options,
            frontend_contract::route_method::RouteMethod::Patch,
            frontend_contract::route_method::RouteMethod::Post,
            frontend_contract::route_method::RouteMethod::Put,
            frontend_contract::route_method::RouteMethod::Trace,
        ].into_iter().all(|route_method| {
            let method = route_method.as_str();
            [
                method.as_ref().to_ascii_lowercase(),
                method.as_ref().to_ascii_uppercase(),
                method.as_ref().chars().enumerate().map(|(index, character)| if index.is_multiple_of(2usize) { character.to_ascii_lowercase() } else { character.to_ascii_uppercase() }).collect(),
            ].into_iter().all(|spelling| {
                let document = serde_json::json!({
                    constants_str::PATHS: { constants_str::TEST_OPENAPI_PATH: {
                        spelling: { constants_str::OPERATION_ID_JSON: constants_str::TEST_OPENAPI_OPERATION_ID },
                        (constants_str::PARAMETERS.to_ascii_lowercase()): null,
                        constants_str::DESCRIPTION: null,
                        constants_str::X: null,
                    } },
                    constants_str::COMPONENTS: { constants_str::SCHEMAS: {} },
                });
                let route = frontend_contract::route_metadata::RouteMetadata::new(
                    route_method,
                    constants_str::TEST_OPENAPI_OPERATION_ID.into(),
                    constants_str::TEST_OPENAPI_PATH.into(),
                );
                let result = crate::validate_openapi_contract::validate_openapi_contract(&document, [route].as_slice().into());
                if route_method == frontend_contract::route_method::RouteMethod::Connect {
                    result.is_err_and(|error| matches!(error,
                        crate::open_api_validation_error::OpenApiValidationError::RuntimeRouteMissing(observed_method, path)
                            if observed_method.as_ref() == connect && path.as_ref() == constants_str::TEST_OPENAPI_PATH
                    ))
                } else {
                    result.is_ok()
                }
            })
        }));
    }

    #[test]
    fn test_schema_reference_validation_ignores_external_and_non_string_references() {
        let document = serde_json::json!({
            constants_str::COMPONENTS: {constants_str::SCHEMAS: {}},
            constants_str::ITEMS: [
                {constants_str::DOLLAR_REF: constants_str::HTTPS_ADMIN_EXAMPLE_COM},
                {constants_str::DOLLAR_REF: 1u8},
                {constants_str::DOLLAR_REF: null},
            ],
        });
        assert!(matches!(
            crate::validate_openapi_schema_references::validate_openapi_schema_references(
                &document
            ),
            Ok(())
        ));
    }
    #[test]
    fn test_schema_reference_escape_matrix_preserves_single_pass_decoding_and_nested_names() {
        assert!([
            (vec!['~'], vec!['~', '0']),
            (vec!['/'], vec!['~', '1']),
            (vec!['~', '1'], vec!['~', '0', '1']),
            (vec!['\u{00e9}', '~', '/'], vec!['\u{00e9}', '~', '0', '~', '1']),
            (vec!['~', '0', '~', '1', '/'], vec!['~', '0', '0', '~', '0', '1', '~', '1']),
        ].into_iter().all(|(name_characters, escaped_characters)| {
            let name = name_characters.into_iter().collect::<String>();
            let escaped = escaped_characters.into_iter().collect::<String>();
            let reference = [constants_str::COMPONENTS_SCHEMAS, escaped.as_str()].concat();
            let nested_reference = [reference.as_str(), constants_str::SLASH, constants_str::PROPERTIES, constants_str::SLASH, constants_str::X].concat();
            let document = serde_json::json!({
                constants_str::COMPONENTS: { constants_str::SCHEMAS: {
                    (name.as_str()): { constants_str::PROPERTIES: { constants_str::X: {} } }
                } },
                constants_str::ITEMS: [
                    { constants_str::DOLLAR_REF: reference },
                    { constants_str::DOLLAR_REF: nested_reference },
                ],
            });
            crate::openapi_schema_references::openapi_schema_references(&document).is_ok_and(|references| {
                references.len() == 1usize
                    && references.iter().next().is_some_and(|schema_reference| schema_reference.as_ref() == name)
                    && matches!(references.validate(&document), Ok(()))
                    && matches!(crate::validate_openapi_schema_references::validate_openapi_schema_references(&document), Ok(()))
            })
        }));
    }
}
