fn table(v: &'static str) -> super::PgTableNameRef<'static> {
    super::PgTableNameRef::from(v)
}
fn sql(v: &'static str) -> super::PgTableSqlFragmentRef<'static> {
    super::PgTableSqlFragmentRef::from(v)
}
fn users_base() -> (
    super::PgTableNameRef<'static>,
    super::PgTableSqlFragmentRef<'static>,
) {
    (
        table(constants_str::USERS_ALT),
        sql(constants_str::SQL_NAMES_ID),
    )
}
fn assert_q(actual: &str, expected: &'static str) {
    assert_eq!(actual, expected);
}
#[test]
fn generate_cm_query_string_is_expected() {
    assert_q(
        &super::generate_cm_query_string(
            table(constants_str::USERS_ALT),
            sql(constants_str::ID_NAME),
            sql(constants_str::DOLLAR_1_DOLLAR_2_DOLLAR_3_DOLLAR_4),
            sql(constants_str::SQL_NAMES_ID),
        ),
        constants_str::INSERT_INTO_USERS_ID_NAME_VALUES_DOLLAR_1_DOLLAR_2_DOLLAR_3,
    );
}
#[test]
fn generate_co_query_string_is_expected() {
    assert_q(
        &super::generate_co_query_string(
            table(constants_str::USERS_ALT),
            sql(constants_str::ID_NAME),
            sql(constants_str::DOLLAR_1_DOLLAR_2),
            sql(constants_str::SQL_NAMES_ID),
        ),
        constants_str::INSERT_INTO_USERS_ID_NAME_VALUES_DOLLAR_1_DOLLAR_2_RETURNING_ID,
    );
}
#[test]
fn generate_rm_query_string_is_expected() {
    assert_q(
        &super::generate_rm_query_string(
            table(constants_str::USERS_ALT),
            sql(constants_str::ID_NAME),
            sql(constants_str::ORDER_BY_ID),
        ),
        constants_str::SELECT_ID_NAME_FROM_USERS_ORDER_BY_ID,
    );
}
#[test]
fn generate_ro_query_string_is_expected() {
    assert_q(
        &super::generate_ro_query_string(
            table(constants_str::USERS_ALT),
            sql(constants_str::ID_NAME),
            sql(constants_str::ID_DOLLAR_1),
        ),
        constants_str::SELECT_ID_NAME_FROM_USERS_WHERE_ID_DOLLAR_1,
    );
}
#[test]
fn generate_column_eq_v_comma_uo_query_part_is_expected() {
    assert_q(
        &super::generate_column_eq_v_comma_uo_query_part(
            sql(constants_str::NAME),
            sql(constants_str::DOLLAR_2),
        ),
        constants_str::NAME_DOLLAR_2_ALT,
    );
}
#[test]
fn generate_when_column_id_then_v_um_query_part_is_expected() {
    assert_q(
        &super::generate_when_column_id_then_v_um_query_part(
            sql(constants_str::SQL_NAMES_ID),
            sql(constants_str::DOLLAR_1_ALT),
            sql(constants_str::DOLLAR_2),
        ),
        constants_str::WHEN_ID_DOLLAR_1_THEN_DOLLAR_2,
    );
}
#[test]
fn generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part_is_expected() {
    assert_q(
        &super::generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part(
            sql(constants_str::NAME),
            sql(constants_str::WHEN_ID_DOLLAR_1_THEN_DOLLAR_2),
        ),
        constants_str::NAME_CASE_WHEN_ID_DOLLAR_1_THEN_DOLLAR_2_ELSE_NAME_END,
    );
}
#[test]
fn generate_um_query_string_is_expected() {
    assert_q(
        &super::generate_um_query_string(
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
fn generate_uo_query_string_is_expected() {
    assert_q(
        &super::generate_uo_query_string(
            table(constants_str::USERS_ALT),
            sql(constants_str::NAME_DOLLAR_2),
            sql(constants_str::SQL_NAMES_ID),
            sql(constants_str::DOLLAR_1_ALT),
            sql(constants_str::ID_NAME),
        ),
        constants_str::UPDATE_USERS_SET_NAME_DOLLAR_2_WHERE_ID_DOLLAR_1_RETURNING_ID,
    );
}
#[test]
fn optimistic_uo_query_requires_matching_revision() {
    let query = super::add_uo_optimistic_revision_predicate(
        super::generate_uo_query_string(
            table(constants_str::USERS_ALT),
            sql(constants_str::NAME_DOLLAR_1_REVISION_REVISION_PLUS_1),
            sql(constants_str::SQL_NAMES_ID),
            sql(constants_str::DOLLAR_2),
            sql(constants_str::ID_REVISION),
        ),
        sql(constants_str::REVISION),
        sql(constants_str::DOLLAR_3),
    );
    assert_q(
        &query,
        constants_str::UPDATE_USERS_SET_NAME_DOLLAR_1_REVISION_REVISION_PLUS_1_WHERE_ID,
    );
}
#[test]
fn revision_rejects_invalid_and_negative_values() {
    assert!(matches!(
        super::PgTableRevision::try_from("invalid".to_owned()),
        Err(super::PgTableRevisionTryFromStringError::Invalid(_))
    ));
    assert!(matches!(
        super::PgTableRevision::try_from("-1".to_owned()),
        Err(super::PgTableRevisionTryFromStringError::Negative)
    ));
    assert_eq!(
        super::PgTableRevision::try_from("7".to_owned())
            .expect("63520e0f revision_rejects_invalid_and_negative_values invariant must hold")
            .to_string(),
        "7"
    );
}
#[test]
fn generate_dm_query_string_is_expected() {
    assert_q(
        &super::generate_dm_query_string(
            table(constants_str::USERS_ALT),
            sql(constants_str::WHERE_ID_IN_DOLLAR_1_DOLLAR_2),
            sql(constants_str::SQL_NAMES_ID),
        ),
        constants_str::DELETE_FROM_USERS_WHERE_ID_IN_DOLLAR_1_DOLLAR_2_RETURNING_ID,
    );
}
#[test]
fn generate_dlo_query_string_is_expected() {
    let (table, primary_key) = users_base();
    assert_q(
        &super::generate_dlo_query_string(table, primary_key),
        constants_str::DELETE_FROM_USERS_WHERE_ID_DOLLAR_1_RETURNING_ID,
    );
}
#[test]
fn generate_um_query_string_wraps_primary_key_selector_for_in_clause() {
    let v = super::generate_um_query_string(
        table(constants_str::USERS_ALT),
        sql(constants_str::NAME_CASE_END),
        sql(constants_str::SQL_NAMES_ID),
        sql(constants_str::DOLLAR_1_DOLLAR_2),
        sql(constants_str::ID_NAME),
    );
    assert!(v.contains("where id in ($1,$2)"));
}
#[test]
fn generate_delete_query_string_uses_provided_filter_without_rewrite() {
    let (table, primary_key) = users_base();
    assert_q(
        &super::generate_delete_query_string(
            table,
            primary_key,
            Some(sql(
                constants_str::WHERE_ID_IN_DOLLAR_1_DOLLAR_2_AND_ACTIVE_TRUE,
            )),
        ),
        constants_str::DELETE_FROM_USERS_WHERE_ID_IN_DOLLAR_1_DOLLAR_2_AND_ACTIVE,
    );
}
#[test]
fn generate_update_query_string_eq_keeps_selector_without_extra_wrapping() {
    assert_q(
        &super::generate_update_query_string(
            table(constants_str::USERS_ALT),
            sql(constants_str::NAME_DOLLAR_2),
            sql(constants_str::SQL_NAMES_ID),
            sql(constants_str::DOLLAR_1_ALT),
            sql(constants_str::ID_NAME),
            super::UpdateSelectorFmt::Eq,
        ),
        constants_str::UPDATE_USERS_SET_NAME_DOLLAR_2_WHERE_ID_DOLLAR_1_RETURNING_ID,
    );
}
#[test]
fn generate_update_query_string_in_list_wraps_selector_once() {
    assert_q(
        &super::generate_update_query_string(
            table(constants_str::USERS_ALT),
            sql(constants_str::NAME_CASE_END),
            sql(constants_str::SQL_NAMES_ID),
            sql(constants_str::DOLLAR_1_DOLLAR_2),
            sql(constants_str::ID_NAME),
            super::UpdateSelectorFmt::InList,
        ),
        constants_str::UPDATE_USERS_SET_NAME_CASE_END_WHERE_ID_IN_DOLLAR_1_DOLLAR,
    );
}
#[test]
fn idempotency_numeric_values_enforce_protocol_and_cleanup_ranges() {
    let _status_error = super::PgTableIdempotencyResponseStatus::try_from(99u16)
        .expect_err(constants_str::VALUE_454794DA);
    let _retention_error =
        super::PgTableIdempotencyCleanupRetentionSeconds::try_from(-constants_i64::ONE)
            .expect_err(constants_str::VALUE_81BC8531);
    let _batch_error = super::PgTableIdempotencyCleanupBatchSize::try_from(constants_i64::ZERO)
        .expect_err(constants_str::VALUE_DDCFA298);
}
