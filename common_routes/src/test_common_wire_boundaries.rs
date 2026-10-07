#[test]
fn test_common_no_body_serializes_as_null_and_rejects_nonnull_values() {
    assert!(
        serde_json::to_value(crate::common_no_body::CommonNoBody)
            .is_ok_and(|value| value == serde_json::Value::Null)
    );
    assert!(
        serde_json::from_value::<crate::common_no_body::CommonNoBody>(serde_json::Value::Null)
            .is_ok_and(|common_no_body| serde_json::to_value(common_no_body)
                .is_ok_and(|value| value == serde_json::Value::Null))
    );
    assert!(
        [
            serde_json::json!(true),
            serde_json::json!(0i32),
            serde_json::json!(constants_str::EMPTY),
            serde_json::json!([]),
            serde_json::json!({}),
        ]
        .into_iter()
        .all(
            |value| serde_json::from_value::<crate::common_no_body::CommonNoBody>(value)
                .is_err_and(|error| error.is_data())
        )
    );
}

#[test]
fn test_health_collection_error_preserves_domain_message_without_source() {
    let error = crate::health_components_error::HealthComponentsError::TooMany;
    assert_eq!(
        error.to_string(),
        constants_str::HEALTH_COMPONENTS_LENGTH_EXCEEDS_LIMIT
    );
    assert!(std::error::Error::source(&error).is_none());
}

#[test]
fn test_authority_only_uri_suffix_uses_uri_path_fallback() {
    let uri = axum::http::Uri::from_static(constants_str::LOCALHOST);
    assert!(uri.path_and_query().is_none());
    let suffix =
        crate::uri_suffix_tests::uri_suffix(crate::axum_http_uri_ref::AxumHttpUriRef::from(&uri));
    assert_eq!(*suffix, uri.path());
    let message = crate::make_no_route_message_tests::make_no_route_message(
        crate::axum_http_uri_ref::AxumHttpUriRef::from(&uri),
    );
    assert_eq!(
        message.as_ref(),
        constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX
    );
}
