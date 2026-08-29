pub(crate) fn password_from_bytes(
    bytes: server_runtime_http::bounded_bytes::BoundedBytes,
) -> Result<
    server_admin_contract::admin_new_password::AdminNewPassword,
    crate::administrator_account_command_error::AdministratorAccountCommandError,
> {
    let text = server_runtime_http::bounded_text::BoundedText::try_from(bytes).map_err(
        crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFile,
    )?;
    let mut password = text.into_inner();
    if password.ends_with('\n') {
        let _newline = password.pop();
        if password.ends_with('\r') {
            let _carriage_return = password.pop();
        }
    }
    server_admin_contract::admin_new_password::AdminNewPassword::try_from(password).map_err(
        |error| {
            let _error_text = format!("{error:?}");
            crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFileValue
        },
    )
}
