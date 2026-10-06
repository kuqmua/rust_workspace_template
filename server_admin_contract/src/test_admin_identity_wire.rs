#[test]
fn test_administrator_identity_wire_validators_preserve_lengths_and_character_policies() {
    fn administrator_identity_wire_matches<
        Identity,
        const MINIMUM: usize,
        const MAXIMUM: usize,
        const ASCII_ONLY: bool,
    >() -> crate::admin_bool::AdminBool
    where
        Identity: serde::de::DeserializeOwned + serde::Serialize,
    {
        let matches = |(raw_value, accepted)| {
            serde_json::to_string(&raw_value).is_ok_and(|serialized_text| {
                let result = serde_json::from_str::<Identity>(&serialized_text);
                if accepted {
                    result.is_ok_and(|identity| {
                        serde_json::to_value(identity)
                            .is_ok_and(|value| value.as_str().is_some_and(|text| text == raw_value))
                    })
                } else {
                    result.is_err()
                }
            })
        };
        let lengths_and_whitespace = [
            (constants_str::X.repeat(MINIMUM), true),
            (constants_str::X.repeat(MAXIMUM), true),
            (
                constants_str::X.repeat(MINIMUM.saturating_sub(1usize)),
                false,
            ),
            (
                constants_str::X.repeat(MAXIMUM.saturating_add(1usize)),
                false,
            ),
            (constants_str::ADMIN.to_owned(), !ASCII_ONLY),
            (constants_str::U_1F496.repeat(MAXIMUM), !ASCII_ONLY),
            ([constants_str::ROOT, constants_str::SPACE].concat(), false),
            ([constants_str::SPACE, constants_str::ROOT].concat(), false),
            (
                [constants_str::ROOT, constants_str::SPACE, constants_str::X].concat(),
                !ASCII_ONLY,
            ),
        ]
        .into_iter()
        .all(matches);
        let character_classes = [
            ('_', true),
            ('.', true),
            ('-', true),
            ('1', true),
            (':', !ASCII_ONLY),
            ('/', !ASCII_ONLY),
            ('\\', !ASCII_ONLY),
            ('\u{0430}', !ASCII_ONLY),
        ]
        .into_iter()
        .all(|(character, accepted)| {
            matches((
                constants_str::ROOT
                    .chars()
                    .chain(std::iter::once(character))
                    .collect::<String>(),
                accepted,
            ))
        });
        let rejected_types = [
            serde_json::json!(null),
            serde_json::json!(false),
            serde_json::json!(1u8),
            serde_json::json!([]),
            serde_json::json!({}),
        ]
        .into_iter()
        .all(|value| serde_json::from_value::<Identity>(value).is_err());
        crate::admin_bool::AdminBool::from(
            lengths_and_whitespace && character_classes && rejected_types,
        )
    }
    assert!(
        [
            administrator_identity_wire_matches::<
                crate::admin_display_name::AdminDisplayName,
                { crate::identity::ADMIN_DISPLAY_NAME_MIN_CHARS },
                { crate::identity::ADMIN_DISPLAY_NAME_MAX_CHARS },
                false,
            >(),
            administrator_identity_wire_matches::<
                crate::admin_login::AdminLogin,
                { crate::identity::ADMIN_LOGIN_MIN_CHARS },
                { crate::identity::ADMIN_LOGIN_MAX_CHARS },
                true,
            >(),
            administrator_identity_wire_matches::<
                crate::admin_role_name::AdminRoleName,
                { crate::identity::ADMIN_ROLE_NAME_MIN_CHARS },
                { crate::identity::ADMIN_ROLE_NAME_MAX_CHARS },
                true,
            >(),
        ]
        .into_iter()
        .all(bool::from)
    );
}

#[test]
fn test_current_password_wire_bounds_preserve_secret_text_and_redaction() {
    let maximum = crate::identity::ADMIN_PASSWORD_MAX_CHARS;
    assert!(
        [
            constants_str::X.to_owned(),
            constants_str::SPACE.to_owned(),
            constants_str::U_1F496.repeat(maximum),
        ]
        .into_iter()
        .all(|raw_value| {
            serde_json::to_string(&raw_value).is_ok_and(|serialized_text| {
                serde_json::from_str::<crate::admin_password::AdminPassword>(&serialized_text)
                    .is_ok_and(|password| {
                        let debug = format!("{password:?}");
                        password.as_ref().as_str() == raw_value
                            && debug.contains(constants_str::REDACTED_ALT_3)
                            && !debug.contains(&raw_value)
                            && serde_json::to_value(password).is_ok_and(|value| {
                                value.as_str().is_some_and(|text| text == raw_value)
                            })
                    })
            })
        })
    );
    assert!(
        [
            constants_str::EMPTY.to_owned(),
            constants_str::X.repeat(maximum.saturating_add(1usize)),
            constants_str::U_1F496.repeat(maximum.saturating_add(1usize)),
        ]
        .into_iter()
        .all(|raw_value| {
            serde_json::to_string(&raw_value).is_ok_and(|serialized_text| {
                serde_json::from_str::<crate::admin_password::AdminPassword>(&serialized_text)
                    .is_err()
            })
        })
    );
}

#[test]
fn test_new_password_wire_preserves_unicode_maximum_and_rejects_invalid_policy() {
    let minimum = crate::identity::ADMIN_NEW_PASSWORD_MIN_CHARS;
    let maximum = crate::identity::ADMIN_PASSWORD_MAX_CHARS;
    assert_eq!(constants_str::TEST_STRONG_PASSWORD.chars().count(), minimum);
    let padding = maximum.saturating_sub(minimum);
    assert!(
        [
            constants_str::TEST_STRONG_PASSWORD.to_owned(),
            [
                constants_str::TEST_STRONG_PASSWORD,
                constants_str::X.repeat(padding).as_str()
            ]
            .concat(),
            [
                constants_str::TEST_STRONG_PASSWORD,
                constants_str::U_1F496.repeat(padding).as_str()
            ]
            .concat(),
        ]
        .into_iter()
        .all(|raw_value| {
            serde_json::to_string(&raw_value).is_ok_and(|serialized_text| {
                serde_json::from_str::<crate::admin_new_password::AdminNewPassword>(
                    &serialized_text,
                )
                .is_ok_and(|password| {
                    let debug = format!("{password:?}");
                    password.as_ref().as_str() == raw_value
                        && debug.contains(constants_str::REDACTED_ALT_3)
                        && !debug.contains(constants_str::TEST_STRONG_PASSWORD)
                        && serde_json::to_value(password)
                            .is_ok_and(|value| value.as_str().is_some_and(|text| text == raw_value))
                })
            })
        })
    );
    assert!(
        [
            constants_str::EMPTY.to_owned(),
            constants_str::X.repeat(minimum),
            constants_str::SPACE.repeat(minimum),
            [constants_str::TEST_STRONG_PASSWORD, constants_str::SPACE].concat(),
            [
                constants_str::TEST_STRONG_PASSWORD,
                constants_str::X
                    .repeat(padding.saturating_add(1usize))
                    .as_str()
            ]
            .concat(),
            [
                constants_str::TEST_STRONG_PASSWORD,
                constants_str::U_1F496
                    .repeat(padding.saturating_add(1usize))
                    .as_str()
            ]
            .concat(),
        ]
        .into_iter()
        .all(|raw_value| {
            serde_json::to_string(&raw_value).is_ok_and(|serialized_text| {
                serde_json::from_str::<crate::admin_new_password::AdminNewPassword>(
                    &serialized_text,
                )
                .is_err()
            })
        })
    );
}
