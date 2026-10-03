pub(crate) fn password_from_bytes(
    bounded_bytes: server_runtime_http::bounded_bytes::BoundedBytes,
) -> Result<
    server_admin_contract::admin_new_password::AdminNewPassword,
    crate::administrator_account_command_error::AdministratorAccountCommandError,
> {
    let text = server_runtime_http::bounded_text::BoundedText::try_from(bounded_bytes).map_err(
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
    fn test_password_bytes_remove_only_one_complete_line_ending() {
        assert!([
            (String::default(), true),
            ('\n'.to_string(), true),
            (['\r', '\n'].into_iter().collect::<String>(), true),
            ('\r'.to_string(), false),
            ('\n'.to_string().repeat(2usize), false),
            (['\r', '\n'].into_iter().collect::<String>().repeat(2usize), false),
            (constants_str::SPACE.to_owned(), false),
        ].into_iter().all(|(suffix, valid)| {
            let text = format!("{}{suffix}", constants_str::TEST_STRONG_PASSWORD);
            let result = crate::password_from_bytes::password_from_bytes(server_runtime_http::bounded_bytes::BoundedBytes::from(text.into_bytes()));
            if valid {
                result.is_ok_and(|password| password.as_ref() == constants_str::TEST_STRONG_PASSWORD)
            } else {
                matches!(result, Err(crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFileValue))
            }
        }));
    }

    #[test]
    fn test_password_bytes_distinguish_utf8_and_password_policy_errors() {
        assert!([vec![255u8], vec![0xc3u8], vec![0xc3u8, b'(']].into_iter().all(|bytes| {
            matches!(crate::password_from_bytes::password_from_bytes(server_runtime_http::bounded_bytes::BoundedBytes::from(bytes)),
                Err(crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFile(server_runtime_http::bounded_read_error::BoundedReadError::Utf8 { .. })))
        }));
        assert!([constants_str::PG_CRUD_EMPTY_SQL_SUFFIX, constants_str::X].into_iter().all(|text| {
            matches!(crate::password_from_bytes::password_from_bytes(server_runtime_http::bounded_bytes::BoundedBytes::from(text.as_bytes().to_vec())),
                Err(crate::administrator_account_command_error::AdministratorAccountCommandError::PasswordFileValue))
        }));
    }
}
