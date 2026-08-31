pub mod admin_command;
pub mod administrator_account_command_error;
pub mod administrator_account_command_exit_code;
pub mod administrator_account_command_status;
pub mod administrator_command_args_error;
pub mod administrator_password_file_path_buf;
pub mod domain_types;
#[cfg(test)]
pub mod error_status;
pub mod initial_administrator_creation_args;
#[cfg(test)]
pub mod password_from_bytes;
pub mod password_from_file;
pub mod password_reset_args;
pub mod sqlx_administrator_database_connection_error;

fn main() -> administrator_account_command_exit_code::AdministratorAccountCommandExitCode {
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            tracing::error!(
                error = %error,
                message = %constants_str::TRACING_ADMIN_RUNTIME_CREATION_FAILED,
            );
            return administrator_account_command_exit_code::AdministratorAccountCommandExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    let run_result = runtime.block_on(async {
        let command = (|| {
            let mut args = std::env::args_os().skip(constants_usize::ONE);
            let login_arg = args.next().ok_or(administrator_command_args_error::AdministratorCommandArgsError::Usage)?;
            Ok(
                if login_arg == std::ffi::OsStr::new(constants_str::VALUE_01BE30BB) {
                    let reset_login_arg =
                        args.next().ok_or(administrator_command_args_error::AdministratorCommandArgsError::Usage)?;
                    let password_file = args.next().ok_or(administrator_command_args_error::AdministratorCommandArgsError::Usage)?;
                    if args.next().is_some() {
                        return Err(administrator_command_args_error::AdministratorCommandArgsError::Usage);
                    }
                    let login = reset_login_arg.into_string().map_err(|value| {
                        drop(value);
                        administrator_command_args_error::AdministratorCommandArgsError::Login
                    })?;
                    admin_command::AdminCommand::PasswordReset(password_reset_args::PasswordResetArgs::new(
                        server_admin_contract::admin_login::AdminLogin::try_from(login).map_err(
                            |error| {
                                let _error_text = format!("{error:?}");
                                administrator_command_args_error::AdministratorCommandArgsError::Login
                            },
                        )?,
                        administrator_password_file_path_buf::AdministratorPasswordFilePathBuf::from(
                            std::path::PathBuf::from(password_file),
                        ),
                    ))
                } else {
                    let display_name_arg =
                        args.next().ok_or(administrator_command_args_error::AdministratorCommandArgsError::Usage)?;
                    let password_file = args.next().ok_or(administrator_command_args_error::AdministratorCommandArgsError::Usage)?;
                    if args.next().is_some() {
                        return Err(administrator_command_args_error::AdministratorCommandArgsError::Usage);
                    }
                    let login = login_arg.into_string().map_err(|value| {
                        drop(value);
                        administrator_command_args_error::AdministratorCommandArgsError::Login
                    })?;
                    let display_name = display_name_arg.into_string().map_err(|value| {
                        drop(value);
                        administrator_command_args_error::AdministratorCommandArgsError::DisplayName
                    })?;
                    admin_command::AdminCommand::CreateInitialAdministrator(initial_administrator_creation_args::InitialAdministratorCreationArgs::new(
                        server_admin_contract::admin_display_name::AdminDisplayName::try_from(
                            display_name,
                        )
                        .map_err(|error| {
                            let _error_text = format!("{error:?}");
                            administrator_command_args_error::AdministratorCommandArgsError::DisplayName
                        })?,
                        server_admin_contract::admin_login::AdminLogin::try_from(login).map_err(
                            |error| {
                                let _error_text = format!("{error:?}");
                                administrator_command_args_error::AdministratorCommandArgsError::Login
                            },
                        )?,
                        administrator_password_file_path_buf::AdministratorPasswordFilePathBuf::from(
                            std::path::PathBuf::from(password_file),
                        ),
                    ))
                },
            )
        })()
        .map_err(administrator_account_command_error::AdministratorAccountCommandError::Args)?;
        let config = server_config::server_config::ServerConfig::try_from_env()
            .map_err(administrator_account_command_error::AdministratorAccountCommandError::Config)?;
        config
            .validate_for_startup()
            .map_err(administrator_account_command_error::AdministratorAccountCommandError::ConfigProduction)?;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1u32)
            .connect(secrecy::ExposeSecret::expose_secret(
                config_lib::domain_types::DatabaseUrlProvider::database_url(&config),
            ))
            .await
            .map_err(|error| {
                administrator_account_command_error::AdministratorAccountCommandError::Connect(
                    sqlx_administrator_database_connection_error::SqlxAdministratorDatabaseConnectionError::from(error),
                )
            })?;
        server_admin::prepare_postgresql::prepare_postgresql(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
        )
        .await
        .map_err(administrator_account_command_error::AdministratorAccountCommandError::Migrate)?;
        let concurrency = std::num::NonZeroUsize::new(config.get_admin_password_hash_concurrency().get())
            .ok_or(administrator_account_command_error::AdministratorAccountCommandError::PasswordFileValue)?;
        let password_hasher = server_admin::admin_password_hasher::AdminPasswordHasher::new(
            server_admin::runtime_admin_password_hash_concurrency::RuntimeAdminPasswordHashConcurrency::from(
                concurrency,
            ),
        );
        match command {
            admin_command::AdminCommand::CreateInitialAdministrator(args) => {
                let (display_name, login, password_file) = args.into_parts();
                let password = password_from_file::password_from_file(&password_file)?;
                server_admin::create_initial_administrator::create_initial_administrator(
                    app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
                    login,
                    display_name,
                    password,
                    &password_hasher,
                )
                .await
                .map_err(administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation)
            }
            admin_command::AdminCommand::PasswordReset(args) => {
                let (login, password_file) = args.into_parts();
                let password = password_from_file::password_from_file(&password_file)?;
                server_admin::reset_admin_password::reset_admin_password(
                    app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
                    login,
                    password,
                    &password_hasher,
                )
                .await
                .map_err(administrator_account_command_error::AdministratorAccountCommandError::PasswordReset)
            }
        }
    });
    match run_result {
        Ok(user_id) => {
            tracing::info!(
                user_id = %user_id,
                message = %constants_str::TRACING_ADMIN_CMD_COMPLETED,
            );
            administrator_account_command_exit_code::AdministratorAccountCommandExitCode::from(
                std::process::ExitCode::SUCCESS,
            )
        }
        Err(error) => {
            tracing::error!(
                error = %error,
                message = %constants_str::TRACING_ADMIN_CMD_FAILED,
            );
            let status = administrator_account_command_status::AdministratorAccountCommandStatus::from(match error {
                administrator_account_command_error::AdministratorAccountCommandError::Args(_)
                | administrator_account_command_error::AdministratorAccountCommandError::PasswordFileValue => 2u8,
                administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(
                    server_admin::initial_administrator_creation_error::InitialAdministratorCreationError::AlreadyInitialized,
                ) => 3u8,
                administrator_account_command_error::AdministratorAccountCommandError::Config(_)
                | administrator_account_command_error::AdministratorAccountCommandError::ConfigProduction(_)
                | administrator_account_command_error::AdministratorAccountCommandError::Connect(_)
                | administrator_account_command_error::AdministratorAccountCommandError::Migrate(_)
                | administrator_account_command_error::AdministratorAccountCommandError::PasswordFile(_)
                | administrator_account_command_error::AdministratorAccountCommandError::PasswordReset(_)
                | administrator_account_command_error::AdministratorAccountCommandError::InitialAdministratorCreation(_) => 1u8,
            });
            administrator_account_command_exit_code::AdministratorAccountCommandExitCode::from(
                std::process::ExitCode::from(u8::from(status)),
            )
        }
    }
}
