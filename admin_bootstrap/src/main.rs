#![allow(
    unused_crate_dependencies,
    reason = "constants_str is used by binary unit tests"
)]
#![allow(
    clippy::single_call_fn,
    reason = "the bootstrap command keeps parsing, secret loading, and database orchestration isolated"
)]

mod domain_types;

const PASSWORD_FILE_MAX_BYTES: usize = 1_024usize;

fn parse_args() -> Result<domain_types::AdminCommand, domain_types::BootstrapArgsError> {
    let mut args = std::env::args_os().skip(constants_usize::ONE);
    let login_arg = args.next().ok_or(domain_types::BootstrapArgsError::Usage)?;
    if login_arg == std::ffi::OsStr::new("reset") {
        let reset_login_arg = args.next().ok_or(domain_types::BootstrapArgsError::Usage)?;
        let password_file = args.next().ok_or(domain_types::BootstrapArgsError::Usage)?;
        if args.next().is_some() {
            return Err(domain_types::BootstrapArgsError::Usage);
        }
        let login = reset_login_arg.into_string().map_err(|value| {
            drop(value);
            domain_types::BootstrapArgsError::Login
        })?;
        return Ok(domain_types::AdminCommand::PasswordReset(
            domain_types::PasswordResetArgs::new(
                server_admin::AdminLogin::try_from(login).map_err(|error| {
                    let _error_text = format!("{error:?}");
                    domain_types::BootstrapArgsError::Login
                })?,
                domain_types::BootstrapPathBuf::from(std::path::PathBuf::from(password_file)),
            ),
        ));
    }
    let display_name_arg = args.next().ok_or(domain_types::BootstrapArgsError::Usage)?;
    let password_file = args.next().ok_or(domain_types::BootstrapArgsError::Usage)?;
    if args.next().is_some() {
        return Err(domain_types::BootstrapArgsError::Usage);
    }
    let login = login_arg.into_string().map_err(|value| {
        drop(value);
        domain_types::BootstrapArgsError::Login
    })?;
    let display_name = display_name_arg.into_string().map_err(|value| {
        drop(value);
        domain_types::BootstrapArgsError::DisplayName
    })?;
    Ok(domain_types::AdminCommand::Bootstrap(
        domain_types::BootstrapArgs::new(
            server_admin::AdminDisplayName::try_from(display_name).map_err(|error| {
                let _error_text = format!("{error:?}");
                domain_types::BootstrapArgsError::DisplayName
            })?,
            server_admin::AdminLogin::try_from(login).map_err(|error| {
                let _error_text = format!("{error:?}");
                domain_types::BootstrapArgsError::Login
            })?,
            domain_types::BootstrapPathBuf::from(std::path::PathBuf::from(password_file)),
        ),
    ))
}

fn password_from_file(
    password_file: &domain_types::BootstrapPathBuf,
) -> Result<server_admin_contract::AdminNewPassword, domain_types::BootstrapCommandError> {
    let bytes = server_runtime_http::read_bounded_file(
        password_file.as_path_ref(),
        server_runtime_http::BoundedReadMaximumBytes::from(PASSWORD_FILE_MAX_BYTES),
    )
    .map_err(domain_types::BootstrapCommandError::PasswordFile)?;
    password_from_bytes(bytes)
}

fn password_from_bytes(
    bytes: server_runtime_http::BoundedBytes,
) -> Result<server_admin_contract::AdminNewPassword, domain_types::BootstrapCommandError> {
    let text = server_runtime_http::BoundedText::try_from(bytes)
        .map_err(domain_types::BootstrapCommandError::PasswordFile)?;
    let mut password = text.into_inner();
    if password.ends_with('\n') {
        let _newline = password.pop();
        if password.ends_with('\r') {
            let _carriage_return = password.pop();
        }
    }
    server_admin_contract::AdminNewPassword::try_from(password).map_err(|error| {
        let _error_text = format!("{error:?}");
        domain_types::BootstrapCommandError::PasswordFileValue
    })
}

async fn run() -> Result<server_admin::AdminUserId, domain_types::BootstrapCommandError> {
    let command = parse_args().map_err(domain_types::BootstrapCommandError::Args)?;
    let config = server_config::domain_types::Config::try_from_env()
        .map_err(domain_types::BootstrapCommandError::Config)?;
    config
        .validate_for_startup()
        .map_err(domain_types::BootstrapCommandError::ConfigProduction)?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1u32)
        .connect(secrecy::ExposeSecret::expose_secret(
            config_lib::GetDatabaseUrl::get_database_url(&config),
        ))
        .await
        .map_err(|error| {
            domain_types::BootstrapCommandError::Connect(domain_types::SqlxBootstrapError::from(
                error,
            ))
        })?;
    server_admin::prep_pg(app_state::domain_types::SqlxPgPoolRef::from(&pool))
        .await
        .map_err(domain_types::BootstrapCommandError::Migrate)?;
    let concurrency = std::num::NonZeroUsize::new(config.admin_password_hash_concurrency.get())
        .ok_or(domain_types::BootstrapCommandError::PasswordFileValue)?;
    let password_hasher =
        server_admin::AdminPasswordHasher::new(server_admin::AdminPasswordHashConcurrency::from(
            server_admin::AdminNonZeroUsize::from(concurrency),
        ));
    match command {
        domain_types::AdminCommand::Bootstrap(args) => {
            let (display_name, login, password_file) = args.into_parts();
            let password = password_from_file(&password_file)?;
            server_admin::bootstrap_admin(
                app_state::domain_types::SqlxPgPoolRef::from(&pool),
                login,
                display_name,
                password,
                &password_hasher,
            )
            .await
            .map_err(domain_types::BootstrapCommandError::Bootstrap)
        }
        domain_types::AdminCommand::PasswordReset(args) => {
            let (login, password_file) = args.into_parts();
            let password = password_from_file(&password_file)?;
            server_admin::reset_admin_password(
                app_state::domain_types::SqlxPgPoolRef::from(&pool),
                login,
                password,
                &password_hasher,
            )
            .await
            .map_err(domain_types::BootstrapCommandError::PasswordReset)
        }
    }
}

#[allow(
    clippy::missing_const_for_fn,
    reason = "repository wrappers initialize through the non-const From trait"
)]
fn error_status(error: &domain_types::BootstrapCommandError) -> domain_types::BootstrapStatus {
    domain_types::BootstrapStatus::from(match error {
        domain_types::BootstrapCommandError::Args(_)
        | domain_types::BootstrapCommandError::PasswordFileValue => 2u8,
        domain_types::BootstrapCommandError::Bootstrap(
            server_admin::AdminBootstrapError::AlreadyInitialized,
        ) => 3u8,
        domain_types::BootstrapCommandError::Config(_)
        | domain_types::BootstrapCommandError::ConfigProduction(_)
        | domain_types::BootstrapCommandError::Connect(_)
        | domain_types::BootstrapCommandError::Migrate(_)
        | domain_types::BootstrapCommandError::PasswordFile(_)
        | domain_types::BootstrapCommandError::PasswordReset(_)
        | domain_types::BootstrapCommandError::Bootstrap(_) => 1u8,
    })
}

fn main() -> domain_types::BootstrapExitCode {
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            tracing::error!(error = %error, "failed to create administrator bootstrap runtime");
            return domain_types::BootstrapExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    match runtime.block_on(run()) {
        Ok(user_id) => {
            tracing::info!(user_id = %user_id, "administrator operation completed");
            domain_types::BootstrapExitCode::from(std::process::ExitCode::SUCCESS)
        }
        Err(error) => {
            tracing::error!(error = %error, "administrator operation failed");
            domain_types::BootstrapExitCode::from(std::process::ExitCode::from(u8::from(
                error_status(&error),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exit_codes_distinguish_invalid_input_and_completed_bootstrap() {
        assert_eq!(
            super::error_status(&super::domain_types::BootstrapCommandError::Args(
                super::domain_types::BootstrapArgsError::Usage,
            )),
            super::domain_types::BootstrapStatus::from(2u8)
        );
        assert_eq!(
            super::error_status(&super::domain_types::BootstrapCommandError::Bootstrap(
                server_admin::AdminBootstrapError::AlreadyInitialized,
            )),
            super::domain_types::BootstrapStatus::from(3u8)
        );
    }

    #[test]
    fn password_file_accepts_one_trailing_line_ending() {
        let password_text = constants_str::TEST_STRONG_PASSWORD;
        let password = super::password_from_bytes(server_runtime_http::BoundedBytes::from(
            format!("{password_text}\r\n").into_bytes(),
        ))
        .expect("05536bb6 password_file_accepts_one_trailing_line_ending invariant must hold");

        let debug = format!("{password:?}");
        assert!(debug.contains(constants_str::REDACTED_ALT_3));
        assert!(!debug.contains(password_text));
    }

    #[test]
    fn password_file_rejects_excess_bytes() {
        let Err(_error) =
            super::password_from_bytes(server_runtime_http::BoundedBytes::from(vec![
                b'a';
                super::PASSWORD_FILE_MAX_BYTES
                    .saturating_add(constants_usize::ONE)
            ]))
        else {
            panic!("7ad9edb5 expected an excessive password file to fail");
        };
    }
}
