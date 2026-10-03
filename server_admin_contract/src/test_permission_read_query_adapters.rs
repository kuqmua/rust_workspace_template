#[test]
fn test_permission_read_queries_preserve_pagination_and_map_sort_columns() {
    fn request_matches<Request>(
        admin_table_query: &crate::admin_table_query::AdminTableQuery,
        contract_str: frontend_contract::contract_str::ContractStr,
        admin_sort_direction: crate::admin_sort_direction::AdminSortDirection,
    ) -> crate::admin_bool::AdminBool
    where
        Request: serde::Serialize + for<'query> TryFrom<&'query crate::admin_table_query::AdminTableQuery, Error = crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError>,
    {
        crate::admin_bool::AdminBool::from(Request::try_from(admin_table_query).is_ok_and(|request| {
            serde_json::to_value(request).is_ok_and(|wire| {
                wire.get(stringify!(pagination)) == Some(&serde_json::json!({(stringify!(offset)): 7u32, (stringify!(limit)): 11u16}))
                    && wire.get(stringify!(search)).is_none_or(|search| *search == serde_json::json!(admin_table_query.search().as_ref()))
                    && wire.get(stringify!(where_many)) == Some(&serde_json::Value::Null)
                    && wire.get(stringify!(order_by)) == Some(&serde_json::json!({
                        (stringify!(column)): {(contract_str.as_ref()): null},
                        (stringify!(order)): admin_sort_direction,
                    }))
            })
        }))
    }
    let cases = [
        (constants_str::EMPTY, constants_str::SQL_NAMES_ID),
        (constants_str::SQL_NAMES_ID, constants_str::SQL_NAMES_ID),
        (
            constants_str::PERMISSION_ACTION_KEY,
            constants_str::PERMISSION_ACTION_KEY,
        ),
    ];
    assert!(cases.into_iter().all(|(sort, column)| {
        [crate::admin_sort_direction::AdminSortDirection::Ascending, crate::admin_sort_direction::AdminSortDirection::Descending].into_iter().all(|direction| {
            serde_json::from_value::<crate::admin_table_query::AdminTableQuery>(serde_json::json!({(stringify!(sort)): sort, (stringify!(direction)): direction, (stringify!(offset)): 7u32, (stringify!(limit)): 11u16})).is_ok_and(|query| {
                request_matches::<crate::admin_permission_actions_read_request::AdminPermissionActionsReadRequest>(&query, frontend_contract::contract_str::ContractStr::from(column), direction) == crate::admin_bool::AdminBool::from(true)
                    && request_matches::<crate::admin_permission_resources_read_request::AdminPermissionResourcesReadRequest>(&query, frontend_contract::contract_str::ContractStr::from(column), direction) == crate::admin_bool::AdminBool::from(true)
            })
        })
    }));
    let relation_cases = [
        (constants_str::EMPTY, constants_str::SQL_NAMES_ID),
        (constants_str::SQL_NAMES_ID, constants_str::SQL_NAMES_ID),
        (
            constants_str::PERMISSION_RESOURCE_ID,
            constants_str::PERMISSION_RESOURCE_ID,
        ),
        (
            constants_str::PERMISSION_ACTION_ID,
            constants_str::PERMISSION_ACTION_ID,
        ),
    ];
    assert!(relation_cases.into_iter().all(|(sort, column)| {
        [crate::admin_sort_direction::AdminSortDirection::Ascending, crate::admin_sort_direction::AdminSortDirection::Descending].into_iter().all(|direction| {
            serde_json::from_value::<crate::admin_table_query::AdminTableQuery>(serde_json::json!({(stringify!(sort)): sort, (stringify!(direction)): direction, (stringify!(offset)): 7u32, (stringify!(limit)): 11u16})).is_ok_and(|query| {
                request_matches::<crate::admin_permission_resource_actions_read_request::AdminPermissionResourceActionsReadRequest>(&query, frontend_contract::contract_str::ContractStr::from(column), direction) == crate::admin_bool::AdminBool::from(true)
            })
        })
    }));
    assert!([crate::admin_sort_direction::AdminSortDirection::Ascending, crate::admin_sort_direction::AdminSortDirection::Descending].into_iter().all(|direction| {
        serde_json::from_value::<crate::admin_table_query::AdminTableQuery>(serde_json::json!({(stringify!(search)): constants_str::X, (stringify!(direction)): direction, (stringify!(offset)): 7u32, (stringify!(limit)): 11u16})).is_ok_and(|query| {
            request_matches::<crate::admin_access_sessions_read_request::AdminAccessSessionsReadRequest>(&query, frontend_contract::contract_str::ContractStr::from(constants_str::CREATED_AT), crate::admin_sort_direction::AdminSortDirection::Descending) == crate::admin_bool::AdminBool::from(true)
                && request_matches::<crate::admin_role_rules_read_request::AdminRoleRulesReadRequest>(&query, frontend_contract::contract_str::ContractStr::from(constants_str::CREATED_AT), crate::admin_sort_direction::AdminSortDirection::Descending) == crate::admin_bool::AdminBool::from(true)
                && request_matches::<crate::admin_system_settings_read_request::AdminSystemSettingsReadRequest>(&query, frontend_contract::contract_str::ContractStr::from(constants_str::SQL_NAMES_ID), crate::admin_sort_direction::AdminSortDirection::Ascending) == crate::admin_bool::AdminBool::from(true)
                && crate::admin_system_settings_read_request::AdminSystemSettingsReadRequest::try_from(&query).is_ok_and(|request| request.get_search().is_some_and(|search| search.as_ref() == query.search().as_ref()))
        })
    }));
    assert!([constants_str::SQL_NAMES_ID, constants_str::CREATED_AT].into_iter().all(|sort| {
        serde_json::from_value::<crate::admin_table_query::AdminTableQuery>(serde_json::json!({(stringify!(sort)): sort})).is_ok_and(|query| {
            matches!(crate::admin_access_sessions_read_request::AdminAccessSessionsReadRequest::try_from(&query), Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown))
                && matches!(crate::admin_role_rules_read_request::AdminRoleRulesReadRequest::try_from(&query), Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown))
                && matches!(crate::admin_system_settings_read_request::AdminSystemSettingsReadRequest::try_from(&query), Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown))
        })
    }));
    assert!([(constants_str::EMPTY, constants_str::SQL_NAMES_ID), (constants_str::SQL_NAMES_ID, constants_str::SQL_NAMES_ID), (constants_str::LOGIN, constants_str::LOGIN), (constants_str::DISPLAY_NAME, constants_str::DISPLAY_NAME), (constants_str::STATUS_ALT, constants_str::IS_BANNED)].into_iter().all(|(sort, column)| {
        [crate::admin_sort_direction::AdminSortDirection::Ascending, crate::admin_sort_direction::AdminSortDirection::Descending].into_iter().all(|direction| {
            serde_json::from_value::<crate::admin_table_query::AdminTableQuery>(serde_json::json!({(stringify!(search)): constants_str::X, (stringify!(sort)): sort, (stringify!(direction)): direction, (stringify!(offset)): 7u32, (stringify!(limit)): 11u16})).is_ok_and(|query| {
                request_matches::<crate::admin_users_read_request::AdminUsersReadRequest>(&query, frontend_contract::contract_str::ContractStr::from(column), if sort.is_empty() { crate::admin_sort_direction::AdminSortDirection::Ascending } else { direction }) == crate::admin_bool::AdminBool::from(true)
            })
        })
    }));
    assert!([(constants_str::EMPTY, constants_str::SQL_NAMES_ID), (constants_str::SQL_NAMES_ID, constants_str::SQL_NAMES_ID), (constants_str::NAME, constants_str::NAME), (constants_str::SYSTEM, constants_str::IS_SYSTEM)].into_iter().all(|(sort, column)| {
        [crate::admin_sort_direction::AdminSortDirection::Ascending, crate::admin_sort_direction::AdminSortDirection::Descending].into_iter().all(|direction| {
            serde_json::from_value::<crate::admin_table_query::AdminTableQuery>(serde_json::json!({(stringify!(search)): constants_str::X, (stringify!(sort)): sort, (stringify!(direction)): direction, (stringify!(offset)): 7u32, (stringify!(limit)): 11u16})).is_ok_and(|query| {
                request_matches::<crate::admin_roles_read_request::AdminRolesReadRequest>(&query, frontend_contract::contract_str::ContractStr::from(column), if sort.is_empty() { crate::admin_sort_direction::AdminSortDirection::Ascending } else { direction }) == crate::admin_bool::AdminBool::from(true)
            })
        })
    }));
    assert!([(constants_str::SQL_NAMES_ID, constants_str::SQL_NAMES_ID), (constants_str::PERMISSION_RESOURCE_ACTION_ID, constants_str::PERMISSION_RESOURCE_ACTION_ID), (constants_str::BASEMAP_ID, constants_str::BASEMAP_ID), (constants_str::LAYER_GROUP_ID, constants_str::LAYER_GROUP_ID), (constants_str::LAYER_ID, constants_str::LAYER_ID), (constants_str::PROJECT_GROUP_ID, constants_str::PROJECT_GROUP_ID), (constants_str::PROJECT_ID, constants_str::PROJECT_ID), (constants_str::PROPERTY_ID, constants_str::PROPERTY_ID), (constants_str::ROLE_ID, constants_str::ROLE_ID), (constants_str::USER_ID, constants_str::USER_ID), (constants_str::FEATURE_ID, constants_str::FEATURE_ID), (constants_str::VALUE_ITEM_ID, constants_str::VALUE_ITEM_ID), (constants_str::CREATED_AT, constants_str::CREATED_AT), (constants_str::EMPTY, constants_str::SQL_NAMES_ID)].into_iter().all(|(sort, column)| {
        [crate::admin_sort_direction::AdminSortDirection::Ascending, crate::admin_sort_direction::AdminSortDirection::Descending].into_iter().all(|direction| {
            serde_json::from_value::<crate::admin_table_query::AdminTableQuery>(serde_json::json!({(stringify!(search)): constants_str::X, (stringify!(sort)): sort, (stringify!(direction)): direction, (stringify!(offset)): 7u32, (stringify!(limit)): 11u16})).is_ok_and(|query| {
                request_matches::<crate::admin_rules_read_request::AdminRulesReadRequest>(&query, frontend_contract::contract_str::ContractStr::from(column), direction) == crate::admin_bool::AdminBool::from(true)
            })
        })
    }));
    let invalid_query = serde_json::from_value::<crate::admin_table_query::AdminTableQuery>(
        serde_json::json!({(stringify!(sort)): constants_str::X}),
    );
    assert!(invalid_query.is_ok_and(|query| {
        matches!(crate::admin_users_read_request::AdminUsersReadRequest::try_from(&query), Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown))
            && matches!(crate::admin_roles_read_request::AdminRolesReadRequest::try_from(&query), Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown))
            && matches!(crate::admin_rules_read_request::AdminRulesReadRequest::try_from(&query), Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown))
            && matches!(crate::admin_permission_actions_read_request::AdminPermissionActionsReadRequest::try_from(&query), Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown))
            && matches!(crate::admin_permission_resources_read_request::AdminPermissionResourcesReadRequest::try_from(&query), Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown))
            && matches!(crate::admin_permission_resource_actions_read_request::AdminPermissionResourceActionsReadRequest::try_from(&query), Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown))
    }));
}
