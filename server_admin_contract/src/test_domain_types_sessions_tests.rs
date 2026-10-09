#[test]
fn test_session_contract_tests() {
    let page = crate::admin_sessions_page::AdminSessionsPage::new(
        crate::admin_session_views::AdminSessionViews::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_C31F90A6),
        crate::admin_page_total::AdminPageTotal::from(3u64),
    );
    assert!(page.items().is_empty());
    assert_eq!(u64::from(page.total()), 3u64);
}

#[test]
fn test_session_identifiers_require_positive_integer_wire_values() {
    assert!([1i64, i64::MAX].into_iter().all(|value| {
        let wire = serde_json::json!(value);
        serde_json::from_value::<crate::admin_session_identifier::AdminSessionIdentifier>(
            wire.clone(),
        )
        .is_ok_and(|identifier| {
            i64::from(identifier) == value
                && serde_json::to_value(identifier).is_ok_and(|serialized| serialized == wire)
        }) && serde_json::from_value::<crate::admin_access_session_id::AdminAccessSessionId>(
            wire.clone(),
        )
        .is_ok_and(|identifier| {
            i64::from(identifier) == value
                && serde_json::to_value(identifier).is_ok_and(|serialized| serialized == wire)
        })
    }));
    assert!(
        [
            serde_json::json!(0i64),
            serde_json::json!(-1i64),
            serde_json::json!(i64::MIN),
            serde_json::json!(constants_str::TEST_ACCESS_SESSION_ID),
            serde_json::Value::Null,
        ]
        .into_iter()
        .all(|wire| {
            serde_json::from_value::<crate::admin_session_identifier::AdminSessionIdentifier>(
                wire.clone(),
            )
            .is_err()
                && serde_json::from_value::<crate::admin_access_session_id::AdminAccessSessionId>(
                    wire,
                )
                .is_err()
        })
    );
}
