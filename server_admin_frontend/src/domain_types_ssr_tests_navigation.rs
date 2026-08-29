#[test]
fn pagination_preserves_server_side_navigation() {
    let html = crate::admin_ssr_view_ext::AdminSsrViewExt::render_admin_ssr(
        crate::table_pagination::table_pagination(
            server_admin_contract::admin_page::AdminPage::Users,
            &server_admin_contract::admin_table_query::AdminTableQuery::default(),
            server_admin_contract::admin_page_total::AdminPageTotal::from(101u64),
            None,
            None,
        ),
    );
    assert!(html.as_ref().contains("class=\"table-page-size\""));
    assert!(html.as_ref().contains("<span>Rows</span><input"));
    assert!(html.as_ref().contains("name=\"limit\""));
    assert!(html.as_ref().contains("type=\"number\""));
    assert!(html.as_ref().contains("name=\"offset\" value=\"20\""));
    assert!(html.as_ref().contains(" disabled"));
    assert!(html.as_ref().contains(">Previous</button>"));
    assert!(!html.as_ref().contains("<script"));

    let table_filter =
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            Some(
                server_admin_contract::admin_filter_field::AdminFilterField::try_from(
                    String::from(constants_str::catalog::LOGIN),
                )
                .expect("7eb9a214 pagination_preserves_server_side_navigation invariant must hold"),
            ),
            Some(frontend_contract::filter_operation::FilterOperation::Eq),
            Some(
                server_admin_contract::admin_filter_value::AdminFilterValue::try_from(
                    String::from(constants_str::test_fixtures::VALUE_2BD806C9),
                )
                .expect("2629c095 pagination_preserves_server_side_navigation invariant must hold"),
            ),
            None,
        );
    let filtered_html = crate::admin_ssr_view_ext::AdminSsrViewExt::render_admin_ssr(
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
            .matches("name=\"filter_field\" value=\"login\"")
            .count(),
        3usize
    );
    assert_eq!(
        filtered_html
            .as_ref()
            .matches("name=\"filter_operation\" value=\"eq\"")
            .count(),
        3usize
    );
    assert_eq!(
        filtered_html
            .as_ref()
            .matches("name=\"filter_value\" value=\"alice\"")
            .count(),
        3usize
    );
    assert_eq!(
        filtered_html
            .as_ref()
            .matches("action=\"/admin/role_permissions\"")
            .count(),
        3usize
    );
    assert!(!filtered_html.as_ref().contains("name=\"table\""));
    assert!(!filtered_html.as_ref().contains("?table="));
}

#[test]
fn navigation_only_contains_accessible_pages() {
    let admin = server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
        server_admin_contract::admin_display_name::AdminDisplayName::try_from(
            constants_str::catalog::ADMIN.to_owned(),
        )
        .expect("cdae3e58 navigation_only_contains_accessible_pages invariant must hold"),
        server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
            .expect("4ff30835 navigation_only_contains_accessible_pages invariant must hold"),
        server_admin_contract::admin_login::AdminLogin::try_from(
            constants_str::catalog::ROOT.to_owned(),
        )
        .expect("9ae5b850 navigation_only_contains_accessible_pages invariant must hold"),
        server_admin_contract::admin_permission_values::AdminPermissionValues::try_from(vec![
            server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
                server_admin_contract::admin_permission::AdminPermission::UsersRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::test_fixtures::VALUE_0C631CF4),
            server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
                server_admin_contract::admin_permission::AdminPermission::TablesRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::test_fixtures::VALUE_AAC52120),
            server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
                server_admin_contract::admin_permission::AdminPermission::AccessSessionsRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::test_fixtures::VALUE_9CC34A06),
        ])
        .expect("e05ce0b9 navigation_only_contains_accessible_pages invariant must hold"),
        server_admin_contract::admin_role_names::AdminRoleNames::try_from(Vec::new())
            .expect("f1ec0093 navigation_only_contains_accessible_pages invariant must hold"),
    );
    let html = crate::render_admin_page_with_access::render_admin_page_with_access(
        server_admin_contract::admin_page::AdminPage::Users,
        crate::admin_ssr_html::AdminSsrHtml::try_from(String::new())
            .expect("aa3fa21e navigation_only_contains_accessible_pages invariant must hold"),
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
        .find(constants_str::test_fixtures::VALUE_A6A17075)
        .expect("7017fe5d navigation_only_contains_accessible_pages invariant must hold");
    let sessions_table = html
        .as_ref()
        .find(constants_str::test_fixtures::VALUE_B04C3167)
        .expect("9510971f navigation_only_contains_accessible_pages invariant must hold");
    let profile_page = html
        .as_ref()
        .find(constants_str::test_fixtures::VALUE_94661A00)
        .expect("21570a0c navigation_only_contains_accessible_pages invariant must hold");
    let sessions_page = html
        .as_ref()
        .find(constants_str::test_fixtures::VALUE_21207624)
        .expect("ba431a21 navigation_only_contains_accessible_pages invariant must hold");
    let sign_out = html
        .as_ref()
        .find(server_admin_contract::admin_html_action::AdminHtmlAction::SignOut.get())
        .expect("46d23e89 navigation_only_contains_accessible_pages invariant must hold");
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
    assert!(!html.as_ref().contains("?table="));
}

#[test]
fn sign_in_uses_server_side_color_without_logo() {
    let settings = server_admin_contract::admin_settings_view::AdminSettingsView::new(
        server_admin_contract::admin_default_route::AdminDefaultRoute::try_from(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Users
                .get()
                .to_owned(),
        )
        .expect("50ffe2fc sign_in_uses_server_side_color_without_logo invariant must hold"),
        None,
        None,
        None,
        Some(
            server_admin_contract::admin_primary_color::AdminPrimaryColor::try_from(String::from(
                constants_str::test_fixtures::VALUE_55F98A52,
            ))
            .expect("9c08c954 sign_in_uses_server_side_color_without_logo invariant must hold"),
        ),
        server_admin_contract::admin_site_name::AdminSiteName::try_from(String::from(
            constants_str::test_fixtures::VALUE_EB57AFDB,
        ))
        .expect("0a28fdd7 sign_in_uses_server_side_color_without_logo invariant must hold"),
        None,
        None,
    );
    let branding =
        server_admin_contract::admin_branding_view::AdminBrandingView::from_settings(&settings);
    let html = crate::render_sign_in::render_sign_in(None, Some(&branding));
    assert!(!html.as_ref().contains("Custom Admin"));
    assert!(!html.as_ref().contains("auth-brand"));
    assert!(!html.as_ref().contains("brand-mark"));
    assert!(!html.as_ref().contains("brand-logo"));
    assert!(html.as_ref().contains("--accent:#123456"));
    assert!(!html.as_ref().contains("<script"));
}
