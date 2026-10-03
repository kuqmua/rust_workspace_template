#[cfg(test)]
mod tests {
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
        assert!(matches!(
            crate::validate_openapi_schema_references::validate_openapi_schema_references(
                &document
            ),
            Err(crate::open_api_validation_error::OpenApiValidationError::DocumentSerialization(_))
        ));
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
}
