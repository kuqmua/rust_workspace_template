#![allow(
    clippy::single_call_fn,
    reason = "the administrator account command keeps parsing, secret loading, and database orchestration isolated"
)]

const PASSWORD_FILE_MAX_BYTES: usize = 1_024usize;

fn parse_args()
-> Result<crate::domain_types::AdminCommand, crate::domain_types::AdministratorCommandArgsError> {
    let mut args = std::env::args_os().skip(constants_usize::ONE);
    let login_arg = args
        .next()
        .ok_or(crate::domain_types::AdministratorCommandArgsError::Usage)?;
    if login_arg == std::ffi::OsStr::new(constants_str::VALUE_01BE30BB) {
        let reset_login_arg = args
            .next()
            .ok_or(crate::domain_types::AdministratorCommandArgsError::Usage)?;
        let password_file = args
            .next()
            .ok_or(crate::domain_types::AdministratorCommandArgsError::Usage)?;
        if args.next().is_some() {
            return Err(crate::domain_types::AdministratorCommandArgsError::Usage);
        }
        let login = reset_login_arg.into_string().map_err(|value| {
            drop(value);
            crate::domain_types::AdministratorCommandArgsError::Login
        })?;
        return Ok(crate::domain_types::AdminCommand::PasswordReset(
            crate::domain_types::PasswordResetArgs::new(
                server_admin::domain_types::AdminLogin::try_from(login).map_err(|error| {
                    let _error_text = format!("{error:?}");
                    crate::domain_types::AdministratorCommandArgsError::Login
                })?,
                crate::domain_types::AdministratorPasswordFilePathBuf::from(
                    std::path::PathBuf::from(password_file),
                ),
            ),
        ));
    }
    let display_name_arg = args
        .next()
        .ok_or(crate::domain_types::AdministratorCommandArgsError::Usage)?;
    let password_file = args
        .next()
        .ok_or(crate::domain_types::AdministratorCommandArgsError::Usage)?;
    if args.next().is_some() {
        return Err(crate::domain_types::AdministratorCommandArgsError::Usage);
    }
    let login = login_arg.into_string().map_err(|value| {
        drop(value);
        crate::domain_types::AdministratorCommandArgsError::Login
    })?;
    let display_name = display_name_arg.into_string().map_err(|value| {
        drop(value);
        crate::domain_types::AdministratorCommandArgsError::DisplayName
    })?;
    Ok(
        crate::domain_types::AdminCommand::CreateInitialAdministrator(
            crate::domain_types::InitialAdministratorCreationArgs::new(
                server_admin::domain_types::AdminDisplayName::try_from(display_name).map_err(
                    |error| {
                        let _error_text = format!("{error:?}");
                        crate::domain_types::AdministratorCommandArgsError::DisplayName
                    },
                )?,
                server_admin::domain_types::AdminLogin::try_from(login).map_err(|error| {
                    let _error_text = format!("{error:?}");
                    crate::domain_types::AdministratorCommandArgsError::Login
                })?,
                crate::domain_types::AdministratorPasswordFilePathBuf::from(
                    std::path::PathBuf::from(password_file),
                ),
            ),
        ),
    )
}

fn password_from_file(
    password_file: &crate::domain_types::AdministratorPasswordFilePathBuf,
) -> Result<
    server_admin_contract::domain_types::AdminNewPassword,
    crate::domain_types::AdministratorAccountCommandError,
> {
    let bytes = server_runtime_http::domain_types::read_bounded_file(
        password_file.as_path_ref(),
        server_runtime_http::domain_types::BoundedReadMaximumBytes::from(PASSWORD_FILE_MAX_BYTES),
    )
    .map_err(crate::domain_types::AdministratorAccountCommandError::PasswordFile)?;
    password_from_bytes(bytes)
}

fn password_from_bytes(
    bytes: server_runtime_http::domain_types::BoundedBytes,
) -> Result<
    server_admin_contract::domain_types::AdminNewPassword,
    crate::domain_types::AdministratorAccountCommandError,
> {
    let text = server_runtime_http::domain_types::BoundedText::try_from(bytes)
        .map_err(crate::domain_types::AdministratorAccountCommandError::PasswordFile)?;
    let mut password = text.into_inner();
    if password.ends_with('\n') {
        let _newline = password.pop();
        if password.ends_with('\r') {
            let _carriage_return = password.pop();
        }
    }
    server_admin_contract::domain_types::AdminNewPassword::try_from(password).map_err(|error| {
        let _error_text = format!("{error:?}");
        crate::domain_types::AdministratorAccountCommandError::PasswordFileValue
    })
}

pub(crate) async fn run() -> Result<
    server_admin::domain_types::AdminUserId,
    crate::domain_types::AdministratorAccountCommandError,
> {
    let command =
        parse_args().map_err(crate::domain_types::AdministratorAccountCommandError::Args)?;
    let config = server_config::domain_types::Config::try_from_env()
        .map_err(crate::domain_types::AdministratorAccountCommandError::Config)?;
    config
        .validate_for_startup()
        .map_err(crate::domain_types::AdministratorAccountCommandError::ConfigProduction)?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1u32)
        .connect(secrecy::ExposeSecret::expose_secret(
            config_lib::domain_types::DatabaseUrlProvider::database_url(&config),
        ))
        .await
        .map_err(|error| {
            crate::domain_types::AdministratorAccountCommandError::Connect(
                crate::domain_types::SqlxAdministratorDatabaseConnectionError::from(error),
            )
        })?;
    server_admin::domain_types::prepare_postgresql(app_state::domain_types::SqlxPgPoolRef::from(
        &pool,
    ))
    .await
    .map_err(crate::domain_types::AdministratorAccountCommandError::Migrate)?;
    let concurrency = std::num::NonZeroUsize::new(config.admin_password_hash_concurrency.get())
        .ok_or(crate::domain_types::AdministratorAccountCommandError::PasswordFileValue)?;
    let password_hasher = server_admin::domain_types::AdminPasswordHasher::new(
        server_admin::domain_types::AdminPasswordHashConcurrency::from(
            server_admin::domain_types::AdminNonZeroUsize::from(concurrency),
        ),
    );
    match command {
        crate::domain_types::AdminCommand::CreateInitialAdministrator(args) => {
            let (display_name, login, password_file) = args.into_parts();
            let password = password_from_file(&password_file)?;
            server_admin::domain_types::create_initial_administrator(
                app_state::domain_types::SqlxPgPoolRef::from(&pool),
                login,
                display_name,
                password,
                &password_hasher,
            )
            .await
            .map_err(
                crate::domain_types::AdministratorAccountCommandError::InitialAdministratorCreation,
            )
        }
        crate::domain_types::AdminCommand::PasswordReset(args) => {
            let (login, password_file) = args.into_parts();
            let password = password_from_file(&password_file)?;
            server_admin::domain_types::reset_admin_password(
                app_state::domain_types::SqlxPgPoolRef::from(&pool),
                login,
                password,
                &password_hasher,
            )
            .await
            .map_err(crate::domain_types::AdministratorAccountCommandError::PasswordReset)
        }
    }
}

#[allow(
    clippy::missing_const_for_fn,
    reason = "repository wrappers initialize through the non-const From trait"
)]
pub(crate) fn error_status(
    error: &crate::domain_types::AdministratorAccountCommandError,
) -> crate::domain_types::AdministratorAccountCommandStatus {
    crate::domain_types::AdministratorAccountCommandStatus::from(match error {
        crate::domain_types::AdministratorAccountCommandError::Args(_)
        | crate::domain_types::AdministratorAccountCommandError::PasswordFileValue => 2u8,
        crate::domain_types::AdministratorAccountCommandError::InitialAdministratorCreation(
            server_admin::domain_types::InitialAdministratorCreationError::AlreadyInitialized,
        ) => 3u8,
        crate::domain_types::AdministratorAccountCommandError::Config(_)
        | crate::domain_types::AdministratorAccountCommandError::ConfigProduction(_)
        | crate::domain_types::AdministratorAccountCommandError::Connect(_)
        | crate::domain_types::AdministratorAccountCommandError::Migrate(_)
        | crate::domain_types::AdministratorAccountCommandError::PasswordFile(_)
        | crate::domain_types::AdministratorAccountCommandError::PasswordReset(_)
        | crate::domain_types::AdministratorAccountCommandError::InitialAdministratorCreation(_) => {
            1u8
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn exit_codes_distinguish_invalid_input_and_completed_initial_administrator_creation() {
        assert_eq!(
            super::error_status(
                &crate::domain_types::AdministratorAccountCommandError::Args(
                    crate::domain_types::AdministratorCommandArgsError::Usage,
                )
            ),
            crate::domain_types::AdministratorAccountCommandStatus::from(2u8)
        );
        assert_eq!(
            super::error_status(&crate::domain_types::AdministratorAccountCommandError::InitialAdministratorCreation(
                server_admin::domain_types::InitialAdministratorCreationError::AlreadyInitialized,
            )),
            crate::domain_types::AdministratorAccountCommandStatus::from(3u8)
        );
    }

    #[test]
    fn password_file_accepts_one_trailing_line_ending() {
        let password_text = constants_str::TEST_STRONG_PASSWORD;
        let password =
            super::password_from_bytes(server_runtime_http::domain_types::BoundedBytes::from(
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
            super::password_from_bytes(server_runtime_http::domain_types::BoundedBytes::from(
                vec![b'a'; super::PASSWORD_FILE_MAX_BYTES.saturating_add(constants_usize::ONE)],
            ))
        else {
            panic!("7ad9edb5 expected an excessive password file to fail");
        };
    }
}
