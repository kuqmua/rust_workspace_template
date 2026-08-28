use super::InitialAdministratorCreationError;

pub async fn create_initial_administrator(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    login: super::AdminLogin,
    display_name: super::AdminDisplayName,
    password: server_admin_contract::domain_types::AdminNewPassword,
    password_hasher: &super::AdminPasswordHasher,
) -> Result<super::AdminUserId, InitialAdministratorCreationError> {
    crate::migrations::migrate_create_initial_administrator(
        pool,
        login,
        display_name,
        password,
        password_hasher,
    )
    .await
}
