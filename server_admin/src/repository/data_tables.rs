#![allow(clippy::single_call_fn)] // one bounded query serves the read-only table inspection boundary

pub(crate) async fn read(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    table: server_admin_contract::AdminDataTable,
) -> Result<server_admin_contract::AdminDataTableView, super::AdminRepositoryError> {
    let (column_names, sql) = match table {
        server_admin_contract::AdminDataTable::AccessSessions => (
            str_constants::SERVER_ADMIN_DATA_ACCESS_SESSIONS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_ACCESS_SESSIONS_SQL,
        ),
        server_admin_contract::AdminDataTable::AuditLog => (
            str_constants::SERVER_ADMIN_DATA_AUDIT_LOG_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_AUDIT_LOG_SQL,
        ),
        server_admin_contract::AdminDataTable::CleanupStatus => (
            str_constants::SERVER_ADMIN_DATA_CLEANUP_STATUS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_CLEANUP_STATUS_SQL,
        ),
        server_admin_contract::AdminDataTable::LoginAttempts => (
            str_constants::SERVER_ADMIN_DATA_LOGIN_ATTEMPTS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_LOGIN_ATTEMPTS_SQL,
        ),
        server_admin_contract::AdminDataTable::MfaRecoveryCodes => (
            str_constants::SERVER_ADMIN_DATA_MFA_RECOVERY_CODES_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_MFA_RECOVERY_CODES_SQL,
        ),
        server_admin_contract::AdminDataTable::Permissions => (
            str_constants::SERVER_ADMIN_DATA_PERMISSIONS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_PERMISSIONS_SQL,
        ),
        server_admin_contract::AdminDataTable::RateLimits => (
            str_constants::SERVER_ADMIN_DATA_RATE_LIMITS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_RATE_LIMITS_SQL,
        ),
        server_admin_contract::AdminDataTable::RefreshTokens => (
            str_constants::SERVER_ADMIN_DATA_REFRESH_TOKENS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_REFRESH_TOKENS_SQL,
        ),
        server_admin_contract::AdminDataTable::RolePermissions => (
            str_constants::SERVER_ADMIN_DATA_ROLE_PERMISSIONS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_ROLE_PERMISSIONS_SQL,
        ),
        server_admin_contract::AdminDataTable::Roles => (
            str_constants::SERVER_ADMIN_DATA_ROLES_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_ROLES_SQL,
        ),
        server_admin_contract::AdminDataTable::SystemSettings => (
            str_constants::SERVER_ADMIN_DATA_SYSTEM_SETTINGS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_SYSTEM_SETTINGS_SQL,
        ),
        server_admin_contract::AdminDataTable::UserMfa => (
            str_constants::SERVER_ADMIN_DATA_USER_MFA_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_USER_MFA_SQL,
        ),
        server_admin_contract::AdminDataTable::UserRoles => (
            str_constants::SERVER_ADMIN_DATA_USER_ROLES_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_USER_ROLES_SQL,
        ),
        server_admin_contract::AdminDataTable::Users => (
            str_constants::SERVER_ADMIN_DATA_USERS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_USERS_SQL,
        ),
    };
    let columns = column_names
        .split(',')
        .map(|value| {
            server_admin_contract::AdminText::try_from(value.to_owned())
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
        })
        .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    let rows = sqlx::query_scalar::<_, Vec<Option<String>>>(sql)
        .fetch_all(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let items = rows
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|value| {
                    server_admin_contract::AdminText::try_from(
                        value.unwrap_or_else(|| str_constants::SERVER_ADMIN_DATA_NULL.to_owned()),
                    )
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
                })
                .collect::<Result<Vec<_>, super::AdminRepositoryError>>()
                .map(server_admin_contract::AdminDataRow::new)
        })
        .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    Ok(server_admin_contract::AdminDataTableView::new(
        columns, items, table,
    ))
}
