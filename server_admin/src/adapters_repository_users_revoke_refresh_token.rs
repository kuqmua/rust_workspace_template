#![allow(clippy::single_call_fn)] // typed function owns one SQL bind/result contract

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
