#[cfg(test)]
use super::super::AdminSsrViewExt;

#[test]
fn pagination_preserves_server_side_navigation() {
    let html = super::super::table_pagination(
        server_admin_contract::AdminPage::Users,
        &server_admin_contract::AdminTableQuery::default(),
        server_admin_contract::AdminPageTotal::from(101u64),
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

    let table_filter = server_admin_contract::AdminDataTableFilterQuery::new(
        Some(
            server_admin_contract::AdminFilterField::try_from(String::from("login"))
                .expect("7eb9a214 pagination_preserves_server_side_navigation invariant must hold"),
        ),
        Some(frontend_contract::FilterOperation::Eq),
        Some(
            server_admin_contract::AdminFilterValue::try_from(String::from("alice"))
                .expect("2629c095 pagination_preserves_server_side_navigation invariant must hold"),
        ),
        None,
    );
    let filtered_html = super::super::table_pagination(
        server_admin_contract::AdminPage::Tables,
        &server_admin_contract::AdminTableQuery::default(),
        server_admin_contract::AdminPageTotal::from(101u64),
        Some(server_admin_contract::AdminDataTable::RolePermissions),
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
    let admin = server_admin_contract::AuthenticatedAdmin::new(
        server_admin_contract::AdminDisplayName::try_from(str_constants::ADMIN.to_owned())
            .expect("cdae3e58 navigation_only_contains_accessible_pages invariant must hold"),
        server_admin_contract::AdminUserId::try_from(i64_constants::ONE)
            .expect("4ff30835 navigation_only_contains_accessible_pages invariant must hold"),
        server_admin_contract::AdminLogin::try_from(str_constants::ROOT.to_owned())
            .expect("9ae5b850 navigation_only_contains_accessible_pages invariant must hold"),
        server_admin_contract::AdminPermissionValues::try_from(vec![
            server_admin_contract::AdminPermissionValue::try_from(
                server_admin_contract::AdminPermission::UsersRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect("6afb4194 navigation_only_contains_accessible_pages invariant must hold"),
            server_admin_contract::AdminPermissionValue::try_from(
                server_admin_contract::AdminPermission::TablesRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect("2c507520 navigation_only_contains_accessible_pages invariant must hold"),
            server_admin_contract::AdminPermissionValue::try_from(
                server_admin_contract::AdminPermission::AccessSessionsRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect("7e7147f6 navigation_only_contains_accessible_pages invariant must hold"),
        ])
        .expect("e05ce0b9 navigation_only_contains_accessible_pages invariant must hold"),
        server_admin_contract::AdminRoleNames::try_from(Vec::new())
            .expect("f1ec0093 navigation_only_contains_accessible_pages invariant must hold"),
    );
    let html = super::super::render_admin_page_with_access(
        server_admin_contract::AdminPage::Users,
        super::super::AdminSsrHtml::try_from(String::new())
            .expect("aa3fa21e navigation_only_contains_accessible_pages invariant must hold"),
        Some(&admin),
        None,
    );
    assert!(
        html.as_ref()
            .contains(server_admin_contract::AdminFrontendPath::Users.get())
    );
    assert!(
        !html
            .as_ref()
            .contains(server_admin_contract::AdminFrontendPath::Roles.get())
    );
    assert!(
        !html
            .as_ref()
            .contains(server_admin_contract::AdminFrontendPath::Permissions.get())
    );
    assert!(
        !html
            .as_ref()
            .contains(server_admin_contract::AdminFrontendPath::Settings.get())
    );
    assert!(
        html.as_ref()
            .contains(server_admin_contract::AdminFrontendPath::Sessions.get())
    );
    assert!(
        html.as_ref()
            .contains(server_admin_contract::AdminFrontendPath::Profile.get())
    );
    assert!(
        html.as_ref().contains(
            server_admin_contract::AdminDataTable::AccessSessions
                .frontend_path()
                .as_ref()
        )
    );
    let users_table = html
        .as_ref()
        .find("href=\"/admin/users\"")
        .expect("7017fe5d navigation_only_contains_accessible_pages invariant must hold");
    let sessions_table = html
        .as_ref()
        .find("href=\"/admin/access_sessions\"")
        .expect("9510971f navigation_only_contains_accessible_pages invariant must hold");
    let profile_page = html
        .as_ref()
        .find("href=\"/admin/profile\"")
        .expect("21570a0c navigation_only_contains_accessible_pages invariant must hold");
    let sessions_page = html
        .as_ref()
        .find("href=\"/admin/sessions\"")
        .expect("ba431a21 navigation_only_contains_accessible_pages invariant must hold");
    let sign_out = html
        .as_ref()
        .find(server_admin_contract::AdminHtmlAction::SignOut.get())
        .expect("46d23e89 navigation_only_contains_accessible_pages invariant must hold");
    assert!(users_table < sessions_table);
    assert!(sessions_table < profile_page);
    assert!(profile_page < sessions_page);
    assert!(sessions_page < sign_out);
    assert!(
        html.as_ref().contains(
            server_admin_contract::AdminDataTable::Users
                .frontend_path()
                .as_ref()
        )
    );
    assert!(!html.as_ref().contains("?table="));
}

#[test]
fn sign_in_uses_server_side_color_without_logo() {
    let settings = server_admin_contract::AdminSettingsView::new(
        server_admin_contract::AdminDefaultRoute::try_from(
            server_admin_contract::AdminFrontendPath::Users
                .get()
                .to_owned(),
        )
        .expect("50ffe2fc sign_in_uses_server_side_color_without_logo invariant must hold"),
        None,
        None,
        None,
        Some(
            server_admin_contract::AdminPrimaryColor::try_from(String::from("#123456"))
                .expect("9c08c954 sign_in_uses_server_side_color_without_logo invariant must hold"),
        ),
        server_admin_contract::AdminSiteName::try_from(String::from("Custom Admin"))
            .expect("0a28fdd7 sign_in_uses_server_side_color_without_logo invariant must hold"),
        None,
        None,
    );
    let branding = server_admin_contract::AdminBrandingView::from_settings(&settings);
    let html = super::super::render_sign_in(None, Some(&branding));
    assert!(!html.as_ref().contains("Custom Admin"));
    assert!(!html.as_ref().contains("auth-brand"));
    assert!(!html.as_ref().contains("brand-mark"));
    assert!(!html.as_ref().contains("brand-logo"));
    assert!(html.as_ref().contains("--accent:#123456"));
    assert!(!html.as_ref().contains("<script"));
}
