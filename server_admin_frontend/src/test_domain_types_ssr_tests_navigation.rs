#[test]
fn test_pagination_preserves_server_side_navigation() {
    let html = crate::admin_ssr_view_ext_tests::AdminSsrViewExt::render_admin_ssr(
        crate::table_pagination::table_pagination(
            server_admin_contract::admin_page::AdminPage::Users,
            &server_admin_contract::admin_table_query::AdminTableQuery::default(),
            server_admin_contract::admin_page_total::AdminPageTotal::from(101u64),
            None,
            None,
        ),
    );
    assert!(html.as_ref().contains(constants_str::VALUE_63C4C5D8));
    assert!(html.as_ref().contains(constants_str::VALUE_50B5BD8E));
    assert!(html.as_ref().contains(constants_str::VALUE_0AA8ABD0));
    assert!(html.as_ref().contains(constants_str::VALUE_C7A9349A));
    assert!(html.as_ref().contains(constants_str::VALUE_0666A453));
    assert!(html.as_ref().contains(constants_str::VALUE_51DD0CFC));
    assert!(html.as_ref().contains(constants_str::VALUE_47ABB6D8));
    assert!(!html.as_ref().contains(constants_str::VALUE_5D74223D));

    let table_filter =
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            Some(
                server_admin_contract::admin_filter_field::AdminFilterField::try_from(
                    String::from(constants_str::LOGIN),
                )
                .expect(constants_str::DIAGNOSTIC_7EB9A214),
            ),
            Some(frontend_contract::filter_operation::FilterOperation::Eq),
            Some(
                server_admin_contract::admin_filter_value::AdminFilterValue::try_from(
                    String::from(constants_str::VALUE_2BD806C9),
                )
                .expect(constants_str::DIAGNOSTIC_2629C095),
            ),
            None,
        );
    let filtered_html = crate::admin_ssr_view_ext_tests::AdminSsrViewExt::render_admin_ssr(
        crate::table_pagination::table_pagination(
            server_admin_contract::admin_page::AdminPage::Tables,
            &server_admin_contract::admin_table_query::AdminTableQuery::default(),
            server_admin_contract::admin_page_total::AdminPageTotal::from(101u64),
            Some(server_admin_contract::admin_data_table::AdminDataTable::RolePermissions),
            Some(&table_filter),
        ),
    );
    assert_eq!(
        filtered_html
            .as_ref()
            .matches(constants_str::VALUE_E7A7CF18)
            .count(),
        3usize
    );
    assert_eq!(
        filtered_html
            .as_ref()
            .matches(constants_str::VALUE_8D5B8BC9)
            .count(),
        3usize
    );
    assert_eq!(
        filtered_html
            .as_ref()
            .matches(constants_str::VALUE_8E0ECBA0)
            .count(),
        3usize
    );
    assert_eq!(
        filtered_html
            .as_ref()
            .matches(constants_str::VALUE_AC55DE88)
            .count(),
        3usize
    );
    assert!(
        !filtered_html
            .as_ref()
            .contains(constants_str::VALUE_155A24DE)
    );
    assert!(
        !filtered_html
            .as_ref()
            .contains(constants_str::VALUE_5EBA168E)
    );
}

#[test]
fn test_navigation_only_contains_accessible_pages() {
    let admin = server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
        server_admin_contract::admin_display_name::AdminDisplayName::try_from(
            constants_str::ADMIN.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_CDAE3E58),
        server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
            .expect(constants_str::DIAGNOSTIC_4FF30835),
        server_admin_contract::admin_login::AdminLogin::try_from(constants_str::ROOT.to_owned())
            .expect(constants_str::DIAGNOSTIC_9AE5B850),
        server_admin_contract::admin_permission_values::AdminPermissionValues::try_from(vec![
            server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
                server_admin_contract::admin_permission::AdminPermission::UsersRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::VALUE_0C631CF4),
            server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
                server_admin_contract::admin_permission::AdminPermission::TablesRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::VALUE_AAC52120),
            server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
                server_admin_contract::admin_permission::AdminPermission::AccessSessionsRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::VALUE_9CC34A06),
        ])
        .expect(constants_str::DIAGNOSTIC_E05CE0B9),
        server_admin_contract::admin_role_names::AdminRoleNames::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_F1EC0093),
    );
    let html = crate::render_admin_page_with_access::render_admin_page_with_access(
        server_admin_contract::admin_page::AdminPage::Users,
        crate::admin_ssr_html::AdminSsrHtml::try_from(String::new())
            .expect(constants_str::DIAGNOSTIC_AA3FA21E),
        Some(&admin),
        None,
    );
    assert!(
        html.as_ref()
            .contains(server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get())
    );
    assert!(
        !html
            .as_ref()
            .contains(server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles.get())
    );
    assert!(!html.as_ref().contains(
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Permissions.get()
    ));
    assert!(
        !html.as_ref().contains(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Settings.get()
        )
    );
    assert!(
        html.as_ref().contains(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Sessions.get()
        )
    );
    assert!(
        html.as_ref()
            .contains(server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile.get())
    );
    assert!(
        html.as_ref().contains(
            server_admin_contract::admin_data_table::AdminDataTable::AccessSessions
                .frontend_path()
                .as_ref()
        )
    );
    let users_table = html
        .as_ref()
        .find(constants_str::VALUE_A6A17075)
        .expect(constants_str::DIAGNOSTIC_7017FE5D);
    let sessions_table = html
        .as_ref()
        .find(constants_str::VALUE_B04C3167)
        .expect(constants_str::DIAGNOSTIC_9510971F);
    let profile_page = html
        .as_ref()
        .find(constants_str::VALUE_94661A00)
        .expect(constants_str::DIAGNOSTIC_21570A0C);
    let sessions_page = html
        .as_ref()
        .find(constants_str::VALUE_21207624)
        .expect(constants_str::DIAGNOSTIC_BA431A21);
    let sign_out = html
        .as_ref()
        .find(server_admin_contract::admin_html_action::AdminHtmlAction::SignOut.get())
        .expect(constants_str::DIAGNOSTIC_46D23E89);
    assert!(users_table < sessions_table);
    assert!(sessions_table < profile_page);
    assert!(profile_page < sessions_page);
    assert!(sessions_page < sign_out);
    assert!(
        html.as_ref().contains(
            server_admin_contract::admin_data_table::AdminDataTable::Users
                .frontend_path()
                .as_ref()
        )
    );
    assert!(!html.as_ref().contains(constants_str::VALUE_5EBA168E));
}

#[test]
fn test_sign_in_uses_server_side_color_without_logo() {
    let settings = server_admin_contract::admin_settings_view::AdminSettingsView::new(
        server_admin_contract::admin_default_route::AdminDefaultRoute::try_from(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Users
                .get()
                .to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_50FFE2FC),
        None,
        None,
        None,
        Some(
            server_admin_contract::admin_primary_color::AdminPrimaryColor::try_from(String::from(
                constants_str::VALUE_55F98A52,
            ))
            .expect(constants_str::DIAGNOSTIC_9C08C954),
        ),
        server_admin_contract::admin_site_name::AdminSiteName::try_from(String::from(
            constants_str::VALUE_EB57AFDB,
        ))
        .expect(constants_str::DIAGNOSTIC_0A28FDD7),
        None,
        None,
    );
    let branding =
        server_admin_contract::admin_branding_view::AdminBrandingView::from_settings(&settings);
    let html = crate::render_sign_in::render_sign_in(None, Some(&branding));
    assert!(!html.as_ref().contains(constants_str::VALUE_EB57AFDB));
    assert!(!html.as_ref().contains(constants_str::VALUE_E5FF7B61));
    assert!(!html.as_ref().contains(constants_str::VALUE_27B0E568));
    assert!(!html.as_ref().contains(constants_str::VALUE_44ED3CA7));
    assert!(html.as_ref().contains(constants_str::VALUE_ECD92E68));
    assert!(!html.as_ref().contains(constants_str::VALUE_5D74223D));
}
