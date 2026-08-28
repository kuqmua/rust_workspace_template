use crate::AdminPasswordResetError;

pub async fn reset_admin_password(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    login: crate::AdminLogin,
    password: server_admin_contract::domain_types::AdminNewPassword,
    password_hasher: &crate::AdminPasswordHasher,
) -> Result<crate::AdminUserId, AdminPasswordResetError> {
    crate::migrations::migrate_reset_admin_password(pool, login, password, password_hasher).await
}
