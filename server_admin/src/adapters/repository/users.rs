#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

pub(crate) async fn insert_user(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    login: &server_admin_contract::domain_types::AdminLogin,
    display_name: &server_admin_contract::domain_types::AdminDisplayName,
    password_hash: &crate::domain_types::AdminPasswordHash,
) -> Result<crate::domain_types::AdminUserId, crate::domain_types::SqlxAdminError> {
    sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_INSERT_USER_SQL)
        .bind(login.as_ref())
        .bind(display_name.as_ref())
        .bind(password_hash.expose().as_ref())
        .fetch_one(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .and_then(|value| {
            crate::domain_types::AdminUserId::try_from(value)
                .map_err(crate::domain_types::SqlxAdminError::from)
        })
}

pub(crate) async fn revoke_refresh_token(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    token_hash: &crate::domain_types::AdminTokenHash,
    user_id: crate::domain_types::AdminUserId,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_REFRESH_TOKEN_SQL)
        .bind(token_hash.expose().as_ref())
        .bind(user_id.get())
        .execute(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn update_user_password(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
    password_hash: &crate::domain_types::AdminPasswordHash,
    password_change_required: crate::domain_types::AdminPasswordChangeRequired,
) -> Result<crate::domain_types::StdAdminBool, crate::domain_types::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_PASSWORD_SQL)
        .bind(user_id.get())
        .bind(password_hash.expose().as_ref())
        .bind(*password_change_required)
        .fetch_optional(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| crate::domain_types::StdAdminBool::from(value.is_some()))
}
