#[test]
fn test_every_generated_table_empty_filter_preserves_placeholder_counter() {
    let text = serde_json::Value::Null.to_string();
    assert!(
        crate::admin_generated_table::AdminGeneratedTable::ALL
            .iter()
            .all(|table| {
                table
                    .parse_filter(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                        text.as_str(),
                    ))
                    .is_ok_and(|filter| {
                        [7u64, u64::MAX].into_iter().all(|initial_increment| {
                            let mut increment =
                                pg_crud_common::query_part_increment::QueryPartIncrement::from(
                                    initial_increment,
                                );
                            let result = filter.query_part(&mut increment);
                            matches!(
                                result,
                                Ok(pg_crud_common::query_part_fragment::QueryPartFragment { .. })
                            ) && increment.get() == initial_increment
                        })
                    })
            })
    );
}

#[test]
fn test_every_generated_table_rejects_malformed_filter_json() {
    assert!(
        crate::admin_generated_table::AdminGeneratedTable::ALL
            .iter()
            .all(|table| {
                matches!(
                    table.parse_filter(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                        constants_str::EMPTY
                    )),
                    Err(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                )
            })
    );
}

#[test]
fn test_every_generated_table_rejects_invalid_filter_json_shapes() {
    let values = [
        serde_json::json!(false),
        serde_json::json!(1i32),
        serde_json::json!(constants_str::X),
        serde_json::json!([]),
    ];
    assert!(values.into_iter().all(|value| {
        let text = value.to_string();
        crate::admin_generated_table::AdminGeneratedTable::ALL
            .iter()
            .all(|table| {
                matches!(
                    table.parse_filter(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                        text.as_str()
                    )),
                    Err(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                )
            })
    }));
}

#[test]
fn test_auxiliary_table_empty_filters_preserve_placeholder_counter() {
    let text = serde_json::Value::Null.to_string();
    let filters =
        [
            serde_json::from_str::<
                crate::admin_cleanup_status::StdOptionalOptionalAdminCleanupStatusWhereMany,
            >(text.as_str())
            .map(crate::data_cleanup_status_filter::DataCleanupStatusFilter::from)
            .map(crate::data_table_filter::DataTableFilter::CleanupStatus),
            serde_json::from_str::<
                crate::admin_login_attempts::StdOptionalOptionalAdminLoginAttemptsWhereMany,
            >(text.as_str())
            .map(crate::data_login_attempts_filter::DataLoginAttemptsFilter::from)
            .map(crate::data_table_filter::DataTableFilter::LoginAttempts),
            serde_json::from_str::<
                crate::admin_rate_limits::StdOptionalOptionalAdminRateLimitsWhereMany,
            >(text.as_str())
            .map(crate::data_rate_limits_filter::DataRateLimitsFilter::from)
            .map(crate::data_table_filter::DataTableFilter::RateLimits),
            serde_json::from_str::<
                crate::admin_refresh_tokens::StdOptionalOptionalAdminRefreshTokensWhereMany,
            >(text.as_str())
            .map(crate::data_refresh_tokens_filter::DataRefreshTokensFilter::from)
            .map(crate::data_table_filter::DataTableFilter::RefreshTokens),
        ];
    assert!(
        filters
            .into_iter()
            .all(|parsed_filter| parsed_filter.is_ok_and(|filter| {
                let mut increment =
                    pg_crud_common::query_part_increment::QueryPartIncrement::from(7u64);
                let result = filter.query_part(&mut increment);
                matches!(
                    result,
                    Ok(pg_crud_common::query_part_fragment::QueryPartFragment { .. })
                ) && increment.get() == 7u64
                    && filter
                        .query_bind(
                            pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(
                                sqlx::query(constants_str::EMPTY),
                            ),
                        )
                        .is_ok_and(|bound_query| {
                            let mut query = bound_query.into_inner();
                            sqlx::Execute::take_arguments(&mut query).is_ok_and(|arguments| {
                                arguments.is_none_or(|arguments| {
                                    sqlx::Arguments::len(&arguments) == 0usize
                                })
                            })
                        })
            }))
    );
}

#[test]
fn test_generated_empty_filters_do_not_bind_query_arguments() {
    let text = serde_json::Value::Null.to_string();
    assert!(
        crate::admin_generated_table::AdminGeneratedTable::ALL
            .iter()
            .all(|table| {
                table
                    .parse_filter(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                        text.as_str(),
                    ))
                    .is_ok_and(|filter| {
                        let sqlx_postgres_query =
                            pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(
                                sqlx::query(constants_str::EMPTY),
                            );
                        filter
                            .query_bind(sqlx_postgres_query)
                            .is_ok_and(|bound_query| {
                                let mut query = bound_query.into_inner();
                                sqlx::Execute::take_arguments(&mut query).is_ok_and(|arguments| {
                                    arguments.is_none_or(|arguments| {
                                        sqlx::Arguments::len(&arguments) == 0usize
                                    })
                                })
                            })
                    })
            })
    );
}
