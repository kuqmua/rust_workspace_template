#![allow(
    unused_crate_dependencies,
    reason = "the binary integration target uses Tokio and constants while the CLI production dependencies belong to the separate executable"
)]

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_administrator_cli_invalid_configuration_returns_failure_exit_code() {
        let make_command = || {
            let mut command = tokio::process::Command::new(env!(
                "CARGO_BIN_EXE_administrator_account_initialization_and_password_reset"
            ));
            let _invalid_configuration = command.env(
                constants_str::ENV_NAMES_SERVICE_SOCKET_ADDRESS,
                constants_str::X,
            );
            command
        };
        let mut creation_command = make_command();
        let _creation_arguments =
            creation_command.args([constants_str::LOCALHOST, constants_str::X, constants_str::X]);
        let mut reset_command = make_command();
        let _reset_arguments = reset_command.args([
            constants_str::VALUE_01BE30BB,
            constants_str::LOCALHOST,
            constants_str::X,
        ]);
        let (creation, reset) = tokio::join!(creation_command.output(), reset_command.output());
        assert!(creation.is_ok_and(|output| output.status.code() == Some(1i32)));
        assert!(reset.is_ok_and(|output| output.status.code() == Some(1i32)));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_administrator_cli_rejects_non_utf8_identity_arguments() {
        let make_command = || {
            tokio::process::Command::new(env!(
                "CARGO_BIN_EXE_administrator_account_initialization_and_password_reset"
            ))
        };
        let invalid =
            <std::ffi::OsString as std::os::unix::ffi::OsStringExt>::from_vec(vec![255u8]);
        let mut creation_login = make_command();
        let _creation_login_arguments = creation_login
            .arg(&invalid)
            .args([constants_str::X, constants_str::X]);
        let mut creation_display = make_command();
        let _creation_display_arguments = creation_display
            .arg(constants_str::LOCALHOST)
            .arg(&invalid)
            .arg(constants_str::X);
        let mut reset_login = make_command();
        let _reset_login_arguments = reset_login
            .arg(constants_str::VALUE_01BE30BB)
            .arg(&invalid)
            .arg(constants_str::X);
        let (login, display, reset) = tokio::join!(
            creation_login.output(),
            creation_display.output(),
            reset_login.output()
        );
        assert!(login.is_ok_and(|output| output.status.code() == Some(2i32)));
        assert!(display.is_ok_and(|output| output.status.code() == Some(2i32)));
        assert!(reset.is_ok_and(|output| output.status.code() == Some(2i32)));
    }

    #[tokio::test]
    async fn test_administrator_cli_rejects_invalid_identity_and_extra_arguments() {
        let make_command = || {
            tokio::process::Command::new(env!(
                "CARGO_BIN_EXE_administrator_account_initialization_and_password_reset"
            ))
        };
        let invalid_login = constants_str::LOCALHOST.to_ascii_uppercase();
        let mut creation_login = make_command();
        let _creation_login_arguments =
            creation_login.args([invalid_login.as_str(), constants_str::X, constants_str::X]);
        let mut creation_display = make_command();
        let _creation_display_arguments = creation_display.args([
            constants_str::LOCALHOST,
            constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            constants_str::X,
        ]);
        let mut creation_extra = make_command();
        let _creation_extra_arguments = creation_extra.args([
            constants_str::LOCALHOST,
            constants_str::X,
            constants_str::X,
            constants_str::X,
        ]);
        let mut reset_login = make_command();
        let _reset_login_arguments = reset_login.args([
            constants_str::VALUE_01BE30BB,
            invalid_login.as_str(),
            constants_str::X,
        ]);
        let mut reset_extra = make_command();
        let _reset_extra_arguments = reset_extra.args([
            constants_str::VALUE_01BE30BB,
            constants_str::LOCALHOST,
            constants_str::X,
            constants_str::X,
        ]);
        let (login, display, extra, reset_identity, reset_arguments) = tokio::join!(
            creation_login.output(),
            creation_display.output(),
            creation_extra.output(),
            reset_login.output(),
            reset_extra.output(),
        );
        assert!(login.is_ok_and(|output| output.status.code() == Some(2i32)));
        assert!(display.is_ok_and(|output| output.status.code() == Some(2i32)));
        assert!(extra.is_ok_and(|output| output.status.code() == Some(2i32)));
        assert!(reset_identity.is_ok_and(|output| output.status.code() == Some(2i32)));
        assert!(reset_arguments.is_ok_and(|output| output.status.code() == Some(2i32)));
    }

    #[tokio::test]
    async fn test_administrator_cli_missing_arguments_return_usage_exit_code() {
        let make_command = || {
            tokio::process::Command::new(env!(
                "CARGO_BIN_EXE_administrator_account_initialization_and_password_reset"
            ))
        };
        let mut missing_creation_arguments = make_command();
        let mut missing_reset_arguments = make_command();
        let _configured_command = missing_reset_arguments.arg(constants_str::VALUE_01BE30BB);
        let (creation, reset) = tokio::join!(
            missing_creation_arguments.output(),
            missing_reset_arguments.output(),
        );
        assert!(creation.is_ok_and(|output| output.status.code() == Some(2i32)));
        assert!(reset.is_ok_and(|output| output.status.code() == Some(2i32)));
    }
}
