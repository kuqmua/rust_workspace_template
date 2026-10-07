#[cfg(test)]
mod tests {
    #[test]
    fn test_secret_box_string_preserves_byte_boundaries_and_redacts_debug() {
        let maximum = crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN;
        [constants_str::X, constants_str::NON_ASCII_U_E9]
            .into_iter()
            .fold((), |(), suffix| {
                let mut input = constants_str::X.repeat(maximum - suffix.len());
                input.push_str(suffix);
                let pointer = input.as_ptr();
                assert!(
                    crate::secrecy_secret_box_string::SecrecySecretBoxString::try_from(input)
                        .is_ok_and(|secret| {
                            let exposed =
                                secrecy::ExposeSecret::expose_secret(secret.as_ref()).as_ref();
                            exposed.len() == maximum
                                && exposed.as_ptr() == pointer
                                && exposed.ends_with(suffix)
                                && format!("{secret:?}") == constants_str::REDACTED_ALT_3
                                && format!("{secret:#?}") == constants_str::REDACTED_ALT_3
                        })
                );
                let mut oversized = constants_str::X.repeat(maximum);
                oversized.push_str(suffix);
                assert!(
                    crate::secrecy_secret_box_string::SecrecySecretBoxString::try_from(oversized)
                        .is_err_and(|error| error == crate::std_config_secret_string::StdConfigSecretStringTryFromStringError::TooLong { len: maximum + suffix.len(), max: maximum })
                );
            });
        [
            constants_str::EMPTY,
            constants_str::TEST_TEXT_WITH_NUL,
            constants_str::NON_ASCII_U_E9,
        ]
        .into_iter()
        .fold((), |(), text| {
            assert!(
                crate::secrecy_secret_box_string::SecrecySecretBoxString::try_from(text.to_owned())
                    .is_ok_and(|secret| {
                        secrecy::ExposeSecret::expose_secret(secret.as_ref()).as_ref() == text
                            && format!("{secret:?}") == constants_str::REDACTED_ALT_3
                            && format!("{secret:#?}") == constants_str::REDACTED_ALT_3
                    })
            );
        });
    }

    #[test]
    fn test_configuration_secret_zeroization_clears_text_idempotently() {
        [
            constants_str::EMPTY,
            constants_str::TEST_TEXT_WITH_NUL,
            constants_str::NON_ASCII_U_E9,
        ]
        .into_iter()
        .fold((), |(), text| {
            assert!(
                crate::std_config_secret_string::StdConfigSecretString::try_from(text.to_owned())
                    .is_ok_and(|mut secret| {
                        assert_eq!(secret.as_ref(), text);
                        secrecy::zeroize::Zeroize::zeroize(&mut secret);
                        assert!(secret.as_ref().is_empty());
                        secrecy::zeroize::Zeroize::zeroize(&mut secret);
                        secret.as_ref().is_empty()
                    })
            );
        });
    }

    #[test]
    #[cfg_attr(
        miri,
        ignore = "Miri interpretation is prohibitively slow when zeroizing the intentional oversized allocation"
    )]
    fn test_secret_box_string_rejects_values_above_shared_limit() {
        let value = constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(
            crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN
                .saturating_add(constants_usize::ONE),
        );
        let Err(_error) = crate::secrecy_secret_box_string::SecrecySecretBoxString::try_from(value)
        else {
            std::panic::panic_any(constants_str::PANIC_41C03FCC);
        };
    }

    #[test]
    fn test_parses_primary_and_verification_secrets() {
        let first = constants_str::TEST_JWT_SECRET_CHARACTER_A
            .repeat(crate::admin_jwt_secret_min_len::ADMIN_JWT_SECRET_MIN_LEN);
        let second = constants_str::TEST_JWT_SECRET_CHARACTER_B
            .repeat(crate::admin_jwt_secret_min_len::ADMIN_JWT_SECRET_MIN_LEN);
        let parsed = <crate::admin_jwt_secret::AdminJwtSecret as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(format!(" {first} , {second} "))
                .expect(constants_str::DIAGNOSTIC_12FD7C6A),
        )
        .expect(constants_str::DIAGNOSTIC_2C18577D);
        assert_eq!(parsed.verification_secrets().len(), 2usize);
        assert!(
            parsed
                .verification_secrets()
                .iter()
                .zip([first.as_str(), second.as_str()])
                .all(|(key, expected)| {
                    secrecy::ExposeSecret::expose_secret(key.as_ref()).as_ref() == expected
                })
        );
        assert!(
            parsed
                .primary()
                .zip(parsed.verification_secrets().first())
                .is_some_and(|(primary, first_key)| std::ptr::eq(primary, first_key))
        );
        assert_eq!(
            format!("{parsed:?}"),
            format!(
                "{}({:?})",
                constants_str::ADMINJWTSECRET,
                constants_str::REDACTED_ALT_3
            )
        );
        assert_eq!(
            format!("{parsed:#?}"),
            format!(
                "{}(\n    {:?},\n)",
                constants_str::ADMINJWTSECRET,
                constants_str::REDACTED_ALT_3
            )
        );
        assert_eq!(
            parsed
                .primary()
                .map(|secret| { secrecy::ExposeSecret::expose_secret(secret.as_ref()).as_ref() }),
            Some(&first)
        );
    }

    #[test]
    fn test_rejects_empty_effective_secret_list() {
        let result = <crate::admin_jwt_secret::AdminJwtSecret as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::TEST_EMPTY_DELIMITED_LIST))
                .expect(constants_str::DIAGNOSTIC_86C514B2),
        );
        assert!(matches!(
            result,
            Err(crate::try_from_std_env_var_ok_admin_jwt_secret_error::TryFromStdEnvVarOkAdminJwtSecretError::Empty)
        ));
    }

    #[test]
    fn test_rejects_empty_secret_between_rotation_keys() {
        let secret = constants_str::TEST_JWT_SECRET_CHARACTER_A
            .repeat(crate::admin_jwt_secret_min_len::ADMIN_JWT_SECRET_MIN_LEN);
        let result = <crate::admin_jwt_secret::AdminJwtSecret as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(format!("{secret},,{secret}"))
                .expect(constants_str::DIAGNOSTIC_9674829D),
        );
        assert!(matches!(
            result,
            Err(crate::try_from_std_env_var_ok_admin_jwt_secret_error::TryFromStdEnvVarOkAdminJwtSecretError::EmptyEntry)
        ));
    }

    #[test]
    fn test_jwt_secret_count_and_byte_boundaries_preserve_validation_order() {
        let parse = |std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk| {
            <crate::admin_jwt_secret::AdminJwtSecret as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok)
        };
        let minimum = crate::admin_jwt_secret_min_len::ADMIN_JWT_SECRET_MIN_LEN;
        let maximum = crate::admin_jwt_secret_max_count::ADMIN_JWT_SECRET_MAX_COUNT;
        let secret = constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(minimum);
        [1usize, maximum - 1usize, maximum]
            .into_iter()
            .fold((), |(), count| {
                let text = std::iter::repeat_n(secret.as_str(), count)
                    .collect::<Vec<_>>()
                    .join(constants_str::COMMA_SPACE);
                assert!(
                    crate::std_env_var_ok::StdEnvVarOk::try_from(text).is_ok_and(|value| parse(
                        value
                    )
                    .is_ok_and(|secrets| {
                        secrets.verification_secrets().len() == count
                            && secrets.verification_secrets().iter().all(|key| {
                                secrecy::ExposeSecret::expose_secret(key.as_ref()).as_ref()
                                    == secret.as_str()
                            })
                    }))
                );
            });
        [
            (secret.as_str(), crate::try_from_std_env_var_ok_admin_jwt_secret_error::TryFromStdEnvVarOkAdminJwtSecretError::TooMany),
            (constants_str::X, crate::try_from_std_env_var_ok_admin_jwt_secret_error::TryFromStdEnvVarOkAdminJwtSecretError::TooMany),
            (constants_str::EMPTY, crate::try_from_std_env_var_ok_admin_jwt_secret_error::TryFromStdEnvVarOkAdminJwtSecretError::Empty),
        ]
        .into_iter()
        .fold((), |(), (entry, expected)| {
            let text = std::iter::repeat_n(entry, maximum + 1usize)
                .collect::<Vec<_>>()
                .join(constants_str::COMMA_SPACE);
            assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(text)
                .is_ok_and(|value| parse(value).is_err_and(|error| error == expected)));
        });
        assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(
            constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(minimum - 1usize)
        )
        .is_ok_and(|value| parse(value).is_err_and(|error| error
            == crate::try_from_std_env_var_ok_admin_jwt_secret_error::TryFromStdEnvVarOkAdminJwtSecretError::TooShort)));
        let unicode_secret = '\u{00e9}'.to_string().repeat(16usize);
        assert!(
            crate::std_env_var_ok::StdEnvVarOk::try_from(unicode_secret).is_ok_and(|value| parse(
                value
            )
            .is_ok_and(|secrets| {
                secrets.verification_secrets().len() == 1usize
                    && secrets.primary().is_some_and(|key| {
                        secrecy::ExposeSecret::expose_secret(key.as_ref())
                            .as_ref()
                            .len()
                            == minimum
                    })
            }))
        );
    }
}
