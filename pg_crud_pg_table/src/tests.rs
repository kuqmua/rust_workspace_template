fn table(v: &'static str) -> crate::pg_table_name_ref::PgTableNameRef<'static> {
    crate::pg_table_name_ref::PgTableNameRef::from(v)
}
fn sql(v: &'static str) -> crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'static> {
    crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef::from(v)
}
fn users_base() -> (
    crate::pg_table_name_ref::PgTableNameRef<'static>,
    crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'static>,
) {
    (
        table(constants_str::catalog::USERS_ALT),
        sql(constants_str::catalog::SQL_NAMES_ID),
    )
}
fn assert_q(actual: &str, expected: &'static str) {
    assert_eq!(actual, expected);
}
#[test]
fn generate_cm_query_string_is_expected() {
    assert_q(
        &crate::generate_cm_query_string::generate_cm_query_string(
            table(constants_str::catalog::USERS_ALT),
            sql(constants_str::catalog::ID_NAME),
            sql(constants_str::catalog::DOLLAR_1_DOLLAR_2_DOLLAR_3_DOLLAR_4),
            sql(constants_str::catalog::SQL_NAMES_ID),
        ),
        constants_str::catalog::INSERT_INTO_USERS_ID_NAME_VALUES_DOLLAR_1_DOLLAR_2_DOLLAR_3,
    );
}
#[test]
fn generate_co_query_string_is_expected() {
    assert_q(
        &crate::generate_co_query_string::generate_co_query_string(
            table(constants_str::catalog::USERS_ALT),
            sql(constants_str::catalog::ID_NAME),
            sql(constants_str::catalog::DOLLAR_1_DOLLAR_2),
            sql(constants_str::catalog::SQL_NAMES_ID),
        ),
        constants_str::catalog::INSERT_INTO_USERS_ID_NAME_VALUES_DOLLAR_1_DOLLAR_2_RETURNING_ID,
    );
}
#[test]
fn generate_rm_query_string_is_expected() {
    assert_q(
        &crate::generate_rm_query_string::generate_rm_query_string(
            table(constants_str::catalog::USERS_ALT),
            sql(constants_str::catalog::ID_NAME),
            sql(constants_str::catalog::ORDER_BY_ID),
        ),
        constants_str::catalog::SELECT_ID_NAME_FROM_USERS_ORDER_BY_ID,
    );
}
#[test]
fn generate_ro_query_string_is_expected() {
    assert_q(
        &crate::generate_ro_query_string::generate_ro_query_string(
            table(constants_str::catalog::USERS_ALT),
            sql(constants_str::catalog::ID_NAME),
            sql(constants_str::catalog::ID_DOLLAR_1),
        ),
        constants_str::catalog::SELECT_ID_NAME_FROM_USERS_WHERE_ID_DOLLAR_1,
    );
}
#[test]
fn generate_column_eq_v_comma_uo_query_part_is_expected() {
    assert_q(
        &crate::generate_column_eq_v_comma_uo_query_part::generate_column_eq_v_comma_uo_query_part(
            sql(constants_str::catalog::NAME),
            sql(constants_str::catalog::DOLLAR_2),
        ),
        constants_str::catalog::NAME_DOLLAR_2_ALT,
    );
}
#[test]
fn generate_when_column_id_then_v_um_query_part_is_expected() {
    assert_q(
        &crate::generate_when_column_id_then_v_um_query_part::generate_when_column_id_then_v_um_query_part(
            sql(constants_str::catalog::SQL_NAMES_ID),
            sql(constants_str::catalog::DOLLAR_1_ALT),
            sql(constants_str::catalog::DOLLAR_2),
        ),
        constants_str::catalog::WHEN_ID_DOLLAR_1_THEN_DOLLAR_2,
    );
}
#[test]
fn generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part_is_expected() {
    assert_q(
        &crate::generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part::generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part(
            sql(constants_str::catalog::NAME),
            sql(constants_str::catalog::WHEN_ID_DOLLAR_1_THEN_DOLLAR_2),
        ),
        constants_str::catalog::NAME_CASE_WHEN_ID_DOLLAR_1_THEN_DOLLAR_2_ELSE_NAME_END,
    );
}
#[test]
fn generate_um_query_string_is_expected() {
    assert_q(
        &crate::generate_um_query_string::generate_um_query_string(
            table(constants_str::catalog::USERS_ALT),
            sql(constants_str::catalog::NAME_CASE_END),
            sql(constants_str::catalog::SQL_NAMES_ID),
            sql(constants_str::catalog::DOLLAR_1_DOLLAR_2),
            sql(constants_str::catalog::ID_NAME),
        ),
        constants_str::catalog::UPDATE_USERS_SET_NAME_CASE_END_WHERE_ID_IN_DOLLAR_1_DOLLAR,
    );
}
#[test]
fn generate_uo_query_string_is_expected() {
    assert_q(
        &crate::generate_uo_query_string::generate_uo_query_string(
            table(constants_str::catalog::USERS_ALT),
            sql(constants_str::catalog::NAME_DOLLAR_2),
            sql(constants_str::catalog::SQL_NAMES_ID),
            sql(constants_str::catalog::DOLLAR_1_ALT),
            sql(constants_str::catalog::ID_NAME),
        ),
        constants_str::catalog::UPDATE_USERS_SET_NAME_DOLLAR_2_WHERE_ID_DOLLAR_1_RETURNING_ID,
    );
}
#[test]
fn optimistic_uo_query_requires_matching_revision() {
    let query = crate::add_uo_optimistic_revision_predicate::add_uo_optimistic_revision_predicate(
        crate::generate_uo_query_string::generate_uo_query_string(
            table(constants_str::catalog::USERS_ALT),
            sql(constants_str::catalog::NAME_DOLLAR_1_REVISION_REVISION_PLUS_1),
            sql(constants_str::catalog::SQL_NAMES_ID),
            sql(constants_str::catalog::DOLLAR_2),
            sql(constants_str::catalog::ID_REVISION),
        ),
        sql(constants_str::catalog::REVISION),
        sql(constants_str::catalog::DOLLAR_3),
    );
    assert_q(
        &query,
        constants_str::catalog::UPDATE_USERS_SET_NAME_DOLLAR_1_REVISION_REVISION_PLUS_1_WHERE_ID,
    );
}
#[test]
fn revision_rejects_invalid_and_negative_values() {
    assert!(matches!(
        crate::pg_table_revision::PgTableRevision::try_from("invalid".to_owned()),
        Err(crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError::Invalid(_))
    ));
    assert!(matches!(
        crate::pg_table_revision::PgTableRevision::try_from("-1".to_owned()),
        Err(crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError::Negative)
    ));
    assert_eq!(
        crate::pg_table_revision::PgTableRevision::try_from("7".to_owned())
            .expect("63520e0f revision_rejects_invalid_and_negative_values invariant must hold")
            .to_string(),
        "7"
    );
}
#[test]
fn generate_dm_query_string_is_expected() {
    assert_q(
        &crate::generate_dm_query_string::generate_dm_query_string(
            table(constants_str::catalog::USERS_ALT),
            sql(constants_str::catalog::WHERE_ID_IN_DOLLAR_1_DOLLAR_2),
            sql(constants_str::catalog::SQL_NAMES_ID),
        ),
        constants_str::catalog::DELETE_FROM_USERS_WHERE_ID_IN_DOLLAR_1_DOLLAR_2_RETURNING_ID,
    );
}
#[test]
fn generate_dlo_query_string_is_expected() {
    let (table, primary_key) = users_base();
    assert_q(
        &crate::generate_dlo_query_string::generate_dlo_query_string(table, primary_key),
        constants_str::catalog::DELETE_FROM_USERS_WHERE_ID_DOLLAR_1_RETURNING_ID,
    );
}
#[test]
fn generate_um_query_string_wraps_primary_key_selector_for_in_clause() {
    let v = crate::generate_um_query_string::generate_um_query_string(
        table(constants_str::catalog::USERS_ALT),
        sql(constants_str::catalog::NAME_CASE_END),
        sql(constants_str::catalog::SQL_NAMES_ID),
        sql(constants_str::catalog::DOLLAR_1_DOLLAR_2),
        sql(constants_str::catalog::ID_NAME),
    );
    assert!(v.contains("where id in ($1,$2)"));
}
#[test]
fn generate_delete_query_string_uses_provided_filter_without_rewrite() {
    let (table, primary_key) = users_base();
    assert_q(
        &crate::generate_delete_query_string::generate_delete_query_string(
            table,
            primary_key,
            Some(sql(
                constants_str::catalog::WHERE_ID_IN_DOLLAR_1_DOLLAR_2_AND_ACTIVE_TRUE,
            )),
        ),
        constants_str::catalog::DELETE_FROM_USERS_WHERE_ID_IN_DOLLAR_1_DOLLAR_2_AND_ACTIVE,
    );
}
#[test]
fn generate_update_query_string_eq_keeps_selector_without_extra_wrapping() {
    assert_q(
        &crate::generate_update_query_string::generate_update_query_string(
            table(constants_str::catalog::USERS_ALT),
            sql(constants_str::catalog::NAME_DOLLAR_2),
            sql(constants_str::catalog::SQL_NAMES_ID),
            sql(constants_str::catalog::DOLLAR_1_ALT),
            sql(constants_str::catalog::ID_NAME),
            crate::update_selector_fmt::UpdateSelectorFmt::Eq,
        ),
        constants_str::catalog::UPDATE_USERS_SET_NAME_DOLLAR_2_WHERE_ID_DOLLAR_1_RETURNING_ID,
    );
}
#[test]
fn generate_update_query_string_in_list_wraps_selector_once() {
    assert_q(
        &crate::generate_update_query_string::generate_update_query_string(
            table(constants_str::catalog::USERS_ALT),
            sql(constants_str::catalog::NAME_CASE_END),
            sql(constants_str::catalog::SQL_NAMES_ID),
            sql(constants_str::catalog::DOLLAR_1_DOLLAR_2),
            sql(constants_str::catalog::ID_NAME),
            crate::update_selector_fmt::UpdateSelectorFmt::InList,
        ),
        constants_str::catalog::UPDATE_USERS_SET_NAME_CASE_END_WHERE_ID_IN_DOLLAR_1_DOLLAR,
    );
}
#[test]
fn idempotency_numeric_values_enforce_protocol_and_cleanup_ranges() {
    let _status_error =
        crate::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus::try_from(
            99u16,
        )
        .expect_err(constants_str::test_fixtures::VALUE_454794DA);
    let _retention_error =
        crate::pg_table_idempotency_cleanup_retention_seconds::PgTableIdempotencyCleanupRetentionSeconds::try_from(-constants_i64::ONE)
            .expect_err(constants_str::test_fixtures::VALUE_81BC8531);
    let _batch_error = crate::pg_table_idempotency_cleanup_batch_size::PgTableIdempotencyCleanupBatchSize::try_from(constants_i64::ZERO)
        .expect_err(constants_str::test_fixtures::VALUE_DDCFA298);
}
