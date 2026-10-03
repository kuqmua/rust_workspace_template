const PASSWORD_FILE_MAX_BYTES: usize = 1_024usize;

pub(crate) fn password_from_file(
    administrator_password_file_path_buf: &crate::administrator_password_file_path_buf::AdministratorPasswordFilePathBuf,
) -> Result<
    server_admin_contract::admin_new_password::AdminNewPassword,
    crate::administrator_account_command_error::AdministratorAccountCommandError,
> {
    let bytes = server_runtime_http::read_bounded_file::read_bounded_file(
        administrator_password_file_path_buf.as_path_ref(),
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
    fn test_password_file_preserves_io_and_read_limit_errors() {
        let crate_directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!([
            crate_directory.join(constants_str::SRC),
            crate_directory.join(constants_str::CARGO_TOML).join(constants_str::X),
        ].into_iter().all(|path| {
            let administrator_password_file_path_buf = crate::administrator_password_file_path_buf::AdministratorPasswordFilePathBuf::from(path);
            matches!(crate::password_from_file::password_from_file(&administrator_password_file_path_buf),
                Err(crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFile(server_runtime_http::bounded_read_error::BoundedReadError::Io { .. })))
        }));
        let administrator_password_file_path_buf =
            crate::administrator_password_file_path_buf::AdministratorPasswordFilePathBuf::from(
                crate_directory
                    .join(constants_str::PARENT_PATH_SEGMENT)
                    .join(constants_str::CARGO_TOML),
            );
        assert!(
            matches!(crate::password_from_file::password_from_file(&administrator_password_file_path_buf),
            Err(crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFile(server_runtime_http::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes })) if maximum_bytes == server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(1024usize))
        );
    }

    #[test]
    fn test_password_file_accepts_one_trailing_line_ending() {
        let password_text = constants_str::TEST_STRONG_PASSWORD;
        let password = crate::password_from_bytes::password_from_bytes(
            server_runtime_http::bounded_bytes::BoundedBytes::from(
                format!("{password_text}\r\n").into_bytes(),
            ),
        )
        .expect(constants_str::DIAGNOSTIC_05536BB6);

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
            std::panic::panic_any(constants_str::PANIC_7AD9EDB5);
        };
    }
}
