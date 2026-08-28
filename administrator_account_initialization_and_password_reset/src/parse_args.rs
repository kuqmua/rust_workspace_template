#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(crate) fn parse_args()
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
