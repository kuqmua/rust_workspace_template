mod admin_command;
mod administrator_account_command_error;
mod administrator_account_command_exit_code;
mod administrator_account_command_status;
mod administrator_command_args_error;
mod administrator_password_file_path_buf;
#[cfg(test)]
mod error_status;
mod initial_administrator_creation_args;
#[cfg(test)]
mod password_from_bytes;
mod password_from_file;
mod password_reset_args;
mod sqlx_administrator_database_connection_error;

pub(crate) use admin_command::AdminCommand;
pub(crate) use administrator_account_command_error::AdministratorAccountCommandError;
pub(crate) use administrator_account_command_exit_code::AdministratorAccountCommandExitCode;
pub(crate) use administrator_account_command_status::AdministratorAccountCommandStatus;
pub(crate) use administrator_command_args_error::AdministratorCommandArgsError;
pub(crate) use administrator_password_file_path_buf::AdministratorPasswordFilePathBuf;
pub(crate) use initial_administrator_creation_args::InitialAdministratorCreationArgs;
pub(crate) use password_reset_args::PasswordResetArgs;
pub(crate) use sqlx_administrator_database_connection_error::SqlxAdministratorDatabaseConnectionError;

fn main() -> AdministratorAccountCommandExitCode {
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            tracing::error!(error = %error, "failed to create initial administrator creation runtime");
            return AdministratorAccountCommandExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    let run_result = runtime.block_on(async {
        let command = (|| {
            let mut args = std::env::args_os().skip(constants_usize::ONE);
            let login_arg = args.next().ok_or(AdministratorCommandArgsError::Usage)?;
            Ok(
                if login_arg == std::ffi::OsStr::new(constants_str::VALUE_01BE30BB) {
                    let reset_login_arg =
                        args.next().ok_or(AdministratorCommandArgsError::Usage)?;
                    let password_file = args.next().ok_or(AdministratorCommandArgsError::Usage)?;
                    if args.next().is_some() {
                        return Err(AdministratorCommandArgsError::Usage);
                    }
                    let login = reset_login_arg.into_string().map_err(|value| {
                        drop(value);
                        AdministratorCommandArgsError::Login
                    })?;
                    AdminCommand::PasswordReset(PasswordResetArgs {
                        login: server_admin::domain_types::AdminLogin::try_from(login).map_err(
                            |error| {
                                let _error_text = format!("{error:?}");
                                AdministratorCommandArgsError::Login
                            },
                        )?,
                        password_file: AdministratorPasswordFilePathBuf::from(
                            std::path::PathBuf::from(password_file),
                        ),
                    })
                } else {
                    let display_name_arg =
                        args.next().ok_or(AdministratorCommandArgsError::Usage)?;
                    let password_file = args.next().ok_or(AdministratorCommandArgsError::Usage)?;
                    if args.next().is_some() {
                        return Err(AdministratorCommandArgsError::Usage);
                    }
                    let login = login_arg.into_string().map_err(|value| {
                        drop(value);
                        AdministratorCommandArgsError::Login
                    })?;
                    let display_name = display_name_arg.into_string().map_err(|value| {
                        drop(value);
                        AdministratorCommandArgsError::DisplayName
                    })?;
                    AdminCommand::CreateInitialAdministrator(InitialAdministratorCreationArgs {
                        display_name: server_admin::domain_types::AdminDisplayName::try_from(
                            display_name,
                        )
                        .map_err(|error| {
                            let _error_text = format!("{error:?}");
                            AdministratorCommandArgsError::DisplayName
                        })?,
                        login: server_admin::domain_types::AdminLogin::try_from(login).map_err(
                            |error| {
                                let _error_text = format!("{error:?}");
                                AdministratorCommandArgsError::Login
                            },
                        )?,
                        password_file: AdministratorPasswordFilePathBuf::from(
                            std::path::PathBuf::from(password_file),
                        ),
                    })
                },
            )
        })()
        .map_err(AdministratorAccountCommandError::Args)?;
        let config = server_config::domain_types::Config::try_from_env()
            .map_err(AdministratorAccountCommandError::Config)?;
        config
            .validate_for_startup()
            .map_err(AdministratorAccountCommandError::ConfigProduction)?;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1u32)
            .connect(secrecy::ExposeSecret::expose_secret(
                config_lib::domain_types::DatabaseUrlProvider::database_url(&config),
            ))
            .await
            .map_err(|error| {
                AdministratorAccountCommandError::Connect(
                    SqlxAdministratorDatabaseConnectionError::from(error),
                )
            })?;
        server_admin::domain_types::prepare_postgresql(app_state::SqlxPgPoolRef::from(&pool))
            .await
            .map_err(AdministratorAccountCommandError::Migrate)?;
        let concurrency = std::num::NonZeroUsize::new(config.admin_password_hash_concurrency.get())
            .ok_or(AdministratorAccountCommandError::PasswordFileValue)?;
        let password_hasher = server_admin::domain_types::AdminPasswordHasher::new(
            server_admin::domain_types::AdminPasswordHashConcurrency::from(concurrency),
        );
        match command {
            AdminCommand::CreateInitialAdministrator(args) => {
                let (display_name, login, password_file) = args.into_parts();
                let password = password_from_file::password_from_file(&password_file)?;
                server_admin::domain_types::create_initial_administrator(
                    app_state::SqlxPgPoolRef::from(&pool),
                    login,
                    display_name,
                    password,
                    &password_hasher,
                )
                .await
                .map_err(AdministratorAccountCommandError::InitialAdministratorCreation)
            }
            AdminCommand::PasswordReset(args) => {
                let (login, password_file) = args.into_parts();
                let password = password_from_file::password_from_file(&password_file)?;
                server_admin::domain_types::reset_admin_password(
                    app_state::SqlxPgPoolRef::from(&pool),
                    login,
                    password,
                    &password_hasher,
                )
                .await
                .map_err(AdministratorAccountCommandError::PasswordReset)
            }
        }
    });
    match run_result {
        Ok(user_id) => {
            tracing::info!(user_id = %user_id, "administrator operation completed");
            AdministratorAccountCommandExitCode::from(std::process::ExitCode::SUCCESS)
        }
        Err(error) => {
            tracing::error!(error = %error, "administrator operation failed");
            let status = AdministratorAccountCommandStatus::from(match error {
                AdministratorAccountCommandError::Args(_)
                | AdministratorAccountCommandError::PasswordFileValue => 2u8,
                AdministratorAccountCommandError::InitialAdministratorCreation(
                    server_admin::domain_types::InitialAdministratorCreationError::AlreadyInitialized,
                ) => 3u8,
                AdministratorAccountCommandError::Config(_)
                | AdministratorAccountCommandError::ConfigProduction(_)
                | AdministratorAccountCommandError::Connect(_)
                | AdministratorAccountCommandError::Migrate(_)
                | AdministratorAccountCommandError::PasswordFile(_)
                | AdministratorAccountCommandError::PasswordReset(_)
                | AdministratorAccountCommandError::InitialAdministratorCreation(_) => 1u8,
            });
            AdministratorAccountCommandExitCode::from(std::process::ExitCode::from(u8::from(
                status,
            )))
        }
    }
}
