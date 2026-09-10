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

#[test]
fn test_update_users_password_validates_and_redacts() {
    let password_request = serde_json::from_value::<
        crate::admin_update_users_request::AdminUpdateUsersRequest,
    >(serde_json::json!({(stringify!(updates)): [{
        (stringify!(filter)): {(stringify!(user_id)): 1i64},
        (stringify!(changes)): {(stringify!(password)): constants_str::VALUE_4EDBB68D}
    }]}));
    assert!(password_request.is_ok_and(|validated_request| {
        validated_request
            .updates()
            .as_ref()
            .first()
            .is_some_and(|update| {
                update.changes().password().is_some()
                    && !format!("{validated_request:?}").contains(constants_str::VALUE_4EDBB68D)
            })
    }));
    assert!(
        serde_json::from_value::<crate::admin_update_user_request::AdminUpdateUserRequest>(
            serde_json::json!({(stringify!(password)): constants_str::ADMIN}),
        )
        .is_err_and(|error| error.is_data())
    );
    assert!(
        [
            serde_json::json!({(stringify!(password)): null}),
            serde_json::json!({}),
        ]
        .into_iter()
        .all(|value| {
            serde_json::from_value::<crate::admin_update_user_request::AdminUpdateUserRequest>(
                value,
            )
            .is_ok_and(|unchanged_request| unchanged_request.password().is_none())
        })
    );
}

#[test]
fn test_user_role_fields_preserve_empty_and_omitted_assignments() {
    assert!(serde_json::from_value::<crate::admin_update_user_request::AdminUpdateUserRequest>(
        serde_json::json!({(stringify!(expected_role_ids)): [1i64], (stringify!(role_ids)): []}),
    ).is_ok_and(|assignment| {
        assignment.expected_role_ids().is_some_and(|identifiers| AsRef::<[crate::admin_role_id::AdminRoleId]>::as_ref(identifiers).len() == 1)
            && assignment.role_ids().is_some_and(|identifiers| AsRef::<[crate::admin_role_id::AdminRoleId]>::as_ref(identifiers).is_empty())
    }));
    assert!(
        [
            serde_json::json!({}),
            serde_json::json!({(stringify!(role_ids)): null, (stringify!(expected_role_ids)): null})
        ]
        .into_iter()
        .all(|value| serde_json::from_value::<
            crate::admin_update_user_request::AdminUpdateUserRequest,
        >(value)
        .is_ok_and(|assignment| assignment.role_ids().is_none()
            && assignment.expected_role_ids().is_none()))
    );
    assert!([0i64, -1i64].into_iter().all(|identifier| {
        serde_json::from_value::<crate::admin_update_user_request::AdminUpdateUserRequest>(
            serde_json::json!({(stringify!(role_ids)): [identifier]}),
        )
        .is_err()
    }));
    assert!(serde_json::from_value::<crate::admin_create_user_request::AdminCreateUserRequest>(
        serde_json::json!({(stringify!(login)): constants_str::LOGIN, (stringify!(display_name)): constants_str::ADMIN, (stringify!(password)): constants_str::VALUE_4EDBB68D, (stringify!(role_ids)): [1i64]}),
    ).is_ok_and(|creation| creation.into_parts().3.is_some_and(|identifiers| AsRef::<[crate::admin_role_id::AdminRoleId]>::as_ref(&identifiers).len() == 1)));
}
