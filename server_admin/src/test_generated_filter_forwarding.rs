#[test]
fn test_every_generated_table_empty_filter_preserves_placeholder_counter() {
    let text = serde_json::Value::Null.to_string();
    assert!(
        crate::admin_generated_table::AdminGeneratedTable::ALL
            .iter()
            .all(|table| {
                table
                    .parse_filter(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                        text.as_str(),
                    ))
                    .is_ok_and(|filter| {
                        [7u64, u64::MAX].into_iter().all(|initial_increment| {
                            let mut increment =
                                pg_crud_common::query_part_increment::QueryPartIncrement::from(
                                    initial_increment,
                                );
                            let result = filter.query_part(&mut increment);
                            matches!(
                                result,
                                Ok(pg_crud_common::query_part_fragment::QueryPartFragment { .. })
                            ) && increment.get() == initial_increment
                        })
                    })
            })
    );
}

#[test]
fn test_every_generated_table_rejects_malformed_filter_json() {
    assert!(
        crate::admin_generated_table::AdminGeneratedTable::ALL
            .iter()
            .all(|table| {
                matches!(
                    table.parse_filter(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                        constants_str::EMPTY
                    )),
                    Err(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                )
            })
    );
}

#[test]
fn test_every_generated_table_rejects_invalid_filter_json_shapes() {
    let values = [
        serde_json::json!(false),
        serde_json::json!(1i32),
        serde_json::json!(constants_str::X),
        serde_json::json!([]),
    ];
    assert!(values.into_iter().all(|value| {
        let text = value.to_string();
        crate::admin_generated_table::AdminGeneratedTable::ALL
            .iter()
            .all(|table| {
                matches!(
                    table.parse_filter(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                        text.as_str()
                    )),
                    Err(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
                )
            })
    }));
}

#[test]
fn test_auxiliary_table_empty_filters_preserve_placeholder_counter() {
    let text = serde_json::Value::Null.to_string();
    let filters =
        [
            serde_json::from_str::<
                crate::admin_cleanup_status::StdOptionalOptionalAdminCleanupStatusWhereMany,
            >(text.as_str())
            .map(crate::data_cleanup_status_filter::DataCleanupStatusFilter::from)
            .map(crate::data_table_filter::DataTableFilter::CleanupStatus),
            serde_json::from_str::<
                crate::admin_login_attempts::StdOptionalOptionalAdminLoginAttemptsWhereMany,
            >(text.as_str())
            .map(crate::data_login_attempts_filter::DataLoginAttemptsFilter::from)
            .map(crate::data_table_filter::DataTableFilter::LoginAttempts),
            serde_json::from_str::<
                crate::admin_rate_limits::StdOptionalOptionalAdminRateLimitsWhereMany,
            >(text.as_str())
            .map(crate::data_rate_limits_filter::DataRateLimitsFilter::from)
            .map(crate::data_table_filter::DataTableFilter::RateLimits),
            serde_json::from_str::<
                crate::admin_refresh_tokens::StdOptionalOptionalAdminRefreshTokensWhereMany,
            >(text.as_str())
            .map(crate::data_refresh_tokens_filter::DataRefreshTokensFilter::from)
            .map(crate::data_table_filter::DataTableFilter::RefreshTokens),
        ];
    assert!(
        filters
            .into_iter()
            .all(|parsed_filter| parsed_filter.is_ok_and(|filter| {
                let mut increment =
                    pg_crud_common::query_part_increment::QueryPartIncrement::from(7u64);
                let result = filter.query_part(&mut increment);
                matches!(
                    result,
                    Ok(pg_crud_common::query_part_fragment::QueryPartFragment { .. })
                ) && increment.get() == 7u64
                    && filter
                        .query_bind(
                            pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(
                                sqlx::query(constants_str::EMPTY),
                            ),
                        )
                        .is_ok_and(|bound_query| {
                            let mut query = bound_query.into_inner();
                            sqlx::Execute::take_arguments(&mut query).is_ok_and(|arguments| {
                                arguments.is_none_or(|arguments| {
                                    sqlx::Arguments::len(&arguments) == 0usize
                                })
                            })
                        })
            }))
    );
}

#[test]
fn test_generated_empty_filters_do_not_bind_query_arguments() {
    let text = serde_json::Value::Null.to_string();
    assert!(
        crate::admin_generated_table::AdminGeneratedTable::ALL
            .iter()
            .all(|table| {
                table
                    .parse_filter(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                        text.as_str(),
                    ))
                    .is_ok_and(|filter| {
                        let sqlx_postgres_query =
                            pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(
                                sqlx::query(constants_str::EMPTY),
                            );
                        filter
                            .query_bind(sqlx_postgres_query)
                            .is_ok_and(|bound_query| {
                                let mut query = bound_query.into_inner();
                                sqlx::Execute::take_arguments(&mut query).is_ok_and(|arguments| {
                                    arguments.is_none_or(|arguments| {
                                        sqlx::Arguments::len(&arguments) == 0usize
                                    })
                                })
                            })
                    })
            })
    );
}

#[test]
fn test_every_generated_table_identifier_filter_preserves_wire_type_and_parse_errors() {
    assert!(
        crate::admin_generated_table::AdminGeneratedTable::ALL
            .iter()
            .all(|table| {
                let value = stringify!(7);
                table
                    .field_contracts()
                    .as_ref()
                    .iter()
                    .any(|field| field.name().as_ref() == constants_str::SQL_NAMES_ID)
                    && table
                        .filter_value(
                            frontend_contract::form_field_name_ref::FormFieldNameRef::from(
                                constants_str::SQL_NAMES_ID,
                            ),
                            frontend_contract::form_value_ref::FormValueRef::from(value),
                        )
                        .is_some_and(|wire| {
                            wire.is_ok_and(|wire| {
                                serde_json::from_str::<serde_json::Value>(wire.as_ref())
                                    .is_ok_and(|json| json.as_i64() == Some(7i64))
                            })
                        })
                    && matches!(
                        table.filter_value(
                            frontend_contract::form_field_name_ref::FormFieldNameRef::from(
                                constants_str::SQL_NAMES_ID
                            ),
                            frontend_contract::form_value_ref::FormValueRef::from(
                                constants_str::UNKNOWN_ALT
                            ),
                        ),
                        Some(Err(
                            frontend_contract::form_value_error::FormValueError { .. }
                        ))
                    )
            })
    );
}

#[test]
fn test_every_generated_table_unknown_field_and_route_return_no_contract() {
    assert!(
        crate::admin_generated_table::AdminGeneratedTable::ALL
            .iter()
            .all(|table| {
                table
                    .filter_value(
                        frontend_contract::form_field_name_ref::FormFieldNameRef::from(
                            constants_str::UNKNOWN_ALT,
                        ),
                        frontend_contract::form_value_ref::FormValueRef::from(constants_str::ADMIN),
                    )
                    .is_none()
                    && table
                        .route_contract(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                            constants_str::UNKNOWN_ALT,
                        ))
                        .is_none()
            })
    );
}
#[test]
fn test_generated_table_read_routes_forward_exact_security_contracts() {
    [
        (crate::admin_generated_table::AdminGeneratedTable::AccessSessions, server_admin_contract::admin_rule::AdminRule::AccessSessionsRead, [crate::admin_access_sessions::AdminAccessSessions::read_route(), crate::admin_access_sessions::AdminAccessSessions::read_payload_example_route()]),
        (crate::admin_generated_table::AdminGeneratedTable::AuditLog, server_admin_contract::admin_rule::AdminRule::AuditLogRead, [crate::admin_audit_log::AdminAuditLog::read_route(), crate::admin_audit_log::AdminAuditLog::read_payload_example_route()]),
        (crate::admin_generated_table::AdminGeneratedTable::PermissionActions, server_admin_contract::admin_rule::AdminRule::PermissionActionsRead, [crate::admin_permission_actions::AdminPermissionActions::read_route(), crate::admin_permission_actions::AdminPermissionActions::read_payload_example_route()]),
        (crate::admin_generated_table::AdminGeneratedTable::PermissionResourceActions, server_admin_contract::admin_rule::AdminRule::PermissionResourceActionsRead, [crate::admin_permission_resource_actions::AdminPermissionResourceActions::read_route(), crate::admin_permission_resource_actions::AdminPermissionResourceActions::read_payload_example_route()]),
        (crate::admin_generated_table::AdminGeneratedTable::PermissionResources, server_admin_contract::admin_rule::AdminRule::PermissionResourcesRead, [crate::admin_permission_resources::AdminPermissionResources::read_route(), crate::admin_permission_resources::AdminPermissionResources::read_payload_example_route()]),
        (crate::admin_generated_table::AdminGeneratedTable::Roles, server_admin_contract::admin_rule::AdminRule::RolesRead, [crate::admin_roles::AdminRoles::read_route(), crate::admin_roles::AdminRoles::read_payload_example_route()]),
        (crate::admin_generated_table::AdminGeneratedTable::RoleRules, server_admin_contract::admin_rule::AdminRule::RoleRulesRead, [crate::admin_role_rules::AdminRoleRules::read_route(), crate::admin_role_rules::AdminRoleRules::read_payload_example_route()]),
        (crate::admin_generated_table::AdminGeneratedTable::UsersDatabaseRead, server_admin_contract::admin_rule::AdminRule::UsersRead, [crate::admin_users_database_read::AdminUsersDatabaseRead::read_route(), crate::admin_users_database_read::AdminUsersDatabaseRead::read_payload_example_route()]),
        (crate::admin_generated_table::AdminGeneratedTable::Rules, server_admin_contract::admin_rule::AdminRule::RulesRead, [crate::admin_rules::AdminRules::read_route(), crate::admin_rules::AdminRules::read_payload_example_route()]),
        (crate::admin_generated_table::AdminGeneratedTable::SystemSettings, server_admin_contract::admin_rule::AdminRule::SystemSettingsRead, [crate::admin_system_settings::AdminSystemSettings::read_route(), crate::admin_system_settings::AdminSystemSettings::read_payload_example_route()]),
        (crate::admin_generated_table::AdminGeneratedTable::UserRoles, server_admin_contract::admin_rule::AdminRule::UserRolesRead, [crate::admin_user_roles::AdminUserRoles::read_route(), crate::admin_user_roles::AdminUserRoles::read_payload_example_route()]),
    ].into_iter().fold((), |(), (admin_generated_table, admin_rule, paths)| {
        paths.into_iter().zip([
            frontend_contract::route_method::RouteMethod::Post,
            frontend_contract::route_method::RouteMethod::Get,
        ]).fold((), |(), (path, route_method)| {
            let contract = admin_generated_table.route_contract(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(path.as_ref()));
            assert!(contract.is_some());
            if let Some(contract) = contract {
                assert_eq!(contract.rule().map(server_admin_core::std_admin_str_ref::StdAdminStrRef::get), Some(admin_rule.as_str().get()));
                assert!(!contract.mutates().get());
                assert_eq!(contract.method(), route_method);
            }
        });
    });
}
