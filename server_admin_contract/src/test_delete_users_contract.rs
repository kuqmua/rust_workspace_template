#[test]
fn test_delete_users_request_preserves_filter() {
    let request = serde_json::from_value::<
        crate::admin_delete_users_request::AdminDeleteUsersRequest,
    >(
        serde_json::json!({(stringify!(filter)): {(stringify!(display_name)): constants_str::ADMIN_ALT, (stringify!(is_banned)): true}}),
    );
    assert!(request.is_ok_and(|admin_delete_users_request| {
        admin_delete_users_request.filter().display_name().is_some()
            && admin_delete_users_request
                .filter()
                .is_banned()
                .copied()
                .is_some_and(bool::from)
    }));
    assert_eq!(
        crate::admin_route::AdminRoute::DeleteUsers
            .contract()
            .method(),
        frontend_contract::route_method::RouteMethod::Delete
    );
}

#[test]
fn test_delete_users_request_rejects_invalid_filter_fields() {
    assert!([
        serde_json::json!({(stringify!(filter)): {(stringify!(user_id)): 0i64}}),
        serde_json::json!({(stringify!(filter)): {(stringify!(login)): constants_str::PG_CRUD_EMPTY_SQL_SUFFIX}}),
        serde_json::json!({(stringify!(filter)): {(stringify!(password_hash)): constants_str::ADMIN_ALT}}),
        serde_json::json!({(stringify!(user_id)): 1i64}),
    ].into_iter().all(|request| serde_json::from_value::<crate::admin_delete_users_request::AdminDeleteUsersRequest>(request).is_err()));
}
