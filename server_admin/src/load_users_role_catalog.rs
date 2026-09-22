#[allow(
    clippy::single_call_fn,
    reason = "the user-management SSR repository query remains a named ownership boundary for its independently testable role catalog conversion"
)]
pub(crate) async fn load_users_role_catalog(
    sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<
    server_admin_contract::admin_role_summaries::AdminRoleSummaries,
    crate::admin_repository_error::AdminRepositoryError,
> {
    let rows = sqlx::query_as::<_, (i64, String, bool, String, String)>(
        constants_str::SERVER_ADMIN_LIST_ROLES_SQL,
    )
    .fetch_all(sqlx_pg_pool_ref.as_ref())
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    let values =
        rows.into_iter()
            .map(|(id, name, is_system, created_at, updated_at)| {
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
                    server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::try_from(
                        created_at,
                    )
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?,
                    server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::try_from(
                        updated_at,
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
