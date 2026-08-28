pub(crate) async fn update_user_password(
    connection: crate::SqlxAdminRepositoryConnectionMutRef<'_>,
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
