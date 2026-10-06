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
