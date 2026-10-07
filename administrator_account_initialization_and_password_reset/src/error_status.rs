pub(crate) fn error_status(
    administrator_account_command_error: &crate::administrator_account_command_error::AdministratorAccountCommandError,
) -> crate::administrator_account_command_status::AdministratorAccountCommandStatus {
    crate::administrator_account_command_status::AdministratorAccountCommandStatus::from(match administrator_account_command_error {
        crate::administrator_account_command_error::AdministratorAccountCommandError::Args(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFileValue => 2u8,
        crate::administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(
            server_admin::initial_administrator_creation_error::InitialAdministratorCreationError::AlreadyInitialized,
        ) => 3u8,
        crate::administrator_account_command_error::AdministratorAccountCommandError::Config(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::ConfigProduction(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::Connect(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::Migrate(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFile(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordReset(_)
        | crate::administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(_) => 1u8,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_administrator_configuration_read_error_returns_failure_status() {
        let env_var_name_result = config_lib::env_var_name::EnvVarName::try_from(
            constants_str::ENV_NAMES_SERVICE_SOCKET_ADDRESS.to_owned(),
        );
        assert!(env_var_name_result.is_ok());
        let Ok(env_var_name) = env_var_name_result else {
            return;
        };
        let administrator_account_command_error =
            crate::administrator_account_command_error::AdministratorAccountCommandError::Config(
                server_config::server_config::ServerConfigTryFromEnvError::StdEnvVarError {
                    std_env_var_error: std::env::VarError::NotPresent,
                    env_var_name,
                },
            );
        assert_eq!(
            crate::error_status::error_status(&administrator_account_command_error),
            crate::administrator_account_command_status::AdministratorAccountCommandStatus::from(
                1u8
            ),
        );
    }

    #[tokio::test]
    async fn test_administrator_password_hashing_errors_return_failure_status() {
        let semaphore = tokio::sync::Semaphore::new(0);
        semaphore.close();
        let creation_acquire_error = semaphore.acquire().await.err();
        assert!(creation_acquire_error.is_some());
        let Some(creation_error) = creation_acquire_error else {
            return;
        };
        let hash_error =
            server_admin::admin_password_hash_error::AdminPasswordHashError::SemaphoreClosed(
                server_admin::tokio_admin_acquire_error::TokioAdminAcquireError::from(
                    creation_error,
                ),
            );
        let creation = crate::administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(
            server_admin::initial_administrator_creation_error::InitialAdministratorCreationError::PasswordHash(hash_error),
        );
        assert_eq!(
            crate::error_status::error_status(&creation),
            crate::administrator_account_command_status::AdministratorAccountCommandStatus::from(
                1u8
            ),
        );
        let reset_acquire_error = semaphore.acquire().await.err();
        assert!(reset_acquire_error.is_some());
        let Some(reset_error) = reset_acquire_error else {
            return;
        };
        let reset = crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordReset(
            server_admin::admin_password_reset_error::AdminPasswordResetError::PasswordHash(
                server_admin::admin_password_hash_error::AdminPasswordHashError::SemaphoreClosed(
                    server_admin::tokio_admin_acquire_error::TokioAdminAcquireError::from(reset_error),
                ),
            ),
        );
        assert_eq!(
            crate::error_status::error_status(&reset),
            crate::administrator_account_command_status::AdministratorAccountCommandStatus::from(
                1u8
            ),
        );
    }

    #[test]
    fn test_administrator_operational_command_variants_return_failure_status() {
        let database_error =
            || server_admin::sqlx_admin_error::SqlxAdminError::from(sqlx::Error::PoolClosed);
        let errors = [
            crate::administrator_account_command_error::AdministratorAccountCommandError::ConfigProduction(server_config::production_config_error::ProductionConfigError::AdminCookieInsecure),
            crate::administrator_account_command_error::AdministratorAccountCommandError::ConfigProduction(server_config::production_config_error::ProductionConfigError::AdminSwaggerEnabled),
            crate::administrator_account_command_error::AdministratorAccountCommandError::ConfigProduction(server_config::production_config_error::ProductionConfigError::CorsOriginInsecure),
            crate::administrator_account_command_error::AdministratorAccountCommandError::ConfigProduction(server_config::production_config_error::ProductionConfigError::DevelopmentJwtSecret),
            crate::administrator_account_command_error::AdministratorAccountCommandError::Connect(crate::sqlx_administrator_database_connection_error::SqlxAdministratorDatabaseConnectionError::from(sqlx::Error::PoolClosed)),
            crate::administrator_account_command_error::AdministratorAccountCommandError::Migrate(server_admin::admin_migrate_error::AdminMigrateError::Reconciliation(database_error())),
            crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordReset(server_admin::admin_password_reset_error::AdminPasswordResetError::AuditDetails),
            crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordReset(server_admin::admin_password_reset_error::AdminPasswordResetError::InvalidLogin),
            crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordReset(server_admin::admin_password_reset_error::AdminPasswordResetError::InvalidPassword),
            crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordReset(server_admin::admin_password_reset_error::AdminPasswordResetError::UnknownLogin),
            crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordReset(server_admin::admin_password_reset_error::AdminPasswordResetError::Pg(database_error())),
            crate::administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(server_admin::initial_administrator_creation_error::InitialAdministratorCreationError::Pg(database_error())),
        ];
        errors.iter().fold((), |(), administrator_account_command_error| {
            assert_eq!(
                crate::error_status::error_status(administrator_account_command_error),
                crate::administrator_account_command_status::AdministratorAccountCommandStatus::from(1u8),
            );
        });
    }

    #[test]
    fn test_administrator_command_operational_errors_and_invalid_password_file_exit_codes() {
        assert!([
            (crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFileValue, 2u8),
            (crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFile(server_runtime_http::bounded_read_error::BoundedReadError::LimiterClosed), 1u8),
            (crate::administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(server_admin::initial_administrator_creation_error::InitialAdministratorCreationError::AuditDetails), 1u8),
            (crate::administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(server_admin::initial_administrator_creation_error::InitialAdministratorCreationError::EmptyDisplayName), 1u8),
            (crate::administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(server_admin::initial_administrator_creation_error::InitialAdministratorCreationError::InvalidLogin), 1u8),
            (crate::administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(server_admin::initial_administrator_creation_error::InitialAdministratorCreationError::InvalidPassword), 1u8),
        ].into_iter().all(|(error, code)| crate::error_status::error_status(&error) == crate::administrator_account_command_status::AdministratorAccountCommandStatus::from(code)));
    }

    #[test]
    fn test_exit_codes_distinguish_invalid_input_and_completed_initial_administrator_creation() {
        assert_eq!(
            crate::error_status::error_status(
                &crate::administrator_account_command_error::AdministratorAccountCommandError::Args(
                    crate::administrator_command_args_error::AdministratorCommandArgsError::Usage,
                )
            ),
            crate::administrator_account_command_status::AdministratorAccountCommandStatus::from(
                2u8
            )
        );
        assert_eq!(
            crate::error_status::error_status(&crate::administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(
                server_admin::initial_administrator_creation_error::InitialAdministratorCreationError::AlreadyInitialized,
            )),
            crate::administrator_account_command_status::AdministratorAccountCommandStatus::from(3u8)
        );
    }
}
