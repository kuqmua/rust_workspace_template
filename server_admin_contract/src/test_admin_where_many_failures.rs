#[test]
fn test_admin_filter_rejections_preserve_operation_specific_errors() {
    let cases = [
        (
            frontend_contract::input_kind::InputKind::Text,
            frontend_contract::filter_operation::FilterOperation::Eq,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::Text,
            frontend_contract::filter_operation::FilterOperation::Eq,
            Some(constants_str::X),
            Some(constants_str::X),
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd,
        ),
        (
            frontend_contract::input_kind::InputKind::Number,
            frontend_contract::filter_operation::FilterOperation::Eq,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::Number,
            frontend_contract::filter_operation::FilterOperation::Eq,
            Some(constants_str::X),
            Some(constants_str::X),
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd,
        ),
        (
            frontend_contract::input_kind::InputKind::Checkbox,
            frontend_contract::filter_operation::FilterOperation::Eq,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::Checkbox,
            frontend_contract::filter_operation::FilterOperation::Eq,
            Some(constants_str::X),
            Some(constants_str::X),
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd,
        ),
        (
            frontend_contract::input_kind::InputKind::Uuid,
            frontend_contract::filter_operation::FilterOperation::Eq,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::Uuid,
            frontend_contract::filter_operation::FilterOperation::Eq,
            Some(constants_str::X),
            Some(constants_str::X),
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd,
        ),
        (
            frontend_contract::input_kind::InputKind::DateTime,
            frontend_contract::filter_operation::FilterOperation::Eq,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::DateTime,
            frontend_contract::filter_operation::FilterOperation::Eq,
            Some(constants_str::X),
            Some(constants_str::X),
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd,
        ),
        (
            frontend_contract::input_kind::InputKind::Date,
            frontend_contract::filter_operation::FilterOperation::Eq,
            Some(constants_str::X),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnsupportedField,
        ),
        (
            frontend_contract::input_kind::InputKind::Time,
            frontend_contract::filter_operation::FilterOperation::Eq,
            Some(constants_str::X),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnsupportedField,
        ),
        (
            frontend_contract::input_kind::InputKind::Text,
            frontend_contract::filter_operation::FilterOperation::Regex,
            Some(constants_str::X),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnsupportedOperation,
        ),
        (
            frontend_contract::input_kind::InputKind::Number,
            frontend_contract::filter_operation::FilterOperation::Regex,
            Some(constants_str::X),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnsupportedOperation,
        ),
        (
            frontend_contract::input_kind::InputKind::Checkbox,
            frontend_contract::filter_operation::FilterOperation::Regex,
            Some(constants_str::X),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnsupportedOperation,
        ),
        (
            frontend_contract::input_kind::InputKind::Uuid,
            frontend_contract::filter_operation::FilterOperation::Regex,
            Some(constants_str::X),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnsupportedOperation,
        ),
        (
            frontend_contract::input_kind::InputKind::DateTime,
            frontend_contract::filter_operation::FilterOperation::Regex,
            Some(constants_str::X),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnsupportedOperation,
        ),
        (
            frontend_contract::input_kind::InputKind::Number,
            frontend_contract::filter_operation::FilterOperation::Eq,
            Some(constants_str::X),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidValue,
        ),
        (
            frontend_contract::input_kind::InputKind::Number,
            frontend_contract::filter_operation::FilterOperation::In,
            Some(constants_str::X),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidValue,
        ),
        (
            frontend_contract::input_kind::InputKind::Checkbox,
            frontend_contract::filter_operation::FilterOperation::Eq,
            Some(constants_str::X),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidValue,
        ),
        (
            frontend_contract::input_kind::InputKind::Checkbox,
            frontend_contract::filter_operation::FilterOperation::In,
            Some(constants_str::X),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidValue,
        ),
        (
            frontend_contract::input_kind::InputKind::Uuid,
            frontend_contract::filter_operation::FilterOperation::Eq,
            Some(constants_str::X),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidValue,
        ),
        (
            frontend_contract::input_kind::InputKind::Uuid,
            frontend_contract::filter_operation::FilterOperation::In,
            Some(constants_str::X),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidValue,
        ),
        (
            frontend_contract::input_kind::InputKind::Number,
            frontend_contract::filter_operation::FilterOperation::Between,
            Some(stringify!(1)),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::Number,
            frontend_contract::filter_operation::FilterOperation::Between,
            Some(stringify!(1)),
            Some(constants_str::X),
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidEnd,
        ),
        (
            frontend_contract::input_kind::InputKind::Number,
            frontend_contract::filter_operation::FilterOperation::In,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::Number,
            frontend_contract::filter_operation::FilterOperation::In,
            Some(constants_str::X),
            Some(constants_str::X),
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd,
        ),
        (
            frontend_contract::input_kind::InputKind::Text,
            frontend_contract::filter_operation::FilterOperation::In,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::Text,
            frontend_contract::filter_operation::FilterOperation::In,
            Some(constants_str::X),
            Some(constants_str::X),
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd,
        ),
        (
            frontend_contract::input_kind::InputKind::Checkbox,
            frontend_contract::filter_operation::FilterOperation::In,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::Checkbox,
            frontend_contract::filter_operation::FilterOperation::In,
            Some(constants_str::X),
            Some(constants_str::X),
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd,
        ),
        (
            frontend_contract::input_kind::InputKind::Uuid,
            frontend_contract::filter_operation::FilterOperation::In,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::Uuid,
            frontend_contract::filter_operation::FilterOperation::In,
            Some(constants_str::X),
            Some(constants_str::X),
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd,
        ),
        (
            frontend_contract::input_kind::InputKind::DateTime,
            frontend_contract::filter_operation::FilterOperation::In,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::DateTime,
            frontend_contract::filter_operation::FilterOperation::In,
            Some(constants_str::X),
            Some(constants_str::X),
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::UnexpectedEnd,
        ),
        (
            frontend_contract::input_kind::InputKind::Number,
            frontend_contract::filter_operation::FilterOperation::GreaterThan,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::Number,
            frontend_contract::filter_operation::FilterOperation::Between,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::DateTime,
            frontend_contract::filter_operation::FilterOperation::GreaterThan,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::DateTime,
            frontend_contract::filter_operation::FilterOperation::Between,
            None,
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::DateTime,
            frontend_contract::filter_operation::FilterOperation::Between,
            Some(constants_str::VALUE_2026_07_13T12_30_00),
            None,
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete,
        ),
        (
            frontend_contract::input_kind::InputKind::Number,
            frontend_contract::filter_operation::FilterOperation::Between,
            Some(constants_str::X),
            Some(stringify!(1)),
            crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidValue,
        ),
    ];
    assert!(cases.into_iter().all(
        |(input_kind, operation, raw_value, raw_end, expected_error)| {
            let query = (|| {
                let field = crate::admin_filter_field::AdminFilterField::try_from(
                    constants_str::SQL_NAMES_ID.to_owned(),
                )
                .ok()?;
                let value = raw_value
                    .map(str::to_owned)
                    .map(crate::admin_filter_value::AdminFilterValue::try_from)
                    .transpose()
                    .ok()?;
                let end = raw_end
                    .map(str::to_owned)
                    .map(crate::admin_filter_value::AdminFilterValue::try_from)
                    .transpose()
                    .ok()?;
                Some(
                    crate::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
                        Some(field),
                        Some(operation),
                        value,
                        end,
                    ),
                )
            })();
            query.is_some_and(|admin_data_table_filter_query| {
                crate::admin_where_many::AdminWhereMany::try_from_filter(
                    &admin_data_table_filter_query,
                    input_kind,
                )
                .is_err_and(|observed_error| {
                    std::mem::discriminant(&observed_error)
                        == std::mem::discriminant(&expected_error)
                })
            })
        }
    ));
}

#[test]
fn test_admin_partial_filter_queries_reject_every_missing_header_combination() {
    let field_result = crate::admin_filter_field::AdminFilterField::try_from(
        constants_str::SQL_NAMES_ID.to_owned(),
    );
    assert!(field_result.is_ok());
    let Ok(field) = field_result else {
        return;
    };
    let value_result =
        crate::admin_filter_value::AdminFilterValue::try_from(stringify!(1).to_owned());
    assert!(value_result.is_ok());
    let Ok(value) = value_result else {
        return;
    };
    assert!((0usize..16usize).filter(|mask| mask & 3usize != 3usize).all(|mask| {
        let query = crate::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            (mask & 1usize != 0usize).then(|| field.clone()),
            (mask & 2usize != 0usize).then_some(frontend_contract::filter_operation::FilterOperation::Eq),
            (mask & 4usize != 0usize).then(|| value.clone()),
            (mask & 8usize != 0usize).then(|| value.clone()),
        );
        let result = crate::admin_where_many::AdminWhereMany::try_from_identifier_filter(&query);
        if mask == 0usize {
            result.is_ok_and(|filter| filter.is_none())
        } else {
            matches!(result, Err(crate::admin_identifier_filter_error::AdminIdentifierFilterError::Incomplete))
        }
    }));
}

#[test]
fn test_admin_timestamp_filters_preserve_parser_errors_for_values_and_ends() {
    let expected =
        frontend_contract::parse_timestamp_filter_wire_json::parse_timestamp_filter_wire_json(
            frontend_contract::form_value_ref::FormValueRef::from(constants_str::X),
            frontend_contract::value_format::ValueFormat::TimestampTz,
        )
        .err();
    assert!(expected.is_some());
    let cases = [
        (
            frontend_contract::filter_operation::FilterOperation::Eq,
            constants_str::X.to_owned(),
            None,
            false,
        ),
        (
            frontend_contract::filter_operation::FilterOperation::GreaterThan,
            constants_str::X.to_owned(),
            None,
            false,
        ),
        (
            frontend_contract::filter_operation::FilterOperation::Between,
            constants_str::X.to_owned(),
            Some(constants_str::X),
            false,
        ),
        (
            frontend_contract::filter_operation::FilterOperation::Between,
            constants_str::VALUE_2026_07_13T12_30_00.to_owned(),
            Some(constants_str::X),
            true,
        ),
        (
            frontend_contract::filter_operation::FilterOperation::In,
            [constants_str::VALUE_2026_07_13T12_30_00, constants_str::X]
                .join(constants_str::TEXT_ALT_7),
            None,
            false,
        ),
    ];
    assert!(cases.into_iter().all(|(operation, raw_value, raw_end, invalid_end)| {
        let query = (|| {
            let field = crate::admin_filter_field::AdminFilterField::try_from(constants_str::CREATED_AT.to_owned()).ok()?;
            let value = crate::admin_filter_value::AdminFilterValue::try_from(raw_value).ok()?;
            let end = raw_end.map(str::to_owned).map(crate::admin_filter_value::AdminFilterValue::try_from).transpose().ok()?;
            Some(crate::admin_data_table_filter_query::AdminDataTableFilterQuery::new(Some(field), Some(operation), Some(value), end))
        })();
        query.is_some_and(|admin_data_table_filter_query| {
            crate::admin_where_many::AdminWhereMany::try_from_filter(&admin_data_table_filter_query, frontend_contract::input_kind::InputKind::DateTime)
                .is_err_and(|error| {
                    if invalid_end {
                        matches!(error, crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidTimestampEnd(source) if Some(&source) == expected.as_ref())
                    } else {
                        matches!(error, crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidTimestampValue(source) if Some(&source) == expected.as_ref())
                    }
                })
        })
    }));
}

#[test]
fn test_admin_uuid_membership_preserves_trimmed_duplicates_and_rejects_invalid_entries() {
    let build_query = |raw_value| {
        let field = crate::admin_filter_field::AdminFilterField::try_from(
            constants_str::SQL_NAMES_ID.to_owned(),
        )
        .ok()?;
        let value = crate::admin_filter_value::AdminFilterValue::try_from(raw_value).ok()?;
        Some(
            crate::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
                Some(field),
                Some(frontend_contract::filter_operation::FilterOperation::In),
                Some(value),
                None,
            ),
        )
    };
    let padded = [
        constants_str::SPACE,
        constants_str::TEST_ACCESS_SESSION_ID,
        constants_str::SPACE,
    ]
    .concat();
    let valid = build_query(
        [
            padded.as_str(),
            constants_str::TEST_ACCESS_SESSION_ID,
            padded.as_str(),
        ]
        .join(constants_str::TEXT_ALT_7),
    );
    assert!(valid.is_some_and(|query| {
        crate::admin_where_many::AdminWhereMany::try_from_filter(&query, frontend_contract::input_kind::InputKind::Uuid)
            .is_ok_and(|filter| filter.is_some_and(|where_many| {
                serde_json::from_str::<serde_json::Value>(where_many.as_ref()).is_ok_and(|wire| {
                    wire == serde_json::json!({
                        (constants_str::SQL_NAMES_ID): {
                            (constants_str::PG_CRUD_OPERATOR_FIELD): constants_str::SERVER_ADMIN_FILTER_OPERATOR_AND,
                            (constants_str::PG_CRUD_VALUES_FIELD): [{
                                (stringify!(In)): {
                                    (constants_str::PG_CRUD_OPERATOR_FIELD): constants_str::SERVER_ADMIN_FILTER_OPERATOR_AND,
                                    (constants_str::PG_CRUD_VALUES_FIELD): [constants_str::TEST_ACCESS_SESSION_ID, constants_str::TEST_ACCESS_SESSION_ID, constants_str::TEST_ACCESS_SESSION_ID]
                                }
                            }]
                        }
                    })
                })
            }))
    }));
    assert!([0usize, 1usize, 2usize].into_iter().all(|invalid_position| {
        [constants_str::X, constants_str::PG_CRUD_EMPTY_SQL_SUFFIX].into_iter().all(|invalid| {
            let value = (0usize..3usize).map(|position| {
                if position == invalid_position { invalid } else { constants_str::TEST_ACCESS_SESSION_ID }
            }).collect::<Vec<_>>().join(constants_str::TEXT_ALT_7);
            build_query(value).is_some_and(|query| matches!(
                crate::admin_where_many::AdminWhereMany::try_from_filter(&query, frontend_contract::input_kind::InputKind::Uuid),
                Err(crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidValue)
            ))
        })
    }));
}

#[test]
fn test_admin_where_many_byte_limit_preserves_direct_text_and_deserialize_compaction() {
    let object = serde_json::Value::Object(serde_json::Map::new()).to_string();
    let exact = [
        object.as_str(),
        constants_str::SPACE
            .repeat(constants_usize::VALUE_1_048_576 - object.len())
            .as_str(),
    ]
    .concat();
    assert!(
        crate::admin_where_many::AdminWhereMany::try_from(exact.clone()).is_ok_and(|where_many| {
            where_many.as_ref() == exact
                && serde_json::to_value(&where_many).is_ok_and(|wire| wire == serde_json::json!({}))
        })
    );
    let oversized = [exact.as_str(), constants_str::SPACE].concat();
    assert!(crate::admin_where_many::AdminWhereMany::try_from(oversized.clone()).is_err_and(|error| matches!(
        error,
        crate::admin_where_many_try_from_string_error::AdminWhereManyTryFromStringError::Length(
            bounded_types::bounded_string_error::BoundedStringError::AboveMaximum { actual_length, maximum_length }
        ) if actual_length == bounded_types::bounded_len::BoundedLen::from(constants_usize::VALUE_1_048_576 + constants_usize::ONE)
            && maximum_length == bounded_types::bounded_len::BoundedLen::from(constants_usize::VALUE_1_048_576)
    )));
    assert!(
        serde_json::from_str::<crate::admin_where_many::AdminWhereMany>(&oversized)
            .is_ok_and(|where_many| where_many.as_ref() == object)
    );
    let oversized_non_object = [
        serde_json::Value::Null.to_string(),
        constants_str::SPACE.repeat(constants_usize::VALUE_1_048_576),
    ]
    .concat();
    assert!(matches!(crate::admin_where_many::AdminWhereMany::try_from(oversized_non_object), Err(crate::admin_where_many_try_from_string_error::AdminWhereManyTryFromStringError::NotObject)));
    let malformed =
        constants_str::X.repeat(constants_usize::VALUE_1_048_576 + constants_usize::ONE);
    assert!(matches!(
        crate::admin_where_many::AdminWhereMany::try_from(malformed),
        Err(
            crate::admin_where_many_try_from_string_error::AdminWhereManyTryFromStringError::Json(
                _
            )
        )
    ));
}

#[test]
fn test_admin_where_many_schema_and_malformed_deserialization() {
    let schema = <crate::admin_where_many::AdminWhereMany as utoipa::PartialSchema>::schema();
    assert!(
        serde_json::to_value(schema)
            .is_ok_and(|wire| wire == serde_json::json!({(stringify!(type)): stringify!(object)}))
    );
    let expected = serde_json::from_str::<serde_json::Value>(constants_str::X).err();
    let observed =
        serde_json::from_str::<crate::admin_where_many::AdminWhereMany>(constants_str::X).err();
    assert!(
        expected
            .zip(observed)
            .is_some_and(|(expected_error, observed_error)| {
                expected_error.classify() == observed_error.classify()
                    && expected_error.line() == observed_error.line()
                    && expected_error.column() == observed_error.column()
            })
    );
}

#[test]
fn test_admin_membership_preserves_typed_values_and_rejects_invalid_entries() {
    assert!(constants_str::VALUE_2026_07_13T12_30_00.split_once('T').is_some_and(|(date, _time)| {
        let expected_timestamps = [0u32, 30u32, 0u32].map(|second| serde_json::json!({
            (constants_str::DATE_NAIVE): date,
            (constants_str::PG_CRUD_PG_TIME): {
                (constants_str::HOUR): 12u32,
                (constants_str::MIN): 30u32,
                (constants_str::SEC): second,
                (constants_str::MICRO): 0u32,
            }
        }));
        [
        (
            frontend_contract::input_kind::InputKind::Number,
            constants_str::SQL_NAMES_ID,
            [constants_str::VALUE_1, constants_str::VALUE_2, constants_str::VALUE_1],
            serde_json::json!([1i64, 2i64, 1i64]),
        ),
        (
            frontend_contract::input_kind::InputKind::Checkbox,
            constants_str::IS_BANNED,
            [constants_str::TRUE, constants_str::FALSE, constants_str::TRUE],
            serde_json::json!([true, false, true]),
        ),
        (
            frontend_contract::input_kind::InputKind::DateTime,
            constants_str::CREATED_AT,
            [constants_str::VALUE_2026_07_13T12_30_00, constants_str::VALUE_2026_07_13T12_30_30, constants_str::VALUE_2026_07_13T12_30_00],
            serde_json::json!(expected_timestamps),
        ),
    ].into_iter().all(|(input_kind, field_name, raw_values, expected_values)| {
        crate::admin_filter_field::AdminFilterField::try_from(field_name.to_owned())
            .is_ok_and(|admin_filter_field| {
                let build_query = |admin_filter_value: crate::admin_filter_value::AdminFilterValue| {
                    crate::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
                        Some(admin_filter_field.clone()),
                        Some(frontend_contract::filter_operation::FilterOperation::In),
                        Some(admin_filter_value),
                        None,
                    )
                };
                let padded = raw_values.into_iter().map(|raw_value| {
                    [constants_str::SPACE, raw_value, constants_str::SPACE].concat()
                }).collect::<Vec<_>>().join(constants_str::TEXT_ALT_7);
                let valid = crate::admin_filter_value::AdminFilterValue::try_from(padded)
                    .is_ok_and(|admin_filter_value| {
                        crate::admin_where_many::AdminWhereMany::try_from_filter(
                            &build_query(admin_filter_value), input_kind,
                        ).is_ok_and(|filter| filter.is_some_and(|admin_where_many| {
                            serde_json::from_str::<serde_json::Value>(admin_where_many.as_ref())
                                .is_ok_and(|wire| {
                                    wire.get(field_name)
                                        .and_then(|field| field.get(constants_str::PG_CRUD_VALUES_FIELD))
                                        .and_then(|predicates| predicates.get(0usize))
                                        .and_then(|predicate| predicate.get(stringify!(In)))
                                        .and_then(|body| body.get(constants_str::PG_CRUD_VALUES_FIELD))
                                        == Some(&expected_values)
                                })
                        }))
                    });
                valid && (0usize..3usize).all(|invalid_position| {
                    [constants_str::X, constants_str::EMPTY, constants_str::SPACE]
                        .into_iter().all(|invalid| {
                            let raw_value = raw_values.into_iter().enumerate()
                                .map(|(position, raw_value)| {
                                    if position == invalid_position { invalid } else { raw_value }
                                }).collect::<Vec<_>>().join(constants_str::TEXT_ALT_7);
                            crate::admin_filter_value::AdminFilterValue::try_from(raw_value)
                                .is_ok_and(|admin_filter_value| {
                                    crate::admin_where_many::AdminWhereMany::try_from_filter(
                                        &build_query(admin_filter_value), input_kind,
                                    ).is_err_and(|admin_identifier_filter_error| {
                                        if input_kind == frontend_contract::input_kind::InputKind::DateTime {
                                            frontend_contract::parse_timestamp_filter_wire_json::parse_timestamp_filter_wire_json(
                                                frontend_contract::form_value_ref::FormValueRef::from(invalid.trim()),
                                                frontend_contract::value_format::ValueFormat::TimestampTz,
                                            ).is_err_and(|expected_source| matches!(
                                                admin_identifier_filter_error,
                                                crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidTimestampValue(source)
                                                    if source == expected_source
                                            ))
                                        } else {
                                            matches!(admin_identifier_filter_error,
                                                crate::admin_identifier_filter_error::AdminIdentifierFilterError::InvalidValue)
                                        }
                                    })
                                })
                        })
                })
            })
    })
    }));
}
