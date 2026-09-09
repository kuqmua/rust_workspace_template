pub(crate) async fn load_users_role_catalog(
    sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<
    server_admin_contract::admin_role_summaries::AdminRoleSummaries,
    crate::admin_repository_error::AdminRepositoryError,
> {
    let rows = sqlx::query_as::<_, (i64, String, bool)>(constants_str::SERVER_ADMIN_LIST_ROLES_SQL)
        .fetch_all(sqlx_pg_pool_ref.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    let role_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
    let links =
        sqlx::query_as::<_, (i64, i64)>(constants_str::SERVER_ADMIN_LIST_ROLE_PERMISSION_IDS_SQL)
            .bind(role_ids.as_slice())
            .fetch_all(sqlx_pg_pool_ref.as_ref())
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    let mut permission_ids_by_role = links.into_iter().try_fold(
        std::collections::HashMap::<
            i64,
            Vec<server_admin_contract::admin_permission_id::AdminPermissionId>,
        >::with_capacity(role_ids.len()),
        |mut values, (role_id, permission_id)| {
            values.entry(role_id).or_default().push(
                server_admin_contract::admin_permission_id::AdminPermissionId::try_from(
                    permission_id,
                )
                .map_err(|_error| {
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                })?,
            );
            Ok::<_, crate::admin_repository_error::AdminRepositoryError>(values)
        },
    )?;
    let values =
        rows.into_iter()
            .map(|(id, name, is_system)| {
                Ok(server_admin_contract::admin_role_summary::AdminRoleSummary::new(
                    server_admin_contract::admin_role_id::AdminRoleId::try_from(id).map_err(
                        |_error| {
                            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::admin_bool::AdminBool::from(is_system),
                    server_admin_contract::admin_role_name::AdminRoleName::try_from(name).map_err(
                        |_error| {
                            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::admin_permission_ids::AdminPermissionIds::try_from(
                        permission_ids_by_role.remove(&id).unwrap_or_default(),
                    )
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?,
                ))
            })
            .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()?;
    server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(values)
        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
}
