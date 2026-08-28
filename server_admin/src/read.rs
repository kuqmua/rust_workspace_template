use crate::{base_sql, data_columns, data_filter, filtered_sql};

pub(crate) async fn read(
    pool: crate::SqlxAdminRepositoryPoolRef<'_>,
    table: server_admin_contract::domain_types::AdminDataTable,
    query: &server_admin_contract::domain_types::AdminDataTableQuery,
) -> Result<server_admin_contract::domain_types::AdminDataTableView, crate::AdminRepositoryError> {
    let spec = table.spec();
    let columns = data_columns(table, spec.columns())?;
    let (base_count_sql, base_sql) = base_sql(table)?;
    let filter = data_filter(table, query.filter())?;
    let mut increment = pg_crud_common::domain_types::QueryPartIncrement::from(constants_u64::ZERO);
    let fragment = filter
        .as_ref()
        .map(|value| value.query_part(&mut increment))
        .transpose()
        .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
    let (count_sql, sql) = fragment.as_ref().map_or_else(
        || {
            Ok((
                crate::domain_types::StdAdminString::try_from(base_count_sql.as_ref().to_owned())
                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
                crate::domain_types::StdAdminString::try_from(base_sql.as_ref().to_owned())
                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
            ))
        },
        |filter_fragment| {
            filtered_sql(
                crate::domain_types::StdAdminStrRef::from(base_count_sql.as_ref().as_str()),
                crate::domain_types::StdAdminStrRef::from(base_sql.as_ref().as_str()),
                filter_fragment,
                increment,
            )
        },
    )?;
    let unbound_count_query = sqlx::query(sqlx::AssertSqlSafe(count_sql.as_ref().as_str()));
    let bound_count_query = filter
        .clone()
        .map(|value| {
            value.query_bind(pg_crud_common::domain_types::SqlxPostgresQuery::from(
                unbound_count_query,
            ))
        })
        .transpose()
        .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?
        .map_or_else(
            || sqlx::query(sqlx::AssertSqlSafe(count_sql.as_ref().as_str())),
            pg_crud_common::domain_types::SqlxPostgresQuery::into_inner,
        );
    let count_row = bound_count_query
        .fetch_one(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?;
    let total = sqlx::Row::try_get::<i64, _>(&count_row, constants_usize::ZERO)
        .map_err(crate::domain_types::SqlxAdminError::from)?;
    let unbound_data_query = sqlx::query(sqlx::AssertSqlSafe(sql.as_ref().as_str()));
    let bound_data_query = filter
        .map(|value| {
            value.query_bind(pg_crud_common::domain_types::SqlxPostgresQuery::from(
                unbound_data_query,
            ))
        })
        .transpose()
        .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?
        .map_or_else(
            || sqlx::query(sqlx::AssertSqlSafe(sql.as_ref().as_str())),
            pg_crud_common::domain_types::SqlxPostgresQuery::into_inner,
        )
        .bind(i64::from(u16::from(query.page().limit())))
        .bind(i64::from(u32::from(query.page().offset())));
    let rows = bound_data_query
        .fetch_all(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?
        .into_iter()
        .map(|row| {
            sqlx::Row::try_get::<Vec<Option<String>>, _>(&row, constants_usize::ZERO)
                .map_err(crate::domain_types::SqlxAdminError::from)
        })
        .collect::<Result<Vec<_>, crate::domain_types::SqlxAdminError>>()?;
    let items = rows
        .into_iter()
        .map(|row| {
            let values = row
                .into_iter()
                .map(|value| {
                    server_admin_contract::domain_types::AdminText::try_from(
                        value.unwrap_or_else(|| constants_str::SERVER_ADMIN_DATA_NULL.to_owned()),
                    )
                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)
                })
                .collect::<Result<Vec<_>, crate::AdminRepositoryError>>()?;
            server_admin_contract::domain_types::AdminTexts::try_from(values)
                .map(server_admin_contract::domain_types::AdminDataRow::new)
                .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)
        })
        .collect::<Result<Vec<_>, crate::AdminRepositoryError>>()?;
    Ok(
        server_admin_contract::domain_types::AdminDataTableView::new(
            columns,
            server_admin_contract::domain_types::AdminDataRows::try_from(items)
                .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
            table,
            crate::repository_page_total(crate::AdminPageTotalCount::from(total))?,
        ),
    )
}
