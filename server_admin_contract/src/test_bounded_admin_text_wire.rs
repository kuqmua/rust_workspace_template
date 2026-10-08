#[test]
fn test_bounded_administrator_text_deserialization_preserves_unicode_limits_and_wire_types() {
    fn administrator_text_wire_matches<Text, const MAXIMUM: usize>() -> crate::admin_bool::AdminBool
    where
        Text: serde::de::DeserializeOwned + serde::Serialize,
    {
        let accepted = [
            constants_str::EMPTY.to_owned(),
            constants_str::X.to_owned(),
            constants_str::THREE_SPACES.to_owned(),
            constants_str::U_1F496.repeat(MAXIMUM),
        ]
        .into_iter()
        .all(|raw_value| {
            serde_json::to_string(&raw_value).is_ok_and(|serialized_text| {
                serde_json::from_str::<Text>(&serialized_text).is_ok_and(|text| {
                    serde_json::to_value(text).is_ok_and(|serialized_value| {
                        serialized_value
                            .as_str()
                            .is_some_and(|value| value == raw_value)
                    })
                })
            })
        });
        let rejected_lengths =
            [constants_str::X, constants_str::U_1F496]
                .into_iter()
                .all(|value| {
                    let raw_value = value.repeat(MAXIMUM.saturating_add(1usize));
                    serde_json::to_string(&raw_value).is_ok_and(|serialized_text| {
                        serde_json::from_str::<Text>(&serialized_text).is_err()
                    })
                });
        let rejected_types = [
            serde_json::json!(null),
            serde_json::json!(false),
            serde_json::json!(1u8),
            serde_json::json!([]),
            serde_json::json!({}),
        ]
        .into_iter()
        .all(|value| serde_json::from_value::<Text>(value).is_err());
        crate::admin_bool::AdminBool::from(accepted && rejected_lengths && rejected_types)
    }
    assert!([
        administrator_text_wire_matches::<crate::admin_text::AdminText, 8192usize>(),
        administrator_text_wire_matches::<crate::admin_filter_value::AdminFilterValue, 4096usize>(),
        administrator_text_wire_matches::<crate::admin_organization_name::AdminOrganizationName, 8192usize>(),
        administrator_text_wire_matches::<crate::admin_organization_contacts::AdminOrganizationContacts, 8192usize>(),
        administrator_text_wire_matches::<crate::admin_session_timestamp::AdminSessionTimestamp, 64usize>(),
        administrator_text_wire_matches::<crate::admin_table_sort_key::AdminTableSortKey, 32usize>(),
        administrator_text_wire_matches::<crate::admin_session_identifier::AdminSessionIdentifier, 64usize>(),
        administrator_text_wire_matches::<crate::admin_table_search::AdminTableSearch, 128usize>(),
        administrator_text_wire_matches::<crate::admin_rule_value::AdminRuleValue, 128usize>(),
    ].into_iter().all(bool::from));
}

#[test]
fn test_access_and_refresh_identifiers_share_validated_uuid_wire_grammar() {
    fn identifier_uuid_wire_matches<Identifier>() -> crate::admin_bool::AdminBool
    where
        Identifier: TryFrom<String>
            + AsRef<str>
            + PartialEq
            + serde::Serialize
            + serde::de::DeserializeOwned,
    {
        let valid = [
            constants_str::TEST_ACCESS_SESSION_ID.to_owned(),
            constants_str::TEST_ACCESS_SESSION_ID.to_ascii_uppercase(),
            constants_str::TEST_ACCESS_SESSION_ID
                .bytes()
                .map(|byte| char::from(if byte == b'-' { byte } else { b'a' }))
                .collect::<String>(),
        ]
        .into_iter()
        .all(|text| {
            Identifier::try_from(text.clone()).is_ok_and(|identifier| {
                identifier.as_ref() == text.as_str()
                    && serde_json::to_value(&identifier)
                        .is_ok_and(|wire| wire == serde_json::json!(text))
                    && serde_json::from_value::<Identifier>(serde_json::json!(text))
                        .is_ok_and(|decoded| decoded == identifier)
            })
        });
        let invalid_positions = (0usize..36usize).all(|invalid_position| {
            let text = constants_str::TEST_ACCESS_SESSION_ID
                .bytes()
                .enumerate()
                .map(|(position, byte)| {
                    char::from(if position == invalid_position {
                        b'_'
                    } else {
                        byte
                    })
                })
                .collect::<String>();
            Identifier::try_from(text.clone()).is_err()
                && serde_json::from_value::<Identifier>(serde_json::json!(text))
                    .is_err_and(|error| error.is_data())
        });
        let invalid_lengths_and_unicode = [
            String::new(),
            constants_str::TEST_ACCESS_SESSION_ID
                .chars()
                .take(35usize)
                .collect::<String>(),
            format!(
                "{}{}",
                constants_str::TEST_ACCESS_SESSION_ID,
                constants_str::A_ALT
            ),
            '\u{00e9}'.to_string().repeat(36usize),
        ]
        .into_iter()
        .all(|text| {
            Identifier::try_from(text.clone()).is_err()
                && serde_json::from_value::<Identifier>(serde_json::json!(text))
                    .is_err_and(|error| error.is_data())
        });
        let invalid_types = [
            serde_json::Value::Null,
            serde_json::json!(1i64),
            serde_json::json!(true),
            serde_json::json!([]),
            serde_json::json!({}),
        ]
        .into_iter()
        .all(|value| {
            serde_json::from_value::<Identifier>(value).is_err_and(|error| error.is_data())
        });
        crate::admin_bool::AdminBool::from(
            valid && invalid_positions && invalid_lengths_and_unicode && invalid_types,
        )
    }
    assert_eq!(
        identifier_uuid_wire_matches::<crate::admin_access_session_id::AdminAccessSessionId>(),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        identifier_uuid_wire_matches::<crate::admin_refresh_token_id::AdminRefreshTokenId>(),
        crate::admin_bool::AdminBool::from(true)
    );
    let short = constants_str::TEST_ACCESS_SESSION_ID
        .chars()
        .take(35usize)
        .collect::<String>();
    assert!(matches!(
        crate::admin_access_session_id::AdminAccessSessionId::try_from(short.clone()),
        Err(
            crate::admin_access_session_id::AdminAccessSessionIdTryFromStringError::TooShort {
                len: 35usize,
                min: 36usize
            }
        )
    ));
    assert!(matches!(
        crate::admin_refresh_token_id::AdminRefreshTokenId::try_from(short),
        Err(
            crate::admin_refresh_token_id::AdminRefreshTokenIdTryFromStringError::TooShort {
                len: 35usize,
                min: 36usize
            }
        )
    ));
    let long = format!(
        "{}{}",
        constants_str::TEST_ACCESS_SESSION_ID,
        constants_str::A_ALT
    );
    assert!(matches!(
        crate::admin_access_session_id::AdminAccessSessionId::try_from(long.clone()),
        Err(
            crate::admin_access_session_id::AdminAccessSessionIdTryFromStringError::TooLong {
                len: 37usize,
                max: 36usize
            }
        )
    ));
    assert!(matches!(
        crate::admin_refresh_token_id::AdminRefreshTokenId::try_from(long),
        Err(
            crate::admin_refresh_token_id::AdminRefreshTokenIdTryFromStringError::TooLong {
                len: 37usize,
                max: 36usize
            }
        )
    ));
    let invalid_value = constants_str::X.repeat(36usize);
    assert!(matches!(
        crate::admin_access_session_id::AdminAccessSessionId::try_from(invalid_value.clone()),
        Err(crate::admin_access_session_id::AdminAccessSessionIdTryFromStringError::InvalidValue)
    ));
    assert!(matches!(
        crate::admin_refresh_token_id::AdminRefreshTokenId::try_from(invalid_value),
        Err(crate::admin_refresh_token_id::AdminRefreshTokenIdTryFromStringError::InvalidValue)
    ));
}

#[test]
fn test_bounded_hash_map_serialization_preserves_entries_and_reports_invalid_json_keys() {
    let empty = bounded_types::bounded_hash_map::BoundedHashMap::<u8, u8, 0usize>::default();
    assert!(serde_json::to_value(empty).is_ok_and(|wire| wire == serde_json::json!({})));
    let result = bounded_types::bounded_hash_map::BoundedHashMap::<u8, u8, 2usize>::try_from(
        std::collections::HashMap::from([(1u8, 2u8), (2u8, 1u8)]),
    );
    assert!(result.as_ref().err().is_none());
    if let Ok(map) = result {
        let expected =
            serde_json::json!({(constants_str::VALUE_1): 2u8, (constants_str::VALUE_2): 1u8});
        assert!(serde_json::to_value(&map).is_ok_and(|wire| wire == expected));
        assert_eq!(map.get(&1u8), Some(&2u8));
        assert_eq!(map.get(&2u8), Some(&1u8));
        assert_eq!(
            map.len(),
            bounded_types::bounded_len::BoundedLen::from(2usize)
        );
        assert!(serde_json::from_value::<bounded_types::bounded_hash_map::BoundedHashMap<u8, u8, 2usize>>(expected).is_ok_and(|decoded| decoded == map));
    }
    let composite_result =
        bounded_types::bounded_hash_map::BoundedHashMap::<[u8; 2usize], u8, 1usize>::try_from(
            std::collections::HashMap::from([([1u8, 2u8], 3u8)]),
        );
    assert!(composite_result.as_ref().err().is_none());
    if let Ok(map) = composite_result {
        assert!(serde_json::to_value(&map).is_err_and(|error| error.is_syntax()));
        assert_eq!(map.get(&[1u8, 2u8]), Some(&3u8));
    }
}
