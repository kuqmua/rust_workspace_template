#[test]
fn test_role_and_rule_identifier_collections_preserve_defaults_duplicates_and_bounds() {
    fn collection_matches<Collection>() -> crate::admin_bool::AdminBool
    where
        Collection: Default + serde::Serialize + serde::de::DeserializeOwned,
    {
        let ordered = serde_json::json!([2i64, 1i64, 2i64]);
        let maximum = serde_json::Value::Array(vec![serde_json::json!(1i64); 10_000usize]);
        let oversized = serde_json::Value::Array(vec![serde_json::json!(1i64); 10_001usize]);
        crate::admin_bool::AdminBool::from(
            serde_json::to_value(Collection::default())
                .is_ok_and(|wire| wire == serde_json::json!([]))
                && serde_json::from_value::<Collection>(ordered.clone()).is_ok_and(|collection| {
                    serde_json::to_value(collection).is_ok_and(|wire| wire == ordered)
                })
                && serde_json::from_value::<Collection>(maximum.clone()).is_ok_and(|collection| {
                    serde_json::to_value(collection).is_ok_and(|wire| wire == maximum)
                })
                && serde_json::from_value::<Collection>(oversized).is_err()
                && [
                    serde_json::json!([0i64]),
                    serde_json::json!([-1i64]),
                    serde_json::json!([constants_str::X]),
                ]
                .into_iter()
                .all(|wire| serde_json::from_value::<Collection>(wire).is_err()),
        )
    }
    assert_eq!(
        collection_matches::<crate::admin_role_ids::AdminRoleIds>(),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        collection_matches::<crate::admin_rule_ids::AdminRuleIds>(),
        crate::admin_bool::AdminBool::from(true)
    );
    let identifier_result = crate::admin_rule_id::AdminRuleId::try_from(constants_i64::ONE);
    assert!(identifier_result.as_ref().err().is_none());
    let Ok(identifier) = identifier_result else {
        return;
    };
    assert!(
        crate::admin_rule_ids::AdminRuleIds::try_from(vec![identifier; 10_000usize]).is_ok_and(
            |identifiers| identifiers.as_ref().len() == 10_000usize
                && identifiers
                    .as_ref()
                    .iter()
                    .all(|value| *value == identifier)
        )
    );
    assert!(
        matches!(crate::admin_rule_ids::AdminRuleIds::try_from(vec![identifier; 10_001usize]), Err(crate::admin_collection_error::AdminCollectionError::TooLong(bounded_types::bounded_value_error::BoundedValueError::AboveMax { actual, max })) if actual.get() == 10_001usize && max.get() == 10_000usize)
    );
}

#[test]
fn test_administrator_identifier_integer_round_trips_preserve_positive_bounds() {
    fn identifier_integer_contract_matches<Identifier>() -> crate::admin_bool::AdminBool
    where
        Identifier: TryFrom<i64, Error = crate::admin_id_try_from_i64_error::AdminIdTryFromI64Error>
            + Into<i64>
            + Copy
            + serde::Serialize
            + serde::de::DeserializeOwned
            + std::fmt::Display,
    {
        crate::admin_bool::AdminBool::from(
            [1i64, i64::MAX].into_iter().all(|value| {
                Identifier::try_from(value).is_ok_and(|identifier| {
                    let converted = Into::<i64>::into(identifier);
                    converted == value
                        && identifier.to_string() == value.to_string()
                        && serde_json::to_value(identifier)
                            .is_ok_and(|wire| wire == serde_json::json!(value))
                        && serde_json::from_value::<Identifier>(serde_json::json!(value)).is_ok_and(
                            |decoded| {
                                let decoded_value = Into::<i64>::into(decoded);
                                decoded_value == value
                            },
                        )
                })
            }) && [0i64, -1i64, i64::MIN].into_iter().all(|value| {
                Identifier::try_from(value).is_err()
                    && serde_json::from_value::<Identifier>(serde_json::json!(value)).is_err()
            }),
        )
    }
    assert_eq!(
        identifier_integer_contract_matches::<crate::admin_cleanup_status_id::AdminCleanupStatusId>(
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        identifier_integer_contract_matches::<crate::admin_login_attempt_id::AdminLoginAttemptId>(),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        identifier_integer_contract_matches::<
            crate::admin_permission_action_id::AdminPermissionActionId,
        >(),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        identifier_integer_contract_matches::<
            crate::admin_permission_resource_action_id::AdminPermissionResourceActionId,
        >(),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        identifier_integer_contract_matches::<
            crate::admin_permission_resource_id::AdminPermissionResourceId,
        >(),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        identifier_integer_contract_matches::<crate::admin_rate_limit_id::AdminRateLimitId>(),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        identifier_integer_contract_matches::<crate::admin_role_rule_id::AdminRoleRuleId>(),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        identifier_integer_contract_matches::<crate::admin_system_setting_id::AdminSystemSettingId>(
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        identifier_integer_contract_matches::<crate::admin_user_role_id::AdminUserRoleId>(),
        crate::admin_bool::AdminBool::from(true)
    );
}
