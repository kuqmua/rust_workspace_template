pub(crate) fn password_from_bytes(
    bytes: server_runtime_http::domain_types::BoundedBytes,
) -> Result<
    server_admin_contract::domain_types::AdminNewPassword,
    crate::AdministratorAccountCommandError,
> {
    let text = server_runtime_http::domain_types::BoundedText::try_from(bytes)
        .map_err(crate::AdministratorAccountCommandError::PasswordFile)?;
    let mut password = text.into_inner();
    if password.ends_with('\n') {
        let _newline = password.pop();
        if password.ends_with('\r') {
            let _carriage_return = password.pop();
        }
    }
    server_admin_contract::domain_types::AdminNewPassword::try_from(password).map_err(|error| {
        let _error_text = format!("{error:?}");
        crate::AdministratorAccountCommandError::PasswordFileValue
    })
}
