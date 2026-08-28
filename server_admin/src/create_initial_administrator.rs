use crate::InitialAdministratorCreationError;

pub async fn create_initial_administrator(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    login: crate::AdminLogin,
    display_name: crate::AdminDisplayName,
    password: server_admin_contract::domain_types::AdminNewPassword,
    password_hasher: &crate::AdminPasswordHasher,
) -> Result<crate::AdminUserId, InitialAdministratorCreationError> {
    crate::migrations::migrate_create_initial_administrator(
        pool,
        login,
        display_name,
        password,
        password_hasher,
    )
    .await
}
