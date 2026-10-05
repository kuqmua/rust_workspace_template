#[test]
fn test_admin_string_error_conversion_preserves_validation_category() {
    let cases = [
        (
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::InvalidBounds {
                min: 2usize,
                max: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::InvalidBounds,
        ),
        (
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::TooShort {
                len: 0usize,
                min: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::TooShort,
        ),
        (
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::TooLong {
                len: 2usize,
                max: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::TooLong,
        ),
        (
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::ContainsNul,
            crate::admin_secret_text_error::AdminSecretTextError::ContainsNul,
        ),
        (
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::InvalidValue,
            crate::admin_secret_text_error::AdminSecretTextError::InvalidValue,
        ),
    ];
    assert!(cases.into_iter().all(|(source, expected)| {
        crate::admin_secret_text_error::AdminSecretTextError::from(source) == expected
    }));
}

#[test]
fn test_admin_cookie_error_conversion_preserves_validation_category() {
    let cases = [
        (
            crate::std_admin_cookie::StdAdminCookieTryFromStringError::InvalidBounds {
                min: 2usize,
                max: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::InvalidBounds,
        ),
        (
            crate::std_admin_cookie::StdAdminCookieTryFromStringError::TooShort {
                len: 0usize,
                min: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::TooShort,
        ),
        (
            crate::std_admin_cookie::StdAdminCookieTryFromStringError::TooLong {
                len: 2usize,
                max: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::TooLong,
        ),
        (
            crate::std_admin_cookie::StdAdminCookieTryFromStringError::ContainsNul,
            crate::admin_secret_text_error::AdminSecretTextError::ContainsNul,
        ),
        (
            crate::std_admin_cookie::StdAdminCookieTryFromStringError::InvalidValue,
            crate::admin_secret_text_error::AdminSecretTextError::InvalidValue,
        ),
    ];
    assert!(cases.into_iter().all(|(source, expected)| {
        crate::admin_secret_text_error::AdminSecretTextError::from(source) == expected
    }));
}

#[test]
fn test_admin_access_token_error_conversion_preserves_validation_category() {
    let cases = [
        (
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::InvalidBounds {
                min: 2usize,
                max: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::InvalidBounds,
        ),
        (
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::TooShort {
                len: 0usize,
                min: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::TooShort,
        ),
        (
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::TooLong {
                len: 2usize,
                max: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::TooLong,
        ),
        (
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::ContainsNul,
            crate::admin_secret_text_error::AdminSecretTextError::ContainsNul,
        ),
        (
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::InvalidValue,
            crate::admin_secret_text_error::AdminSecretTextError::InvalidValue,
        ),
    ];
    assert!(cases.into_iter().all(|(source, expected)| {
        crate::admin_secret_text_error::AdminSecretTextError::from(source) == expected
    }));
}
#[test]
fn test_sign_in_password_contract_adapter_preserves_text_and_runtime_redaction() {
    assert!(
        [
            constants_str::SLASH.repeat(server_admin_contract::identity::ADMIN_PASSWORD_MIN_CHARS),
            constants_str::SLASH.repeat(server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS),
            constants_str::TEST_TEXT_WITH_NUL.to_owned(),
            std::iter::repeat_n(
                char::MAX,
                server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS
            )
            .collect::<String>(),
        ]
        .into_iter()
        .all(|secret_text| {
            server_admin_contract::admin_password::AdminPassword::try_from(secret_text.clone())
                .is_ok_and(|contract_password| {
                    crate::admin_password_from_contract::admin_password_from_contract(
                        contract_password,
                    )
                    .is_ok_and(|runtime_password| {
                        secrecy::ExposeSecret::expose_secret(runtime_password.get_inner()).as_ref()
                            == &secret_text
                            && format!("{runtime_password:?}")
                                .contains(constants_str::REDACTED_ALT_3)
                            && !format!("{runtime_password:?}").contains(&secret_text)
                    })
                })
        })
    );
}

#[test]
fn test_new_password_contract_adapter_preserves_policy_boundaries_and_runtime_redaction() {
    assert!(
        [
            server_admin_contract::identity::ADMIN_NEW_PASSWORD_MIN_CHARS,
            server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS,
        ]
        .into_iter()
        .all(|length| {
            let secret_text = ['A', 'a', '1', '!']
                .into_iter()
                .chain(std::iter::repeat_n('x', length - 4usize))
                .collect::<String>();
            server_admin_contract::admin_new_password::AdminNewPassword::try_from(
                secret_text.clone(),
            )
            .is_ok_and(|contract_password| {
                crate::admin_new_password_from_contract::admin_new_password_from_contract(
                    contract_password,
                )
                .is_ok_and(|runtime_password| {
                    secrecy::ExposeSecret::expose_secret(runtime_password.get_inner()).as_ref()
                        == &secret_text
                        && format!("{runtime_password:?}").contains(constants_str::REDACTED_ALT_3)
                        && !format!("{runtime_password:?}").contains(&secret_text)
                })
            })
        })
    );
}
#[test]
fn test_runtime_password_deserialization_rejects_invalid_lengths_and_non_string_inputs() {
    assert!(
        [
            constants_str::EMPTY.to_owned(),
            constants_str::SLASH.repeat(server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS + 1usize),
            std::iter::repeat_n(char::MAX, server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS + 1usize)
                .collect::<String>(),
        ]
        .into_iter()
        .all(|secret_text| {
            matches!(
                crate::runtime_admin_password::RuntimeAdminPassword::try_from(secret_text.clone()),
                Err(crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError::InvalidLength)
            ) && serde_json::from_value::<crate::runtime_admin_password::RuntimeAdminPassword>(
                serde_json::Value::String(secret_text),
            ).is_err()
        })
    );
    assert!(
        [
            serde_json::Value::Null,
            serde_json::Value::Bool(true),
            serde_json::Value::from(1i64)
        ]
        .into_iter()
        .all(|payload| serde_json::from_value::<
            crate::runtime_admin_password::RuntimeAdminPassword,
        >(payload)
        .is_err())
    );
}

#[test]
fn test_runtime_password_deserialization_preserves_valid_secret_boundaries() {
    assert!(
        [
            constants_str::SLASH.repeat(server_admin_contract::identity::ADMIN_PASSWORD_MIN_CHARS),
            constants_str::SLASH.repeat(server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS),
            constants_str::TEST_TEXT_WITH_NUL.to_owned(),
            std::iter::repeat_n(
                char::MAX,
                server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS
            )
            .collect::<String>(),
        ]
        .into_iter()
        .all(|secret_text| {
            serde_json::from_value::<crate::runtime_admin_password::RuntimeAdminPassword>(
                serde_json::Value::String(secret_text.clone()),
            )
            .is_ok_and(|runtime_password| {
                secrecy::ExposeSecret::expose_secret(runtime_password.get_inner()).as_ref()
                    == &secret_text
            })
        })
    );
}

#[test]
fn test_runtime_password_open_api_schema_preserves_write_only_string_bounds_and_name() {
    assert_eq!(
        <crate::runtime_admin_password::RuntimeAdminPassword as utoipa::ToSchema>::name(),
        constants_str::ADMINPASSWORD
    );
    assert!(matches!(
        <crate::runtime_admin_password::RuntimeAdminPassword as utoipa::PartialSchema>::schema(),
        utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(object))
            if object.schema_type == utoipa::openapi::schema::SchemaType::Type(utoipa::openapi::schema::Type::String)
                && object.min_length == Some(server_admin_contract::identity::ADMIN_PASSWORD_MIN_CHARS)
                && object.max_length == Some(server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS)
                && object.write_only == Some(true)
    ));
}
