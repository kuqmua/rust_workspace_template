#[test]
fn test_migrated_table_query_rejects_browser_pagination_bounds_and_duplicate_keys() {
    [
        (stringify!(limit), constants_str::VALUE_0),
        (stringify!(limit), constants_str::VALUE_101),
        (stringify!(limit), constants_str::VALUE_F1234D75),
        (stringify!(limit), constants_str::MIGRATION_0F255EC1),
        (stringify!(limit), constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        (stringify!(offset), constants_str::MIGRATION_1BAD6B8C),
        (stringify!(offset), constants_str::MIGRATION_6C1CA400),
        (stringify!(offset), constants_str::VALUE_F1234D75),
        (stringify!(offset), constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
    ]
    .into_iter()
    .fold((), |(), (key, value)| {
        let parsed = <crate::admin_table_query::AdminTableQuery as serde::Deserialize>::deserialize(
            serde::de::value::MapDeserializer::<_, serde_json::Error>::new(
                [(key, value)].into_iter(),
            ),
        );
        let serde_json_error = parsed.expect_err(constants_str::DIAGNOSTIC_C8FC7821);
        assert!(serde_json_error.is_data(), "{key}={value}");
    });
    [
        stringify!(limit),
        stringify!(offset),
        stringify!(search),
        stringify!(sort),
        stringify!(direction),
    ]
    .into_iter()
    .fold((), |(), key| {
        let parsed = <crate::admin_table_query::AdminTableQuery as serde::Deserialize>::deserialize(
            serde::de::value::MapDeserializer::<_, serde_json::Error>::new(
                [
                    (
                        key,
                        if key == stringify!(direction) {
                            crate::admin_sort_direction::AdminSortDirection::Ascending.as_ref()
                        } else {
                            constants_str::VALUE_1
                        },
                    ),
                    (
                        key,
                        if key == stringify!(direction) {
                            crate::admin_sort_direction::AdminSortDirection::Ascending.as_ref()
                        } else {
                            constants_str::VALUE_1
                        },
                    ),
                ]
                .into_iter(),
            ),
        );
        let serde_json_error = parsed.expect_err(constants_str::DIAGNOSTIC_7523D1C9);
        assert!(serde_json_error.is_data(), "{key}");
    });
}

#[test]
fn test_migrated_table_query_preserves_browser_defaults_and_pagination() {
    let defaults = <crate::admin_table_query::AdminTableQuery as serde::Deserialize>::deserialize(
        serde::de::value::MapDeserializer::<_, serde_json::Error>::new(std::iter::empty::<(
            &str,
            &str,
        )>()),
    )
    .expect(constants_str::DIAGNOSTIC_DC2BB206);
    assert_eq!(u16::from(defaults.limit()), 20u16);
    assert_eq!(u32::from(defaults.offset()), 0u32);
    [
        (constants_str::VALUE_1, constants_str::VALUE_0, 1u16, 0u32),
        (
            constants_str::VALUE_100,
            constants_str::VALUE_42,
            100u16,
            42u32,
        ),
    ]
    .into_iter()
    .fold(
        (),
        |(), (limit, offset, expected_limit, expected_offset)| {
            let parsed =
                <crate::admin_table_query::AdminTableQuery as serde::Deserialize>::deserialize(
                    serde::de::value::MapDeserializer::<_, serde_json::Error>::new(
                        [(stringify!(limit), limit), (stringify!(offset), offset)].into_iter(),
                    ),
                )
                .expect(constants_str::DIAGNOSTIC_AAAA7631);
            assert_eq!(u16::from(parsed.limit()), expected_limit);
            assert_eq!(u32::from(parsed.offset()), expected_offset);
        },
    );
}

#[test]
fn test_migrated_detail_page_catalog_ignores_list_queries_in_both_route_forms() {
    let detail_pages = [
        crate::admin_frontend_path::AdminFrontendPath::UserRead,
        crate::admin_frontend_path::AdminFrontendPath::UsersRead,
        crate::admin_frontend_path::AdminFrontendPath::RoleRead,
        crate::admin_frontend_path::AdminFrontendPath::RolesRead,
        crate::admin_frontend_path::AdminFrontendPath::RuleRead,
        crate::admin_frontend_path::AdminFrontendPath::RulesRead,
        crate::admin_frontend_path::AdminFrontendPath::PermissionActionRead,
        crate::admin_frontend_path::AdminFrontendPath::PermissionActionsRead,
        crate::admin_frontend_path::AdminFrontendPath::PermissionResourceActionRead,
        crate::admin_frontend_path::AdminFrontendPath::PermissionResourceActionsRead,
        crate::admin_frontend_path::AdminFrontendPath::PermissionResourceRead,
        crate::admin_frontend_path::AdminFrontendPath::PermissionResourcesRead,
        crate::admin_frontend_path::AdminFrontendPath::UserRoleRead,
        crate::admin_frontend_path::AdminFrontendPath::UserRolesRead,
        crate::admin_frontend_path::AdminFrontendPath::RoleRuleRead,
        crate::admin_frontend_path::AdminFrontendPath::RoleRulesRead,
        crate::admin_frontend_path::AdminFrontendPath::RefreshTokenRead,
        crate::admin_frontend_path::AdminFrontendPath::RefreshTokensRead,
        crate::admin_frontend_path::AdminFrontendPath::AccessSessionRead,
        crate::admin_frontend_path::AdminFrontendPath::AccessSessionsRead,
        crate::admin_frontend_path::AdminFrontendPath::LoginAttemptRead,
        crate::admin_frontend_path::AdminFrontendPath::LoginAttemptsRead,
        crate::admin_frontend_path::AdminFrontendPath::AuditLogRead,
        crate::admin_frontend_path::AdminFrontendPath::AuditLogsRead,
        crate::admin_frontend_path::AdminFrontendPath::SystemSettingRead,
        crate::admin_frontend_path::AdminFrontendPath::SystemSettingsRead,
        crate::admin_frontend_path::AdminFrontendPath::RateLimitRead,
        crate::admin_frontend_path::AdminFrontendPath::RateLimitsRead,
        crate::admin_frontend_path::AdminFrontendPath::CleanupStatusRead,
        crate::admin_frontend_path::AdminFrontendPath::CleanupStatusesRead,
    ];
    assert_eq!(detail_pages.len(), 30usize);
    detail_pages
        .into_iter()
        .fold((), |(), admin_frontend_path| {
            let registered = admin_frontend_path.get();
            let (prefix, parameter_tail) = registered
                .split_once('{')
                .expect(constants_str::DIAGNOSTIC_6C87AFD3);
            let (_, suffix) = parameter_tail
                .split_once('}')
                .expect(constants_str::DIAGNOSTIC_7ACFDFAA);
            let identifier = if matches!(
                admin_frontend_path,
                crate::admin_frontend_path::AdminFrontendPath::AccessSessionRead
                    | crate::admin_frontend_path::AdminFrontendPath::AccessSessionsRead
                    | crate::admin_frontend_path::AdminFrontendPath::RefreshTokenRead
                    | crate::admin_frontend_path::AdminFrontendPath::RefreshTokensRead
            ) {
                constants_str::TEST_REFRESH_TOKEN_ID
            } else {
                constants_str::VALUE_1
            };
            let path = format!("{prefix}{identifier}{suffix}");
            assert!(
                !bool::from(
                    crate::admin_path_uses_table_query::admin_path_uses_table_query(
                        crate::admin_page_path_ref::AdminPagePathRef::from(path.as_str())
                    )
                ),
                "{path}"
            );
        });
    crate::admin_data_table::AdminDataTable::ALL
        .into_iter()
        .fold((), |(), table| {
            let path = table.frontend_path();
            assert!(bool::from(
                crate::admin_path_uses_table_query::admin_path_uses_table_query(
                    crate::admin_page_path_ref::AdminPagePathRef::from(path.as_ref())
                )
            ));
        });
    [
        crate::admin_page::AdminPage::Profile,
        crate::admin_page::AdminPage::Health,
        crate::admin_page::AdminPage::Branding,
        crate::admin_page::AdminPage::Settings,
    ]
    .into_iter()
    .fold((), |(), page| assert!(!bool::from(page.uses_table_query())));
}

#[test]
fn test_migrated_settings_default_route_rejects_unknown_pages() {
    let error = crate::admin_default_route::AdminDefaultRoute::try_from(
        constants_str::MIGRATION_6DE4EE39.to_owned(),
    )
    .expect_err(constants_str::DIAGNOSTIC_4AFFD58B);
    assert!(!error.to_string().is_empty());
    let admin_default_route = crate::admin_default_route::AdminDefaultRoute::try_from(
        crate::admin_frontend_path::AdminFrontendPath::Users
            .get()
            .to_owned(),
    )
    .expect(constants_str::DIAGNOSTIC_2BCB227D);
    assert_eq!(
        admin_default_route.as_ref(),
        crate::admin_frontend_path::AdminFrontendPath::Users.get()
    );
}
