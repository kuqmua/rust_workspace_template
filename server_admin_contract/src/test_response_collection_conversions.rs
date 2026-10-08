#[test]
fn test_response_collections_preserve_conversion_order_duplicates_and_bounds() {
    fn response_collection_matches<Item, Collection>(
        item: &Item,
        second_item: &Item,
    ) -> crate::admin_bool::AdminBool
    where
        Item: Clone + serde::Serialize,
        Collection: TryFrom<Vec<Item>, Error = crate::admin_collection_error::AdminCollectionError>
            + serde::Serialize,
    {
        let items = vec![item.clone(), second_item.clone(), item.clone()];
        let expected = serde_json::to_value(&items);
        crate::admin_bool::AdminBool::from(expected.is_ok_and(|expected_wire| {
            Collection::try_from(items).is_ok_and(|collection| serde_json::to_value(collection).is_ok_and(|wire| wire == expected_wire))
                && Collection::try_from(Vec::new()).is_ok_and(|collection| serde_json::to_value(collection).is_ok_and(|wire| wire == serde_json::json!([])))
                && Collection::try_from(vec![item.clone(); 10_000usize]).is_ok_and(|collection| serde_json::to_value(collection).is_ok_and(|wire| wire.as_array().is_some_and(|values| values.len() == 10_000usize)))
                && matches!(Collection::try_from(vec![item.clone(); 10_001usize]), Err(crate::admin_collection_error::AdminCollectionError::TooLong(bounded_types::bounded_value_error::BoundedValueError::AboveMax { actual, max })) if actual.get() == 10_001usize && max.get() == 10_000usize)
        }))
    }
    let text_result = crate::admin_text::AdminText::try_from(constants_str::X.to_owned());
    let second_text_result =
        crate::admin_text::AdminText::try_from(constants_str::ADMIN.to_owned());
    assert!(text_result.is_ok());
    assert!(second_text_result.is_ok());
    let (Ok(text), Ok(second_text)) = (text_result, second_text_result) else {
        return;
    };
    assert_eq!(
        response_collection_matches::<_, crate::admin_texts::AdminTexts>(&text, &second_text),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        response_collection_matches::<_, crate::admin_data_tables::AdminDataTables>(
            &crate::admin_data_table::AdminDataTable::Users,
            &crate::admin_data_table::AdminDataTable::Roles
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    let row_result = serde_json::from_value::<crate::admin_data_row::AdminDataRow>(
        serde_json::json!({(stringify!(values)): [constants_str::X]}),
    );
    let second_row_result = serde_json::from_value::<crate::admin_data_row::AdminDataRow>(
        serde_json::json!({(stringify!(values)): [constants_str::ADMIN]}),
    );
    assert!(row_result.is_ok());
    assert!(second_row_result.is_ok());
    let (Ok(row), Ok(second_row)) = (row_result, second_row_result) else {
        return;
    };
    assert_eq!(
        response_collection_matches::<_, crate::admin_data_rows::AdminDataRows>(&row, &second_row),
        crate::admin_bool::AdminBool::from(true)
    );
    let column_result = serde_json::from_value::<crate::admin_data_column::AdminDataColumn>(
        serde_json::json!({(stringify!(filters)): [], (stringify!(label)): constants_str::X, (stringify!(name)): constants_str::LOGIN, (stringify!(input_kind)): frontend_contract::input_kind::InputKind::Text}),
    );
    let second_column_result = serde_json::from_value::<crate::admin_data_column::AdminDataColumn>(
        serde_json::json!({(stringify!(filters)): [], (stringify!(label)): constants_str::ADMIN, (stringify!(name)): constants_str::USER_ID, (stringify!(input_kind)): frontend_contract::input_kind::InputKind::Number}),
    );
    assert!(column_result.is_ok());
    assert!(second_column_result.is_ok());
    let (Ok(column), Ok(second_column)) = (column_result, second_column_result) else {
        return;
    };
    assert_eq!(
        response_collection_matches::<_, crate::admin_data_columns::AdminDataColumns>(
            &column,
            &second_column
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    assert!(
        crate::admin_texts::AdminTexts::try_from(vec![text.clone(), second_text, text]).is_ok_and(
            |texts| {
                serde_json::to_value(texts.as_slice()).is_ok_and(|wire| {
                    wire == serde_json::json!([
                        constants_str::X,
                        constants_str::ADMIN,
                        constants_str::X
                    ])
                })
            }
        )
    );
    let tables = [
        crate::admin_data_table::AdminDataTable::Users,
        crate::admin_data_table::AdminDataTable::Roles,
        crate::admin_data_table::AdminDataTable::Users,
    ];
    assert!(
        crate::admin_data_tables::AdminDataTables::try_from(tables.to_vec())
            .is_ok_and(|values| values.as_slice() == tables)
    );
    let expected_rows = serde_json::json!([&row, &second_row, &row]);
    assert!(
        crate::admin_data_rows::AdminDataRows::try_from(vec![row.clone(), second_row, row])
            .is_ok_and(|rows| serde_json::to_value(rows.as_slice())
                .is_ok_and(|wire| wire == expected_rows))
    );
    let expected_columns = serde_json::json!([&column, &second_column, &column]);
    assert!(
        crate::admin_data_columns::AdminDataColumns::try_from(vec![
            column.clone(),
            second_column,
            column
        ])
        .is_ok_and(|columns| serde_json::to_value(columns.as_slice())
            .is_ok_and(|wire| wire == expected_columns))
    );
    let user_results = [1i64, 2i64].map(|identifier| {
        serde_json::from_value::<crate::admin_user_summary::AdminUserSummary>(serde_json::json!({(stringify!(display_name)): constants_str::ADMIN, (stringify!(id)): identifier, (stringify!(is_banned)): false, (stringify!(login)): constants_str::LOGIN, (stringify!(role_ids)): []}))
    });
    assert!(user_results.iter().all(Result::is_ok));
    let [Ok(user), Ok(second_user)] = user_results else {
        return;
    };
    assert_eq!(
        response_collection_matches::<_, crate::admin_user_summaries::AdminUserSummaries>(
            &user,
            &second_user
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    let expected_users = serde_json::json!([&user, &second_user, &user]);
    assert!(
        crate::admin_user_summaries::AdminUserSummaries::try_from(vec![
            user.clone(),
            second_user,
            user
        ])
        .is_ok_and(|values| serde_json::to_value(values.as_slice())
            .is_ok_and(|wire| wire == expected_users))
    );
    let role_results = [1i64, 2i64].map(|identifier| {
        serde_json::from_value::<crate::admin_role_summary::AdminRoleSummary>(serde_json::json!({(stringify!(id)): identifier, (stringify!(is_system)): false, (stringify!(name)): constants_str::LOGIN, (stringify!(created_at)): constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT, (stringify!(updated_at)): constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT}))
    });
    assert!(role_results.iter().all(Result::is_ok));
    let [Ok(role), Ok(second_role)] = role_results else {
        return;
    };
    assert_eq!(
        response_collection_matches::<_, crate::admin_role_summaries::AdminRoleSummaries>(
            &role,
            &second_role
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    let expected_roles = serde_json::json!([&role, &second_role, &role]);
    assert!(
        crate::admin_role_summaries::AdminRoleSummaries::try_from(vec![
            role.clone(),
            second_role,
            role
        ])
        .is_ok_and(|values| serde_json::to_value(values.as_slice())
            .is_ok_and(|wire| wire == expected_roles))
    );
    let rule_results = [1i64, 2i64].map(|identifier| {
        serde_json::from_value::<crate::admin_rule_summary::AdminRuleSummary>(serde_json::json!({(stringify!(id)): identifier, (stringify!(name)): constants_str::VALUE_C6919F81, (stringify!(created_at)): constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT}))
    });
    assert!(rule_results.iter().all(Result::is_ok));
    let [Ok(rule), Ok(second_rule)] = rule_results else {
        return;
    };
    assert_eq!(
        response_collection_matches::<_, crate::admin_rule_summaries::AdminRuleSummaries>(
            &rule,
            &second_rule
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    let expected_rules = serde_json::json!([&rule, &second_rule, &rule]);
    assert!(
        crate::admin_rule_summaries::AdminRuleSummaries::try_from(vec![
            rule.clone(),
            second_rule,
            rule
        ])
        .is_ok_and(|values| serde_json::to_value(values.as_slice())
            .is_ok_and(|wire| wire == expected_rules))
    );
    let audit_results = [1i64, 2i64].map(|identifier| {
        serde_json::from_value::<crate::admin_audit_view::AdminAuditView>(serde_json::json!({(stringify!(id)): identifier, (stringify!(action)): constants_str::X, (stringify!(created_at)): constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT, (stringify!(resource)): constants_str::LOGIN, (stringify!(succeeded)): true, (stringify!(details)): null, (stringify!(resource_id)): null, (stringify!(user_id)): null, (stringify!(user_login)): null}))
    });
    assert!(audit_results.iter().all(Result::is_ok));
    let [Ok(audit), Ok(second_audit)] = audit_results else {
        return;
    };
    assert_eq!(
        response_collection_matches::<_, crate::admin_audit_views::AdminAuditViews>(
            &audit,
            &second_audit
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    let expected_audits = serde_json::json!([&audit, &second_audit, &audit]);
    assert!(
        crate::admin_audit_views::AdminAuditViews::try_from(vec![
            audit.clone(),
            second_audit,
            audit
        ])
        .is_ok_and(|values| serde_json::to_value(values.as_slice())
            .is_ok_and(|wire| wire == expected_audits))
    );
    let role_identifier_results = [1i64, 2i64].map(|identifier| {
        serde_json::from_value::<crate::admin_role_id::AdminRoleId>(serde_json::json!(identifier))
    });
    assert!(role_identifier_results.iter().all(Result::is_ok));
    let [Ok(role_identifier), Ok(second_role_identifier)] = role_identifier_results else {
        return;
    };
    assert_eq!(
        response_collection_matches::<_, crate::admin_role_ids::AdminRoleIds>(
            &role_identifier,
            &second_role_identifier
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    let role_name_results = [1i64, 2i64].map(|identifier| {
        serde_json::from_value::<crate::admin_role_name::AdminRoleName>(serde_json::json!(
            if identifier == 1i64 {
                constants_str::LOGIN
            } else {
                constants_str::ROOT
            }
        ))
    });
    assert!(role_name_results.iter().all(Result::is_ok));
    let [Ok(role_name), Ok(second_role_name)] = role_name_results else {
        return;
    };
    assert_eq!(
        response_collection_matches::<_, crate::admin_role_names::AdminRoleNames>(
            &role_name,
            &second_role_name
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    let user_update_results = [1i64, 2i64].map(|identifier| {
        serde_json::from_value::<crate::admin_user_update::AdminUserUpdate>(serde_json::json!({(stringify!(changes)): {(stringify!(is_banned)): identifier == 1i64}, (stringify!(filter)): {(stringify!(user_id)): identifier}}))
    });
    assert!(user_update_results.iter().all(Result::is_ok));
    let [Ok(user_update), Ok(second_user_update)] = user_update_results else {
        return;
    };
    assert_eq!(
        response_collection_matches::<_, crate::admin_user_updates::AdminUserUpdates>(
            &user_update,
            &second_user_update
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    assert!(
        crate::admin_role_ids::AdminRoleIds::default()
            .as_slice()
            .is_empty()
    );
    let expected_role_identifiers = [role_identifier, second_role_identifier, role_identifier];
    assert!(
        crate::admin_role_ids::AdminRoleIds::try_from(expected_role_identifiers.to_vec())
            .is_ok_and(|identifiers| identifiers.as_slice() == expected_role_identifiers)
    );
    let user_identifier_results = [1i64, 2i64].map(|identifier| {
        serde_json::from_value::<crate::admin_user_id::AdminUserId>(serde_json::json!(identifier))
    });
    assert!(user_identifier_results.iter().all(Result::is_ok));
    let [Ok(user_identifier), Ok(second_user_identifier)] = user_identifier_results else {
        return;
    };
    assert_eq!(
        response_collection_matches::<_, crate::admin_user_ids::AdminUserIds>(
            &user_identifier,
            &second_user_identifier
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    let rule_value_results = [
        crate::admin_rule::AdminRule::UsersRead,
        crate::admin_rule::AdminRule::RolesRead,
    ]
    .map(|admin_rule| {
        crate::admin_rule_value::AdminRuleValue::try_from(admin_rule.as_str().get().to_owned())
    });
    assert!(rule_value_results.iter().all(Result::is_ok));
    if let [Ok(rule_value), Ok(second_rule_value)] = rule_value_results {
        assert_eq!(
            response_collection_matches::<_, crate::admin_rule_values::AdminRuleValues>(
                &rule_value,
                &second_rule_value
            ),
            crate::admin_bool::AdminBool::from(true)
        );
        let expected = serde_json::json!([&rule_value, &second_rule_value, &rule_value]);
        assert!(
            serde_json::from_value::<crate::admin_rule_values::AdminRuleValues>(expected.clone())
                .is_ok_and(|values| serde_json::to_value(values.as_ref())
                    .is_ok_and(|wire| wire == expected))
        );
        let oversized = serde_json::json!(vec![rule_value; 10_001usize]);
        assert!(
            serde_json::from_value::<crate::admin_rule_values::AdminRuleValues>(oversized)
                .is_err_and(|error| error.is_data())
        );
    }
}
