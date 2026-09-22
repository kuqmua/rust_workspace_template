#[test]
fn test_delete_access_sessions_request_preserves_filter() {
    let request = serde_json::from_value::<
        crate::admin_delete_access_sessions_request::AdminDeleteAccessSessionsRequest,
    >(serde_json::json!({
        (stringify!(filter)): {
            (stringify!(session_id)): constants_str::VALUE_4943E43B,
            (stringify!(user_id)): 1i64,
        }
    }));
    assert!(request.is_ok_and(|admin_delete_access_sessions_request| {
        admin_delete_access_sessions_request
            .filter()
            .session_id()
            .is_some()
            && admin_delete_access_sessions_request
                .filter()
                .user_id()
                .is_some()
    }));
    let route = crate::admin_route::AdminRoute::DeleteAccessSessions;
    assert_eq!(
        route.contract().method(),
        frontend_contract::route_method::RouteMethod::Delete
    );
    assert_eq!(
        route.contract().authentication(),
        frontend_contract::authentication_requirement::AuthenticationRequirement::Rule(
            frontend_contract::contract_str::ContractStr::from(
                crate::admin_rule::AdminRule::AccessSessionsDelete
                    .as_str()
                    .get(),
            )
        )
    );
}

#[test]
fn test_delete_access_sessions_request_rejects_invalid_filter_fields() {
    assert!(
        [
            serde_json::json!({(stringify!(filter)): {(stringify!(user_id)): 0i64}}),
            serde_json::json!({(stringify!(filter)): {(stringify!(unknown)): constants_str::VALUE_4943E43B}}),
            serde_json::json!({(stringify!(session_id)): constants_str::VALUE_4943E43B}),
        ]
        .into_iter()
        .all(|request| serde_json::from_value::<
            crate::admin_delete_access_sessions_request::AdminDeleteAccessSessionsRequest,
        >(request)
        .is_err())
    );
}
