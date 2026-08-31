const PASSWORD_FILE_MAX_BYTES: usize = 1_024usize;

pub(crate) fn password_from_file(
    password_file: &crate::administrator_password_file_path_buf::AdministratorPasswordFilePathBuf,
) -> Result<
    server_admin_contract::admin_new_password::AdminNewPassword,
    crate::administrator_account_command_error::AdministratorAccountCommandError,
> {
    let bytes = server_runtime_http::read_bounded_file::read_bounded_file(
        password_file.as_path_ref(),
        server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
            PASSWORD_FILE_MAX_BYTES,
        ),
    )
    .map_err(
        crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFile,
    )?;
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_password_file_accepts_one_trailing_line_ending() {
        let password_text = constants_str::TEST_STRONG_PASSWORD;
        let password = crate::password_from_bytes::password_from_bytes(
            server_runtime_http::bounded_bytes::BoundedBytes::from(
                format!("{password_text}\r\n").into_bytes(),
            ),
        )
        .expect("05536bb6 password_file_accepts_one_trailing_line_ending invariant must hold");

        let debug = format!("{password:?}");
        assert!(debug.contains(constants_str::REDACTED_ALT_3));
        assert!(!debug.contains(password_text));
    }

    #[test]
    fn test_password_file_rejects_excess_bytes() {
        let Err(_error) = crate::password_from_bytes::password_from_bytes(
            server_runtime_http::bounded_bytes::BoundedBytes::from(vec![
                b'a';
                super::PASSWORD_FILE_MAX_BYTES
                    .saturating_add(constants_usize::ONE)
            ]),
        ) else {
            panic!("7ad9edb5 expected an excessive password file to fail");
        };
    }
}
