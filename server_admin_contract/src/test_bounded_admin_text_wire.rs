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
        administrator_text_wire_matches::<crate::admin_table_search::AdminTableSearch, 128usize>(),
        administrator_text_wire_matches::<crate::admin_rule_value::AdminRuleValue, 128usize>(),
    ].into_iter().all(bool::from));
}

#[test]
fn test_refresh_identifier_requires_positive_integer_wire_values() {
    assert!([1i64, i64::MAX].into_iter().all(|value| {
        serde_json::from_value::<crate::admin_refresh_token_id::AdminRefreshTokenId>(
            serde_json::json!(value),
        )
        .is_ok_and(|identifier| {
            i64::from(identifier) == value
                && serde_json::to_value(identifier)
                    .is_ok_and(|wire| wire == serde_json::json!(value))
        })
    }));
    assert!(
        [
            serde_json::json!(0i64),
            serde_json::json!(-1i64),
            serde_json::json!(constants_str::TEST_REFRESH_TOKEN_ID),
            serde_json::Value::Null
        ]
        .into_iter()
        .all(|wire| serde_json::from_value::<
            crate::admin_refresh_token_id::AdminRefreshTokenId,
        >(wire)
        .is_err())
    );
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
