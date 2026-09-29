pub(crate) async fn data_tables_get(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_data_table: server_admin_contract::admin_data_table::AdminDataTable,
    axum_admin_query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_data_table_query::AdminDataTableQuery,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let _actor = crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        admin_auth_request.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(admin_auth_request.get_headers().as_ref()),
        *admin_auth_request.get_peer(),
        admin_data_table.rule().as_str(),
        server_admin_core::std_admin_bool::StdAdminBool::from(false),
    )
    .await?;
    let pool = crate::sqlx_admin_repository_pool_ref::SqlxAdminRepositoryPoolRef::from(
        admin_auth_request.get_state().as_ref().get_pool().as_ref(),
    );
    let search = axum_admin_query.page().search().as_ref();
    let search_pattern = server_admin_core::escape_admin_like_pattern::escape_admin_like_pattern(
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(search),
    )
    .map_err(crate::admin_secret_text_error::AdminSecretTextError::from)
    .map_err(crate::admin_error::AdminError::secret_text)?;
    let view = async {
        let admin_generated_table =
            crate::admin_generated_table::AdminGeneratedTable::for_data_table(admin_data_table);
        let columns =
            crate::admin_data_columns::admin_data_columns(admin_data_table, admin_generated_table)?;
        let filter = crate::data_filter::data_filter(admin_data_table, axum_admin_query.filter())?;
        let mut increment =
            pg_crud_common::query_part_increment::QueryPartIncrement::from(constants_u64::ZERO);
        let fragment = filter
            .as_ref()
            .map(|value| value.query_part(&mut increment))
            .transpose()
            .map_err(|_error| {
                crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
            })?;
        let (count_sql, sql) = crate::data_table_query_sql::data_table_query_sql(
            admin_data_table,
            axum_admin_query.page(),
            fragment.as_ref(),
            increment,
        )?;
        let unbound_count_query = sqlx::query(sqlx::AssertSqlSafe(count_sql.as_ref().as_str()));
        let bound_count_query = filter
            .clone()
            .map(|value| {
                value.query_bind(
                    pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(
                        unbound_count_query,
                    ),
                )
            })
            .transpose()
            .map_err(|_error| {
                crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
            })?
            .map_or_else(
                || sqlx::query(sqlx::AssertSqlSafe(count_sql.as_ref().as_str())),
                pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::into_inner,
            );
        let bound_count_query = if search.is_empty() {
            bound_count_query
        } else {
            bound_count_query.bind(search_pattern.as_ref().as_str())
        };
        let count_row = bound_count_query
            .fetch_one(*pool)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let total = sqlx::Row::try_get::<i64, _>(&count_row, constants_usize::ZERO)
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let unbound_data_query = sqlx::query(sqlx::AssertSqlSafe(sql.as_ref().as_str()));
        let bound_data_query = filter
            .map(|value| {
                value.query_bind(
                    pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(
                        unbound_data_query,
                    ),
                )
            })
            .transpose()
            .map_err(|_error| {
                crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
            })?
            .map_or_else(
                || sqlx::query(sqlx::AssertSqlSafe(sql.as_ref().as_str())),
                pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::into_inner,
            );
        let bound_data_query = if search.is_empty() {
            bound_data_query
        } else {
            bound_data_query.bind(search_pattern.as_ref().as_str())
        };
        let bound_data_query = bound_data_query
            .bind(i64::from(u16::from(axum_admin_query.page().limit())))
            .bind(i64::from(u32::from(axum_admin_query.page().offset())));
        let rows = bound_data_query
            .fetch_all(*pool)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?
            .into_iter()
            .map(|row| {
                sqlx::Row::try_get::<Vec<Option<String>>, _>(&row, constants_usize::ZERO)
                    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
            })
            .collect::<Result<Vec<_>, crate::sqlx_admin_error::SqlxAdminError>>()?;
        let items = rows
            .into_iter()
            .map(|row| {
                let values = row
                    .into_iter()
                    .map(|value| {
                        server_admin_contract::admin_text::AdminText::try_from(
                            value.unwrap_or_else(|| {
                                constants_str::SERVER_ADMIN_DATA_NULL.to_owned()
                            }),
                        )
                        .map_err(|_error| {
                            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                        })
                    })
                    .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>(
                    )?;
                server_admin_contract::admin_texts::AdminTexts::try_from(values)
                    .map(server_admin_contract::admin_data_row::AdminDataRow::new)
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })
            })
            .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()?;
        Ok(
            server_admin_contract::admin_data_table_view::AdminDataTableView::new(
                columns,
                server_admin_contract::admin_data_rows::AdminDataRows::try_from(items).map_err(
                    |_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    },
                )?,
                admin_data_table,
                crate::repository_page_total::repository_page_total(
                    crate::admin_page_total_count::AdminPageTotalCount::from(total),
                )?,
            ),
        )
    }
    .await
    .map_err(crate::map_repository_error::map_repository_error)?;
    Ok(crate::json_response::json_response(view))
}
