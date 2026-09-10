pub(crate) async fn load_role_permission_catalog(
    sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<
    server_admin_contract::admin_permission_summaries::AdminPermissionSummaries,
    crate::admin_repository_error::AdminRepositoryError,
> {
    let values =
        sqlx::query_as::<_, (i64, String)>(constants_str::SERVER_ADMIN_LIST_PERMISSIONS_SQL)
            .fetch_all(sqlx_pg_pool_ref.as_ref())
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?
            .into_iter()
            .map(|(id, name)| {
                Ok(
                server_admin_contract::admin_permission_summary::AdminPermissionSummary::new(
                    server_admin_contract::admin_permission_id::AdminPermissionId::try_from(id)
                        .map_err(|_error| {
                            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                        })?,
                    server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
                        name,
                    )
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?,
                ),
            )
            })
            .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()?;
    server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(values)
        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
}
