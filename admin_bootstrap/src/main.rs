#![allow(
    unused_crate_dependencies,
    reason = "str_constants is used by binary unit tests"
)]
#![allow(
    clippy::single_call_fn,
    reason = "the bootstrap command keeps parsing, secret loading, and database orchestration isolated"
)]

const PASSWORD_FILE_MAX_BYTES: usize = 1_024usize;

#[derive(optml::Optml, Debug, newtype::FromInner)]
struct StdBootstrapPath(std::path::PathBuf);
#[derive(optml::Optml, Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
struct SqlxBootstrapError(sqlx::Error);
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
struct BootstrapStatus(u8);
#[derive(optml::Optml, Debug)]
struct BootstrapArgs {
    display_name: server_admin::AdminDisplayName,
    login: server_admin::AdminLogin,
    password_file: StdBootstrapPath,
}
#[derive(optml::Optml, Debug)]
struct PasswordResetArgs {
    login: server_admin::AdminLogin,
    password_file: StdBootstrapPath,
}
#[derive(optml::Optml, Debug)]
enum AdminCommand {
    Bootstrap(BootstrapArgs),
    PasswordReset(PasswordResetArgs),
}
#[derive(optml::Optml, Debug, thiserror::Error)]
enum BootstrapArgsError {
    #[error("administrator bootstrap display name is invalid")]
    DisplayName,
    #[error("administrator bootstrap login is invalid")]
    Login,
    #[error(
        "usage: admin_bootstrap <login> <display_name> <password_file> | admin_bootstrap reset <login> <password_file>; password_file must contain only the new password"
    )]
    Usage,
}
#[derive(optml::Optml, Debug, thiserror::Error)]
enum BootstrapCommandError {
    #[error(transparent)]
    Args(BootstrapArgsError),
    #[error("failed to create the first administrator: {0}")]
    Bootstrap(server_admin::AdminBootstrapError),
    #[error("failed to read configuration: {0}")]
    Config(server_config::ConfigTryFromEnvError),
    #[error("unsafe production configuration: {0}")]
    ConfigProduction(server_config::ProductionConfigError),
    #[error("failed to connect to postgres: {0}")]
    Connect(SqlxBootstrapError),
    #[error("failed to prepare administrator schema: {0}")]
    Migrate(server_admin::AdminMigrateError),
    #[error("failed to read administrator bootstrap password file: {0}")]
    PasswordFile(server_runtime_http::BoundedReadError),
    #[error("administrator bootstrap password file is invalid")]
    PasswordFileValue,
    #[error("failed to reset the administrator password: {0}")]
    PasswordReset(server_admin::AdminPasswordResetError),
}
#[derive(optml::Optml, newtype::FromInner)]
struct StdBootstrapExitCode(std::process::ExitCode);
impl std::process::Termination for StdBootstrapExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}

fn parse_args() -> Result<AdminCommand, BootstrapArgsError> {
    let mut args = std::env::args_os().skip(1usize);
    let login_arg = args.next().ok_or(BootstrapArgsError::Usage)?;
    if login_arg == std::ffi::OsStr::new("reset") {
        let reset_login_arg = args.next().ok_or(BootstrapArgsError::Usage)?;
        let password_file = args.next().ok_or(BootstrapArgsError::Usage)?;
        if args.next().is_some() {
            return Err(BootstrapArgsError::Usage);
        }
        let login = reset_login_arg.into_string().map_err(|value| {
            drop(value);
            BootstrapArgsError::Login
        })?;
        return Ok(AdminCommand::PasswordReset(PasswordResetArgs {
            login: server_admin::AdminLogin::try_from(login).map_err(|error| {
                let _error_text = format!("{error:?}");
                BootstrapArgsError::Login
            })?,
            password_file: StdBootstrapPath::from(std::path::PathBuf::from(password_file)),
        }));
    }
    let display_name_arg = args.next().ok_or(BootstrapArgsError::Usage)?;
    let password_file = args.next().ok_or(BootstrapArgsError::Usage)?;
    if args.next().is_some() {
        return Err(BootstrapArgsError::Usage);
    }
    let login = login_arg.into_string().map_err(|value| {
        drop(value);
        BootstrapArgsError::Login
    })?;
    let display_name = display_name_arg.into_string().map_err(|value| {
        drop(value);
        BootstrapArgsError::DisplayName
    })?;
    Ok(AdminCommand::Bootstrap(BootstrapArgs {
        display_name: server_admin::AdminDisplayName::try_from(display_name).map_err(|error| {
            let _error_text = format!("{error:?}");
            BootstrapArgsError::DisplayName
        })?,
        login: server_admin::AdminLogin::try_from(login).map_err(|error| {
            let _error_text = format!("{error:?}");
            BootstrapArgsError::Login
        })?,
        password_file: StdBootstrapPath::from(std::path::PathBuf::from(password_file)),
    }))
}

fn password_from_file(
    password_file: &StdBootstrapPath,
) -> Result<server_admin_contract::AdminNewPassword, BootstrapCommandError> {
    let bytes = server_runtime_http::read_bounded_file(
        server_runtime_http::StdPathRef::from(password_file.0.as_path()),
        server_runtime_http::BoundedReadMaximumBytes::from(PASSWORD_FILE_MAX_BYTES),
    )
    .map_err(BootstrapCommandError::PasswordFile)?;
    password_from_bytes(bytes)
}

fn password_from_bytes(
    bytes: server_runtime_http::BoundedBytes,
) -> Result<server_admin_contract::AdminNewPassword, BootstrapCommandError> {
    let text = server_runtime_http::BoundedText::try_from(bytes)
        .map_err(BootstrapCommandError::PasswordFile)?;
    let mut password = text.into_inner();
    if password.ends_with('\n') {
        let _newline = password.pop();
        if password.ends_with('\r') {
            let _carriage_return = password.pop();
        }
    }
    server_admin_contract::AdminNewPassword::try_from(password).map_err(|error| {
        let _error_text = format!("{error:?}");
        BootstrapCommandError::PasswordFileValue
    })
}

async fn run() -> Result<server_admin::AdminUserId, BootstrapCommandError> {
    let command = parse_args().map_err(BootstrapCommandError::Args)?;
    let config = server_config::Config::try_from_env().map_err(BootstrapCommandError::Config)?;
    config
        .validate_for_startup()
        .map_err(BootstrapCommandError::ConfigProduction)?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1u32)
        .connect(secrecy::ExposeSecret::expose_secret(
            config_lib::GetDatabaseUrl::get_database_url(&config),
        ))
        .await
        .map_err(|error| BootstrapCommandError::Connect(SqlxBootstrapError::from(error)))?;
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool))
        .await
        .map_err(BootstrapCommandError::Migrate)?;
    let concurrency = std::num::NonZeroUsize::new(config.admin_password_hash_concurrency.get())
        .ok_or(BootstrapCommandError::PasswordFileValue)?;
    let password_hasher =
        server_admin::AdminPasswordHasher::new(server_admin::AdminPasswordHashConcurrency::from(
            server_admin::StdAdminNonZeroUsize::from(concurrency),
        ));
    match command {
        AdminCommand::Bootstrap(args) => {
            let password = password_from_file(&args.password_file)?;
            server_admin::bootstrap_admin(
                app_state::SqlxPgPoolRef::from(&pool),
                args.login,
                args.display_name,
                password,
                &password_hasher,
            )
            .await
            .map_err(BootstrapCommandError::Bootstrap)
        }
        AdminCommand::PasswordReset(args) => {
            let password = password_from_file(&args.password_file)?;
            server_admin::reset_admin_password(
                app_state::SqlxPgPoolRef::from(&pool),
                args.login,
                password,
                &password_hasher,
            )
            .await
            .map_err(BootstrapCommandError::PasswordReset)
        }
    }
}

#[allow(
    clippy::missing_const_for_fn,
    reason = "repository wrappers initialize through the non-const From trait"
)]
fn error_status(error: &BootstrapCommandError) -> BootstrapStatus {
    BootstrapStatus::from(match error {
        BootstrapCommandError::Args(_) | BootstrapCommandError::PasswordFileValue => 2u8,
        BootstrapCommandError::Bootstrap(server_admin::AdminBootstrapError::AlreadyInitialized) => {
            3u8
        }
        BootstrapCommandError::Config(_)
        | BootstrapCommandError::ConfigProduction(_)
        | BootstrapCommandError::Connect(_)
        | BootstrapCommandError::Migrate(_)
        | BootstrapCommandError::PasswordFile(_)
        | BootstrapCommandError::PasswordReset(_)
        | BootstrapCommandError::Bootstrap(_) => 1u8,
    })
}

fn main() -> StdBootstrapExitCode {
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            eprintln!("failed to create administrator bootstrap runtime: {error}");
            return StdBootstrapExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    match runtime.block_on(run()) {
        Ok(user_id) => {
            println!("administrator operation completed for identifier {user_id}");
            StdBootstrapExitCode::from(std::process::ExitCode::SUCCESS)
        }
        Err(error) => {
            eprintln!("{error}");
            StdBootstrapExitCode::from(std::process::ExitCode::from(error_status(&error).0))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exit_codes_distinguish_invalid_input_and_completed_bootstrap() {
        assert_eq!(
            super::error_status(&super::BootstrapCommandError::Args(
                super::BootstrapArgsError::Usage,
            )),
            super::BootstrapStatus::from(2u8)
        );
        assert_eq!(
            super::error_status(&super::BootstrapCommandError::Bootstrap(
                server_admin::AdminBootstrapError::AlreadyInitialized,
            )),
            super::BootstrapStatus::from(3u8)
        );
    }

    #[test]
    fn password_file_accepts_one_trailing_line_ending() {
        let password_text = str_constants::TEST_STRONG_PASSWORD;
        let password = super::password_from_bytes(server_runtime_http::BoundedBytes::from(
            format!("{password_text}\r\n").into_bytes(),
        ))
        .expect("05536bb6 password_file_accepts_one_trailing_line_ending invariant must hold");

        let debug = format!("{password:?}");
        assert!(debug.contains(str_constants::REDACTED_ALT_3));
        assert!(!debug.contains(password_text));
    }

    #[test]
    fn password_file_rejects_excess_bytes() {
        let Err(_error) =
            super::password_from_bytes(server_runtime_http::BoundedBytes::from(vec![
                b'a';
                super::PASSWORD_FILE_MAX_BYTES
                    .saturating_add(1usize)
            ]))
        else {
            panic!("7ad9edb5 expected an excessive password file to fail");
        };
    }
}
