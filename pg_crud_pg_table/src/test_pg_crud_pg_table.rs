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
