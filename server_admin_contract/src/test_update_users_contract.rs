#[test]
fn test_update_users_request_preserves_individual_changes() {
    let request = serde_json::from_value::<
        crate::admin_update_users_request::AdminUpdateUsersRequest,
    >(serde_json::json!({
        (stringify!(updates)): [
            {(stringify!(filter)): {(stringify!(user_id)): 1i64}, (stringify!(changes)): {(stringify!(login)): constants_str::ADMIN_ALT}},
            {(stringify!(filter)): {(stringify!(user_id)): 2i64}, (stringify!(changes)): {(stringify!(is_banned)): false}}
        ]
    }));
    assert!(request.is_ok_and(|admin_update_users_request| {
        let updates = admin_update_users_request.updates().as_ref();
        updates.len() == 2
            && updates.first().is_some_and(|admin_user_update| {
                admin_user_update.changes().login().is_some()
                    && admin_user_update.changes().is_banned().is_none()
            })
            && updates.get(1).is_some_and(|admin_user_update| {
                admin_user_update
                    .changes()
                    .is_banned()
                    .copied()
                    .is_some_and(|admin_bool| !bool::from(admin_bool))
            })
    }));
}

#[test]
fn test_update_users_request_rejects_invalid_identifiers_fields_and_values() {
    assert!([
        serde_json::json!({(stringify!(filter)): {(stringify!(user_id)): 0i64}, (stringify!(changes)): {(stringify!(login)): constants_str::ADMIN_ALT}}),
        serde_json::json!({(stringify!(filter)): {(stringify!(user_id)): 1i64}, (stringify!(changes)): {(stringify!(login)): constants_str::PG_CRUD_EMPTY_SQL_SUFFIX}}),
        serde_json::json!({(stringify!(filter)): {(stringify!(user_id)): 1i64}, (stringify!(changes)): {(stringify!(password_hash)): constants_str::ADMIN_ALT}}),
        serde_json::json!({(stringify!(filter)): {(stringify!(login)): constants_str::PG_CRUD_EMPTY_SQL_SUFFIX}, (stringify!(changes)): {(stringify!(is_banned)): false}}),
        serde_json::json!({(stringify!(filter)): {(stringify!(password_hash)): constants_str::ADMIN_ALT}, (stringify!(changes)): {(stringify!(is_banned)): false}}),
        serde_json::json!({(stringify!(user_id)): 1i64, (stringify!(changes)): {(stringify!(is_banned)): false}}),
    ].into_iter().all(|update| {
        serde_json::from_value::<crate::admin_update_users_request::AdminUpdateUsersRequest>(
            serde_json::json!({(stringify!(updates)): [update]}),
        ).is_err()
    }));
}
