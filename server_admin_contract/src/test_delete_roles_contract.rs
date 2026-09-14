#[test]
fn test_delete_roles_request_preserves_filter() {
    let request = serde_json::from_value::<
        crate::admin_delete_roles_request::AdminDeleteRolesRequest,
    >(
        serde_json::json!({(stringify!(filter)): {(stringify!(name)): constants_str::ADMIN_ALT, (stringify!(is_system)): false}}),
    );
    assert!(request.is_ok_and(|admin_delete_roles_request| {
        admin_delete_roles_request.filter().get_name().is_some()
            && admin_delete_roles_request
                .filter()
                .get_is_system()
                .copied()
                .is_some_and(|admin_bool| !bool::from(admin_bool))
    }));
    assert_eq!(
        crate::admin_route::AdminRoute::DeleteRoles
            .contract()
            .method(),
        frontend_contract::route_method::RouteMethod::Delete
    );
}

#[test]
fn test_delete_roles_request_rejects_invalid_filter_fields() {
    assert!([
        serde_json::json!({(stringify!(filter)): {(stringify!(role_id)): 0i64}}),
        serde_json::json!({(stringify!(filter)): {(stringify!(name)): constants_str::PG_CRUD_EMPTY_SQL_SUFFIX}}),
        serde_json::json!({(stringify!(filter)): {(stringify!(unknown)): constants_str::ADMIN_ALT}}),
        serde_json::json!({(stringify!(role_id)): 1i64}),
    ].into_iter().all(|request| serde_json::from_value::<crate::admin_delete_roles_request::AdminDeleteRolesRequest>(request).is_err()));
}
