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
    fn test_password_file_reads_owned_fixtures_and_preserves_validation_boundaries() {
        let directory = std::env::temp_dir().join(format!(
            "{}-{}",
            stringify!(test_password_file_reads_owned_fixtures_and_preserves_validation_boundaries),
            std::process::id(),
        ));
        assert!(matches!(
            std::fs::DirBuilder::new().create(&directory),
            Ok(())
        ));
        let path = directory.join(constants_str::X);
        let administrator_password_file_path_buf =
            crate::administrator_password_file_path_buf::AdministratorPasswordFilePathBuf::from(
                path.clone(),
            );
        let outcome = (|| {
            let endings = [
                (String::new(), true),
                ('\n'.to_string(), true),
                (['\r', '\n'].into_iter().collect::<String>(), true),
                ('\r'.to_string(), false),
                ('\n'.to_string().repeat(2usize), false),
                (
                    ['\r', '\n'].into_iter().collect::<String>().repeat(2usize),
                    false,
                ),
                (constants_str::SPACE.to_owned(), false),
            ];
            if !endings.into_iter().all(|(ending, valid)| {
                let text = format!("{}{ending}", constants_str::TEST_STRONG_PASSWORD);
                if std::fs::write(&path, text.as_bytes()).is_err() {
                    return false;
                }
                let result = super::password_from_file(&administrator_password_file_path_buf);
                if valid {
                    result.is_ok_and(|password| {
                        let debug = format!("{password:?}");
                        password.as_ref() == constants_str::TEST_STRONG_PASSWORD
                            && debug.contains(constants_str::REDACTED_ALT_3)
                            && !debug.contains(constants_str::TEST_STRONG_PASSWORD)
                    })
                } else {
                    matches!(result, Err(crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFileValue))
                }
            }) {
                return false;
            }
            let malformed = [vec![255u8], vec![0xc3u8], vec![0xc3u8, b'(']];
            if !malformed.into_iter().all(|bytes| {
                std::fs::write(&path, bytes).is_ok()
                    && matches!(super::password_from_file(&administrator_password_file_path_buf),
                        Err(crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFile(server_runtime_http::bounded_read_error::BoundedReadError::Utf8 { .. })))
            }) {
                return false;
            }
            [0usize, super::PASSWORD_FILE_MAX_BYTES, super::PASSWORD_FILE_MAX_BYTES + 1usize]
                .into_iter().all(|length| {
                    if std::fs::write(&path, vec![b'a'; length]).is_err() {
                        return false;
                    }
                    let result = super::password_from_file(&administrator_password_file_path_buf);
                    if length <= super::PASSWORD_FILE_MAX_BYTES {
                        matches!(result, Err(crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFileValue))
                    } else {
                        matches!(result, Err(crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFile(server_runtime_http::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes })) if maximum_bytes == server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(super::PASSWORD_FILE_MAX_BYTES))
                    }
                })
        })();
        assert!(matches!(std::fs::remove_dir_all(&directory), Ok(())));
        assert!(outcome);
    }

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
