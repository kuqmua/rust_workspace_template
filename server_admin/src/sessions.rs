#[allow(
    clippy::single_call_fn,
    reason = "sessions remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn sessions(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_data_table_query::AdminDataTableQuery,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let authenticated = crate::authorization_authenticate::authorization_authenticate(
        admin_auth_request.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(
            admin_auth_request.get_headers().as_ref(),
        ),
        *admin_auth_request.get_peer(),
    )
    .await?;
    let filter = crate::data_filter::data_filter(
        server_admin_contract::admin_data_table::AdminDataTable::AccessSessions,
        axum_admin_query.filter(),
    )
    .map_err(crate::map_repository_error::map_repository_error)?;
    let mut increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(1u64);
    let fragment = filter
        .as_ref()
        .map(|value| value.query_part(&mut increment))
        .transpose()
        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
        .and_then(|fragment| {
            fragment
                .map(|fragment| {
                    let predicate = fragment
                        .as_ref()
                        .strip_prefix(constants_str::WHERE_ALT.trim_end())
                        .ok_or(
                            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                        )?;
                    let mut sql = constants_str::AND_ALT.to_owned();
                    sql.push_str(predicate);
                    pg_crud_common::query_part_fragment::QueryPartFragment::try_from(sql).map_err(
                        |_error| {
                            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                        },
                    )
                })
                .transpose()
        })
        .map_err(crate::map_repository_error::map_repository_error)?;
    let count_sql = fragment
        .as_ref()
        .map_or_else(
            || {
                server_admin_core::std_admin_string::StdAdminString::try_from(
                    constants_str::SERVER_ADMIN_COUNT_ACTIVE_SESSIONS_SQL.to_owned(),
                )
                .map_err(|_error| {
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                })
            },
            |filter_fragment| {
                let mut sql = constants_str::SERVER_ADMIN_COUNT_ACTIVE_SESSIONS_SQL.to_owned();
                sql.push(' ');
                sql.push_str(filter_fragment.as_ref());
                server_admin_core::std_admin_string::StdAdminString::try_from(sql).map_err(
                    |_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    },
                )
            },
        )
        .map_err(crate::map_repository_error::map_repository_error)?;
    let data_sql = fragment
        .as_ref()
        .map_or_else(
            || {
                server_admin_core::std_admin_string::StdAdminString::try_from(
                    constants_str::SERVER_ADMIN_LIST_ACTIVE_SESSIONS_SQL.to_owned(),
                )
                .map_err(|_error| {
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                })
            },
            |filter_fragment| {
                let (data_prefix, ordered_suffix) =
                    constants_str::SERVER_ADMIN_LIST_ACTIVE_SESSIONS_SQL
                        .split_once(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR)
                        .ok_or(
                            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                        )?;
                let order = ordered_suffix
                    .split_once(constants_str::SERVER_ADMIN_FILTER_LIMIT_PREFIX)
                    .map(|(order, _limit)| order)
                    .ok_or(
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                    )?;
                let limit_index = increment.get().saturating_add(1u64);
                let offset_index = limit_index.saturating_add(1u64);
                let mut sql = data_prefix.to_owned();
                sql.push(' ');
                sql.push_str(filter_fragment.as_ref());
                sql.push_str(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR);
                sql.push_str(order);
                sql.push_str(constants_str::SERVER_ADMIN_FILTER_LIMIT_PREFIX);
                sql.push_str(limit_index.to_string().as_str());
                sql.push_str(constants_str::SERVER_ADMIN_FILTER_OFFSET_PREFIX);
                sql.push_str(offset_index.to_string().as_str());
                server_admin_core::std_admin_string::StdAdminString::try_from(sql).map_err(
                    |_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    },
                )
            },
        )
        .map_err(crate::map_repository_error::map_repository_error)?;
    let unbound_count_query = sqlx::query(sqlx::AssertSqlSafe(count_sql.as_ref().as_str()))
        .bind(authenticated.get_id().get());
    let bound_count_query = filter
        .clone()
        .map(|value| {
            value.query_bind(
                pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(unbound_count_query),
            )
        })
        .transpose()
        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
        .map_err(crate::map_repository_error::map_repository_error)?
        .map_or_else(
            || {
                sqlx::query(sqlx::AssertSqlSafe(count_sql.as_ref().as_str()))
                    .bind(authenticated.get_id().get())
            },
            pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::into_inner,
        );
    let count_row = bound_count_query
        .fetch_one(admin_auth_request.get_state().as_ref().get_pool().as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_error::AdminError::from)?;
    let total = sqlx::Row::try_get::<i64, _>(&count_row, constants_usize::ZERO)
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_error::AdminError::from)?;
    let unbound_data_query = sqlx::query(sqlx::AssertSqlSafe(data_sql.as_ref().as_str()))
        .bind(authenticated.get_id().get());
    let bound_data_query = filter
        .map(|value| {
            value.query_bind(
                pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(unbound_data_query),
            )
        })
        .transpose()
        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
        .map_err(crate::map_repository_error::map_repository_error)?
        .map_or_else(
            || {
                sqlx::query(sqlx::AssertSqlSafe(data_sql.as_ref().as_str()))
                    .bind(authenticated.get_id().get())
            },
            pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::into_inner,
        )
        .bind(i64::from(u16::from(axum_admin_query.page().limit())))
        .bind(i64::from(u32::from(axum_admin_query.page().offset())));
    let items = bound_data_query
        .fetch_all(admin_auth_request.get_state().as_ref().get_pool().as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_error::AdminError::from)?
        .into_iter()
        .map(|row| {
            let id = sqlx::Row::try_get::<uuid::Uuid, _>(&row, constants_usize::ZERO)
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
                .map_err(crate::admin_repository_error::AdminRepositoryError::from)?;
            let created_at = sqlx::Row::try_get::<String, _>(&row, constants_usize::ONE)
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
                .map_err(crate::admin_repository_error::AdminRepositoryError::from)?;
            let expires_at = sqlx::Row::try_get::<String, _>(&row, constants_usize::TWO)
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
                .map_err(crate::admin_repository_error::AdminRepositoryError::from)?;
            Ok(
            server_admin_contract::admin_session_view::AdminSessionView::new(
                server_admin_contract::admin_session_timestamp::AdminSessionTimestamp::try_from(
                    created_at,
                )
                .map_err(|_error| {
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                })?,
                server_admin_contract::admin_session_timestamp::AdminSessionTimestamp::try_from(
                    expires_at,
                )
                .map_err(|_error| {
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                })?,
                server_admin_contract::admin_session_identifier::AdminSessionIdentifier::try_from(
                    id.to_string(),
                )
                .map_err(|_error| {
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                })?,
                server_admin_contract::admin_bool::AdminBool::from(
                    id == authenticated.get_session_id().get().get(),
                ),
            ),
        )
        })
        .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()
        .map_err(crate::map_repository_error::map_repository_error)?;
    let page = server_admin_contract::admin_sessions_page::AdminSessionsPage::new(
        server_admin_contract::admin_session_views::AdminSessionViews::try_from(items)
            .map_err(|_error| {
                crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
            })
            .map_err(crate::map_repository_error::map_repository_error)?,
        crate::repository_page_total::repository_page_total(
            crate::admin_page_total_count::AdminPageTotalCount::from(total),
        )
        .map_err(crate::map_repository_error::map_repository_error)?,
    );
    Ok(crate::json_response::json_response(page))
}
