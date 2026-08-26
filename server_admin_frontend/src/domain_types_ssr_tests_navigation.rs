#[cfg(test)]
use super::super::AdminSsrViewExt;

#[test]
fn pagination_preserves_server_side_navigation() {
    let html = super::super::table_pagination(
        server_admin_contract::domain_types::AdminPage::Users,
        &server_admin_contract::domain_types::AdminTableQuery::default(),
        server_admin_contract::domain_types::AdminPageTotal::from(101u64),
        None,
        None,
    )
    .render_admin_ssr();
    assert!(html.as_ref().contains("class=\"table-page-size\""));
    assert!(html.as_ref().contains("<span>Rows</span><input"));
    assert!(html.as_ref().contains("name=\"limit\""));
    assert!(html.as_ref().contains("type=\"number\""));
    assert!(html.as_ref().contains("name=\"offset\" value=\"20\""));
    assert!(html.as_ref().contains(" disabled"));
    assert!(html.as_ref().contains(">Previous</button>"));
    assert!(!html.as_ref().contains("<script"));

    let table_filter = server_admin_contract::domain_types::AdminDataTableFilterQuery::new(
        Some(
            server_admin_contract::domain_types::AdminFilterField::try_from(String::from(
                constants_str::LOGIN,
            ))
            .expect("7eb9a214 pagination_preserves_server_side_navigation invariant must hold"),
        ),
        Some(frontend_contract::domain_types::FilterOperation::Eq),
        Some(
            server_admin_contract::domain_types::AdminFilterValue::try_from(String::from(
                constants_str::VALUE_2BD806C9,
            ))
            .expect("2629c095 pagination_preserves_server_side_navigation invariant must hold"),
        ),
        None,
    );
    let filtered_html = super::super::table_pagination(
        server_admin_contract::domain_types::AdminPage::Tables,
        &server_admin_contract::domain_types::AdminTableQuery::default(),
        server_admin_contract::domain_types::AdminPageTotal::from(101u64),
        Some(server_admin_contract::domain_types::AdminDataTable::RolePermissions),
        Some(&table_filter),
    )
    .render_admin_ssr();
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
    let admin = server_admin_contract::domain_types::AuthenticatedAdmin::new(
        server_admin_contract::domain_types::AdminDisplayName::try_from(
            constants_str::ADMIN.to_owned(),
        )
        .expect("cdae3e58 navigation_only_contains_accessible_pages invariant must hold"),
        server_admin_contract::domain_types::AdminUserId::try_from(constants_i64::ONE)
            .expect("4ff30835 navigation_only_contains_accessible_pages invariant must hold"),
        server_admin_contract::domain_types::AdminLogin::try_from(constants_str::ROOT.to_owned())
            .expect("9ae5b850 navigation_only_contains_accessible_pages invariant must hold"),
        server_admin_contract::domain_types::AdminPermissionValues::try_from(vec![
            server_admin_contract::domain_types::AdminPermissionValue::try_from(
                server_admin_contract::domain_types::AdminPermission::UsersRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::VALUE_0C631CF4),
            server_admin_contract::domain_types::AdminPermissionValue::try_from(
                server_admin_contract::domain_types::AdminPermission::TablesRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::VALUE_AAC52120),
            server_admin_contract::domain_types::AdminPermissionValue::try_from(
                server_admin_contract::domain_types::AdminPermission::AccessSessionsRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::VALUE_9CC34A06),
        ])
        .expect("e05ce0b9 navigation_only_contains_accessible_pages invariant must hold"),
        server_admin_contract::domain_types::AdminRoleNames::try_from(Vec::new())
            .expect("f1ec0093 navigation_only_contains_accessible_pages invariant must hold"),
    );
    let html = super::super::render_admin_page_with_access(
        server_admin_contract::domain_types::AdminPage::Users,
        super::super::AdminSsrHtml::try_from(String::new())
            .expect("aa3fa21e navigation_only_contains_accessible_pages invariant must hold"),
        Some(&admin),
        None,
    );
    assert!(
        html.as_ref()
            .contains(server_admin_contract::domain_types::AdminFrontendPath::Users.get())
    );
    assert!(
        !html
            .as_ref()
            .contains(server_admin_contract::domain_types::AdminFrontendPath::Roles.get())
    );
    assert!(
        !html
            .as_ref()
            .contains(server_admin_contract::domain_types::AdminFrontendPath::Permissions.get())
    );
    assert!(
        !html
            .as_ref()
            .contains(server_admin_contract::domain_types::AdminFrontendPath::Settings.get())
    );
    assert!(
        html.as_ref()
            .contains(server_admin_contract::domain_types::AdminFrontendPath::Sessions.get())
    );
    assert!(
        html.as_ref()
            .contains(server_admin_contract::domain_types::AdminFrontendPath::Profile.get())
    );
    assert!(
        html.as_ref().contains(
            server_admin_contract::domain_types::AdminDataTable::AccessSessions
                .frontend_path()
                .as_ref()
        )
    );
    let users_table = html
        .as_ref()
        .find(constants_str::VALUE_A6A17075)
        .expect("7017fe5d navigation_only_contains_accessible_pages invariant must hold");
    let sessions_table = html
        .as_ref()
        .find(constants_str::VALUE_B04C3167)
        .expect("9510971f navigation_only_contains_accessible_pages invariant must hold");
    let profile_page = html
        .as_ref()
        .find(constants_str::VALUE_94661A00)
        .expect("21570a0c navigation_only_contains_accessible_pages invariant must hold");
    let sessions_page = html
        .as_ref()
        .find(constants_str::VALUE_21207624)
        .expect("ba431a21 navigation_only_contains_accessible_pages invariant must hold");
    let sign_out = html
        .as_ref()
        .find(server_admin_contract::domain_types::AdminHtmlAction::SignOut.get())
        .expect("46d23e89 navigation_only_contains_accessible_pages invariant must hold");
    assert!(users_table < sessions_table);
    assert!(sessions_table < profile_page);
    assert!(profile_page < sessions_page);
    assert!(sessions_page < sign_out);
    assert!(
        html.as_ref().contains(
            server_admin_contract::domain_types::AdminDataTable::Users
                .frontend_path()
                .as_ref()
        )
    );
    assert!(!html.as_ref().contains("?table="));
}

#[test]
fn sign_in_uses_server_side_color_without_logo() {
    let settings = server_admin_contract::domain_types::AdminSettingsView::new(
        server_admin_contract::domain_types::AdminDefaultRoute::try_from(
            server_admin_contract::domain_types::AdminFrontendPath::Users
                .get()
                .to_owned(),
        )
        .expect("50ffe2fc sign_in_uses_server_side_color_without_logo invariant must hold"),
        None,
        None,
        None,
        Some(
            server_admin_contract::domain_types::AdminPrimaryColor::try_from(String::from(
                constants_str::VALUE_55F98A52,
            ))
            .expect("9c08c954 sign_in_uses_server_side_color_without_logo invariant must hold"),
        ),
        server_admin_contract::domain_types::AdminSiteName::try_from(String::from(
            constants_str::VALUE_EB57AFDB,
        ))
        .expect("0a28fdd7 sign_in_uses_server_side_color_without_logo invariant must hold"),
        None,
        None,
    );
    let branding = server_admin_contract::domain_types::AdminBrandingView::from_settings(&settings);
    let html = super::super::render_sign_in(None, Some(&branding));
    assert!(!html.as_ref().contains("Custom Admin"));
    assert!(!html.as_ref().contains("auth-brand"));
    assert!(!html.as_ref().contains("brand-mark"));
    assert!(!html.as_ref().contains("brand-logo"));
    assert!(html.as_ref().contains("--accent:#123456"));
    assert!(!html.as_ref().contains("<script"));
}
