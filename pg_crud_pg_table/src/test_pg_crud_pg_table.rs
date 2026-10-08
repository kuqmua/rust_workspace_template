fn table(str: &'static str) -> crate::pg_table_name_ref::PgTableNameRef<'static> {
    crate::pg_table_name_ref::PgTableNameRef::from(str)
}
fn sql(str: &'static str) -> crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'static> {
    crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef::from(str)
}

fn assert_fragment_result(
    actual: Result<
        crate::pg_table_query_part_fragment::PgTableQueryPartFragment,
        crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError,
    >,
    expected: &'static str,
) {
    assert!(matches!(actual, Ok(fragment) if fragment.to_string() == expected));
}
fn assert_query_result(
    actual: Result<
        crate::pg_table_query_string::PgTableQueryString,
        crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError,
    >,
    expected: &'static str,
) {
    assert!(matches!(actual, Ok(query) if query.to_string() == expected));
}
#[test]
fn test_generate_create_many_query_string_is_expected() {
    assert_query_result(
        crate::generate_create_many_query_string::generate_create_many_query_string(
            table(constants_str::USERS_ALT),
            sql(constants_str::ID_NAME),
            sql(constants_str::DOLLAR_1_DOLLAR_2_DOLLAR_3_DOLLAR_4),
            sql(constants_str::SQL_NAMES_ID),
        ),
        constants_str::INSERT_INTO_USERS_ID_NAME_VALUES_DOLLAR_1_DOLLAR_2_DOLLAR_3,
    );
}
#[test]
fn test_generate_read_many_query_string_is_expected() {
    assert_query_result(
        crate::generate_read_many_query_string::generate_read_many_query_string(
            table(constants_str::USERS_ALT),
            sql(constants_str::ID_NAME),
            sql(constants_str::ORDER_BY_ID),
        ),
        constants_str::SELECT_ID_NAME_FROM_USERS_ORDER_BY_ID,
    );
}
#[test]
fn test_generate_when_column_id_then_value_update_many_query_part_is_expected() {
    assert_fragment_result(
        crate::generate_when_column_id_then_value_update_many_query_part::generate_when_column_id_then_value_update_many_query_part(
            sql(constants_str::SQL_NAMES_ID),
            sql(constants_str::DOLLAR_1_ALT),
            sql(constants_str::DOLLAR_2),
        ),
        constants_str::WHEN_ID_DOLLAR_1_THEN_DOLLAR_2,
    );
}
#[test]
fn test_generate_column_equals_case_accumulator_else_column_end_comma_update_many_query_part_is_expected()
 {
    assert_fragment_result(
        crate::generate_column_equals_case_accumulator_else_column_end_comma_update_many_query_part::generate_column_equals_case_accumulator_else_column_end_comma_update_many_query_part(
            sql(constants_str::NAME),
            sql(constants_str::WHEN_ID_DOLLAR_1_THEN_DOLLAR_2),
        ),
        constants_str::NAME_CASE_WHEN_ID_DOLLAR_1_THEN_DOLLAR_2_ELSE_NAME_END,
    );
}
#[test]
fn test_generate_update_many_query_string_is_expected() {
    assert_query_result(
        crate::generate_update_many_query_string::generate_update_many_query_string(
            table(constants_str::USERS_ALT),
            sql(constants_str::NAME_CASE_END),
            sql(constants_str::SQL_NAMES_ID),
            sql(constants_str::DOLLAR_1_DOLLAR_2),
            sql(constants_str::ID_NAME),
        ),
        constants_str::UPDATE_USERS_SET_NAME_CASE_END_WHERE_ID_IN_DOLLAR_1_DOLLAR,
    );
}
#[test]
fn test_optimistic_bulk_update_query_requires_matching_revision() {
    let base_query = crate::generate_update_many_query_string::generate_update_many_query_string(
        table(constants_str::USERS_ALT),
        sql(constants_str::NAME_DOLLAR_1_REVISION_REVISION_PLUS_1),
        sql(constants_str::SQL_NAMES_ID),
        sql(constants_str::DOLLAR_2),
        sql(constants_str::ID_REVISION),
    );
    let query = base_query.and_then(|validated_query| {
        crate::add_update_optimistic_revision_predicate::add_update_optimistic_revision_predicate(
            validated_query,
            sql(constants_str::REVISION),
            sql(constants_str::DOLLAR_3),
        )
    });
    assert_query_result(
        query,
        constants_str::UPDATE_USERS_SET_NAME_DOLLAR_1_REVISION_REVISION_PLUS_1_WHERE_ID,
    );
}
#[test]
fn test_revision_rejects_invalid_and_negative_values() {
    assert!(matches!(
        crate::pg_table_revision::PgTableRevision::try_from(constants_str::VALUE_F1234D75.to_owned()),
        Err(crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError::Invalid(_))
    ));
    assert!(matches!(
        crate::pg_table_revision::PgTableRevision::try_from(constants_str::VALUE_1BAD6B8C.to_owned()),
        Err(crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError::Negative)
    ));
    assert_eq!(
        crate::pg_table_revision::PgTableRevision::try_from(
            constants_str::VALUE_7902699B.to_owned()
        )
        .expect(constants_str::DIAGNOSTIC_63520E0F)
        .to_string(),
        constants_str::VALUE_7902699B
    );
}
#[test]
fn test_generate_delete_many_query_string_is_expected() {
    assert_query_result(
        crate::generate_delete_many_query_string::generate_delete_many_query_string(
            table(constants_str::USERS_ALT),
            sql(constants_str::WHERE_ID_IN_DOLLAR_1_DOLLAR_2),
            sql(constants_str::SQL_NAMES_ID),
        ),
        constants_str::DELETE_FROM_USERS_WHERE_ID_IN_DOLLAR_1_DOLLAR_2_RETURNING_ID,
    );
}
#[test]
fn test_generate_update_many_query_string_wraps_primary_key_selector_for_in_clause() {
    let v = crate::generate_update_many_query_string::generate_update_many_query_string(
        table(constants_str::USERS_ALT),
        sql(constants_str::NAME_CASE_END),
        sql(constants_str::SQL_NAMES_ID),
        sql(constants_str::DOLLAR_1_DOLLAR_2),
        sql(constants_str::ID_NAME),
    );
    assert!(
        matches!(v, Ok(query) if query.to_string().contains(constants_str::WHERE_ID_IN_DOLLAR_1_DOLLAR_2))
    );
}
#[test]
fn test_generate_delete_many_query_string_preserves_filtered_batch_selector() {
    let table = table(constants_str::USERS_ALT);
    let primary_key = sql(constants_str::SQL_NAMES_ID);
    assert_query_result(
        crate::generate_delete_many_query_string::generate_delete_many_query_string(
            table,
            sql(constants_str::WHERE_ID_IN_DOLLAR_1_DOLLAR_2_AND_ACTIVE_TRUE),
            primary_key,
        ),
        constants_str::DELETE_FROM_USERS_WHERE_ID_IN_DOLLAR_1_DOLLAR_2_AND_ACTIVE,
    );
}
#[test]
fn test_query_string_length_error_preserves_bounded_diagnostic_and_has_no_source() {
    assert!(
        [
            (0usize, 0usize),
            (
                crate::pg_table_string_wrapper_max_len::PG_TABLE_STRING_WRAPPER_MAX_LEN
                    + constants_usize::ONE,
                crate::pg_table_string_wrapper_max_len::PG_TABLE_STRING_WRAPPER_MAX_LEN,
            ),
            (usize::MAX, usize::MAX),
        ]
        .into_iter()
        .all(|(len, max)| {
            let pg_table_string_wrapper_try_from_string_error =
                crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError::TooLong {
                    len,
                    max,
                };
            let error_text = to_err_string::to_err_string::ToErrString::to_err_string(
                &pg_table_string_wrapper_try_from_string_error,
            );
            let display = pg_table_string_wrapper_try_from_string_error.to_string();
            error_text.as_ref() == display
                && display.contains(&len.to_string())
                && display.contains(&max.to_string())
                && std::error::Error::source(&pg_table_string_wrapper_try_from_string_error)
                    .is_none()
        })
    );
}
#[test]
fn test_query_builders_reject_oversized_sql() {
    let huge = constants_str::X
        .repeat(crate::pg_table_string_wrapper_max_len::PG_TABLE_STRING_WRAPPER_MAX_LEN);
    let fragment = crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef::from(huge.as_str());
    let table = table(constants_str::USERS_ALT);
    let small = sql(constants_str::SQL_NAMES_ID);
    let optimistic = crate::generate_update_many_query_string::generate_update_many_query_string(
        table, small, small, small, small,
    )
    .and_then(|query| {
        crate::add_update_optimistic_revision_predicate::add_update_optimistic_revision_predicate(
            query, fragment, small,
        )
    });
    assert!(
        [
            crate::generate_create_many_query_string::generate_create_many_query_string(
                table, fragment, small, small,
            ),
            crate::generate_read_many_query_string::generate_read_many_query_string(table, fragment, small),
            crate::generate_update_many_query_string::generate_update_many_query_string(
                table, fragment, small, small, small,
            ),
            crate::generate_delete_many_query_string::generate_delete_many_query_string(table, fragment, small),
            optimistic,
        ]
        .into_iter()
        .all(|result| matches!(result, Err(crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError::TooLong { len, max }) if len > max))
    );
    assert!(
        [
            crate::generate_when_column_id_then_value_update_many_query_part::generate_when_column_id_then_value_update_many_query_part(
                fragment, small, small,
            ),
            crate::generate_column_equals_case_accumulator_else_column_end_comma_update_many_query_part::generate_column_equals_case_accumulator_else_column_end_comma_update_many_query_part(
                fragment, small,
            ),
        ]
        .into_iter()
        .all(|result| matches!(result, Err(crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError::TooLong { len, max }) if len > max))
    );
}
#[test]
fn test_idempotency_numeric_values_enforce_protocol_and_cleanup_ranges() {
    let _status_error =
        crate::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus::try_from(
            99u16,
        )
        .expect_err(constants_str::VALUE_454794DA);
    let _retention_error =
        crate::pg_table_idempotency_cleanup_retention_seconds::PgTableIdempotencyCleanupRetentionSeconds::try_from(-constants_i64::ONE)
            .expect_err(constants_str::VALUE_81BC8531);
    let _batch_error = crate::pg_table_idempotency_cleanup_batch_size::PgTableIdempotencyCleanupBatchSize::try_from(constants_i64::ZERO)
        .expect_err(constants_str::VALUE_DDCFA298);
}

#[test]
fn test_idempotency_response_status_preserves_full_range_and_known_failure() {
    assert!((100u16..1_000u16).all(|value| {
        crate::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus::try_from(
            value,
        )
        .is_ok_and(|status| u16::from(status) == value)
    }));
    [0u16, 99u16, 1_000u16, u16::MAX].into_iter().fold((), |(), value| {
        assert_eq!(
            crate::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus::try_from(value),
            Err(crate::pg_table_idempotency_response_status_try_from_u16_error::PgTableIdempotencyResponseStatusTryFromU16Error::OutOfRange)
        );
    });
    assert_eq!(u16::from(crate::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus::internal_server_error()), 500u16);
}

#[test]
fn test_idempotency_cleanup_values_preserve_limits_and_distinct_zero_contracts() {
    [i64::MIN, -constants_i64::ONE, constants_i64::ZERO].into_iter().fold((), |(), value| {
        assert_eq!(
            crate::pg_table_idempotency_cleanup_batch_size::PgTableIdempotencyCleanupBatchSize::try_from(value),
            Err(crate::pg_table_idempotency_cleanup_value_try_from_i64_error::PgTableIdempotencyCleanupValueTryFromI64Error::NotPositive)
        );
    });
    [i64::MIN, -constants_i64::ONE].into_iter().fold((), |(), value| {
        assert_eq!(
            crate::pg_table_idempotency_cleanup_retention_seconds::PgTableIdempotencyCleanupRetentionSeconds::try_from(value),
            Err(crate::pg_table_idempotency_cleanup_value_try_from_i64_error::PgTableIdempotencyCleanupValueTryFromI64Error::Negative)
        );
    });
    assert!([constants_i64::ONE, i64::MAX].into_iter().all(|value| {
        crate::pg_table_idempotency_cleanup_batch_size::PgTableIdempotencyCleanupBatchSize::try_from(value)
            .is_ok_and(|batch| batch.get().get() == value)
    }));
    assert!([constants_i64::ZERO, constants_i64::ONE, i64::MAX].into_iter().all(|value| {
        crate::pg_table_idempotency_cleanup_retention_seconds::PgTableIdempotencyCleanupRetentionSeconds::try_from(value)
            .is_ok_and(|retention| retention.get() == value)
    }));
}

#[test]
fn test_optimistic_revision_preserves_queries_without_returning_and_targets_last_clause() {
    assert!([0usize, 1usize, crate::pg_table_string_wrapper_max_len::PG_TABLE_STRING_WRAPPER_MAX_LEN].into_iter().all(|length| {
        let text = constants_str::X.repeat(length);
        crate::pg_table_query_string::PgTableQueryString::try_from(text.clone()).and_then(|query| {
            crate::add_update_optimistic_revision_predicate::add_update_optimistic_revision_predicate(
                query, sql(constants_str::REVISION), sql(constants_str::DOLLAR_3),
            )
        }).is_ok_and(|query| query.to_string() == text)
    }));
    let original = format!(
        "{}{}{}{}{}",
        constants_str::X,
        constants_str::RETURNING,
        constants_str::X,
        constants_str::RETURNING,
        constants_str::X
    );
    let expected = format!(
        "{}{}{}{}{}{}{}{}{}",
        constants_str::X,
        constants_str::RETURNING,
        constants_str::X,
        constants_str::AND,
        constants_str::REVISION,
        constants_str::TEXT_ALT,
        constants_str::DOLLAR_3,
        constants_str::RETURNING,
        constants_str::X
    );
    assert!(crate::pg_table_query_string::PgTableQueryString::try_from(original).and_then(|query| {
        crate::add_update_optimistic_revision_predicate::add_update_optimistic_revision_predicate(
            query, sql(constants_str::REVISION), sql(constants_str::DOLLAR_3),
        )
    }).is_ok_and(|query| query.to_string() == expected));
}

#[test]
fn test_query_and_fragment_wrappers_preserve_exact_byte_limits_and_error_lengths() {
    let maximum = crate::pg_table_string_wrapper_max_len::PG_TABLE_STRING_WRAPPER_MAX_LEN;
    assert!([0usize, maximum - 1usize, maximum, maximum + 1usize].into_iter().all(|length| {
        ['x', '\u{e9}'].into_iter().all(|character| {
            let repetitions = if character.is_ascii() { length } else { length >> 1usize };
            let mut text = character.to_string().repeat(repetitions);
            text.push_str(&constants_str::X.repeat(length - text.len()));
            [
                crate::pg_table_query_string::PgTableQueryString::try_from(text.clone()).map(|query| query.to_string()),
                crate::pg_table_query_part_fragment::PgTableQueryPartFragment::try_from(text.clone()).map(|fragment| fragment.to_string()),
            ].into_iter().all(|result| {
                if length <= maximum {
                    result.is_ok_and(|preserved| preserved == text)
                } else {
                    matches!(result, Err(crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError::TooLong { len, max }) if len == length && max == maximum)
                }
            })
        })
    }));
}

#[test]
fn test_revision_preserves_numeric_boundaries_and_parse_error_kinds() {
    assert!([0i64, 1i64, i64::MAX].into_iter().all(|value| {
        let canonical = value.to_string();
        [
            canonical.clone(),
            format!("+{canonical}"),
            format!("0{canonical}"),
        ]
        .into_iter()
        .all(|text| {
            crate::pg_table_revision::PgTableRevision::try_from(text)
                .is_ok_and(|revision| revision.to_string() == canonical)
        })
    }));
    assert!(
        crate::pg_table_revision::PgTableRevision::try_from(
            ['-', '0'].into_iter().collect::<String>()
        )
        .is_ok_and(|revision| revision.to_string() == constants_str::VALUE_0)
    );
    assert!([i64::MIN, -1i64].into_iter().all(|value| {
        matches!(crate::pg_table_revision::PgTableRevision::try_from(value.to_string()), Err(crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError::Negative))
    }));
    assert!([
        (String::new(), std::num::IntErrorKind::Empty),
        (constants_str::X.to_owned(), std::num::IntErrorKind::InvalidDigit),
        (format!(" {}", constants_str::VALUE_1), std::num::IntErrorKind::InvalidDigit),
        ((i128::from(i64::MAX) + 1i128).to_string(), std::num::IntErrorKind::PosOverflow),
        ((i128::from(i64::MIN) - 1i128).to_string(), std::num::IntErrorKind::NegOverflow),
    ].into_iter().all(|(text, expected)| {
        let result = crate::pg_table_revision::PgTableRevision::try_from(text);
        match result {
            Err(error @ crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError::Invalid(_)) => {
                std::error::Error::source(&error).is_some()
                    && matches!(&error, crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError::Invalid(crate::pg_table_revision_parse_int_error::PgTableRevisionParseIntError::Parse(source)) if source.kind() == &expected)
            }
            Ok(_) | Err(crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError::Negative) => false,
        }
    }));
}
