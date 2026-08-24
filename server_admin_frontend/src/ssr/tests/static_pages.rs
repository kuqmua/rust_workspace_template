#[cfg(test)]
use super::{test_admin, test_branding};

#[test]
fn typed_static_pages_render_rows_actions_roles_and_escaped_text() {
    let admin = test_admin();
    let branding = test_branding();
    let query = server_admin_contract::AdminTableQuery::default();
    let permission_id = server_admin_contract::AdminPermissionId::try_from(7i64).expect("6bc2a15e typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold");
    let permissions = server_admin_contract::AdminPermissionsPage::new(
        server_admin_contract::AdminPermissionSummaries::try_from(vec![
            server_admin_contract::AdminPermissionSummary::new(
                permission_id,
                server_admin_contract::AdminPermissionValue::try_from(String::from("users.read"))
                    .expect("9d7f0c42 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
            ),
        ])
        .expect("0ca582e4 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        server_admin_contract::AdminPageTotal::from(1u64),
    );
    let permissions_html =
        super::super::render_permissions(&permissions, &query, &admin, &branding);
    assert!(permissions_html.as_ref().contains("data-label=\"id\""));
    assert!(permissions_html.as_ref().contains(">7</td>"));
    assert!(permissions_html.as_ref().contains(">users.read</td>"));

    let role_id = server_admin_contract::AdminRoleId::try_from(3i64).expect("b751e0a4 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold");
    let users = server_admin_contract::AdminUsersPage::new(
        server_admin_contract::AdminUserSummaries::try_from(vec![
            server_admin_contract::AdminUserSummary::new(
                server_admin_contract::AdminDisplayName::try_from(String::from("Bob User"))
                    .expect("4ef37b81 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
                server_admin_contract::AdminUserId::try_from(2i64).expect("ea691d50 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
                server_admin_contract::AdminBool::from(true),
                server_admin_contract::AdminLogin::try_from(String::from("bob")).expect("72c54e9a typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
                server_admin_contract::AdminRoleIds::try_from(vec![role_id]).expect("1f84cb63 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
            ),
        ])
        .expect("39ad70e2 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        server_admin_contract::AdminRoleSummaries::try_from(vec![
            server_admin_contract::AdminRoleSummary::new(
                role_id,
                server_admin_contract::AdminBool::from(false),
                server_admin_contract::AdminRoleName::try_from(String::from("reviewer"))
                    .expect("d02b63f8 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
                server_admin_contract::AdminPermissionIds::try_from(vec![permission_id])
                    .expect("8561ce4d typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
            ),
        ])
        .expect("2a9f75c1 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        server_admin_contract::AdminPageTotal::from(1u64),
    );
    let users_html = super::super::render_users(&users, &query, &admin, &branding);
    assert!(users_html.as_ref().contains("data-label=\"login\""));
    assert!(users_html.as_ref().contains(">bob</td>"));
    assert!(users_html.as_ref().contains("data-label=\"banned\""));
    assert!(users_html.as_ref().contains(">true</td>"));
    assert!(users_html.as_ref().contains(">reviewer</td>"));

    let roles = server_admin_contract::AdminRolesPage::new(
        server_admin_contract::AdminRoleSummaries::try_from(users.roles().to_vec())
            .expect("7ce41b06 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        server_admin_contract::AdminPermissionSummaries::try_from(permissions.items().to_vec())
            .expect("c306d98a typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        server_admin_contract::AdminPageTotal::from(1u64),
    );
    let roles_html = super::super::render_roles(&roles, &query, &admin, &branding);
    assert!(roles_html.as_ref().contains("data-label=\"name\""));
    assert!(roles_html.as_ref().contains(">reviewer</td>"));
    assert!(roles_html.as_ref().contains(">users.read</td>"));

    let sessions = server_admin_contract::AdminSessionsPage::new(
        server_admin_contract::AdminSessionViews::try_from(vec![
            server_admin_contract::AdminSessionView::new(
                server_admin_contract::AdminSessionTimestamp::try_from(String::from(
                    "2026-08-01T10:00:00Z",
                ))
                .expect("6a4de195 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
                server_admin_contract::AdminSessionTimestamp::try_from(String::from(
                    "2026-08-02T10:00:00Z",
                ))
                .expect("f81c2b47 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
                server_admin_contract::AdminSessionIdentifier::try_from(String::from("session-1"))
                    .expect("04b9a7d2 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
                server_admin_contract::AdminBool::from(true),
            ),
        ])
        .expect("bc30f861 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        server_admin_contract::AdminPageTotal::from(1u64),
    );
    let sessions_html = super::super::render_sessions(&sessions, &query, &admin, &branding);
    assert!(sessions_html.as_ref().contains("value=\"session-1\""));
    assert!(
        sessions_html
            .as_ref()
            .contains("data-name=\"AlertDialogContent\"")
    );
    assert!(sessions_html.as_ref().contains("singlestage-dialog"));
    assert!(!sessions_html.as_ref().contains("command=\"show-modal\""));
    assert!(
        sessions_html
            .as_ref()
            .contains("data-name=\"AlertDialogHeader\"")
    );
    assert!(
        sessions_html
            .as_ref()
            .contains("data-name=\"AlertDialogFooter\"")
    );
    assert!(
        sessions_html
            .as_ref()
            .contains(">Confirm session revocation</label>")
    );

    let profile_html = super::super::render_profile(&admin, &branding);
    assert!(profile_html.as_ref().contains(">operator, auditor</p>"));
    assert!(profile_html.as_ref().contains("name=\"current_password\""));

    let public_text = super::super::render_text_page(
        server_admin_contract::AdminPage::Metrics,
        super::super::AdminSsrText::try_from(String::from("Metrics")).expect("e5a204bd typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        super::super::AdminSsrText::try_from(String::from("<ready>")).expect("107cde83 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
    );
    assert!(public_text.as_ref().contains("&lt;ready&gt;"));
    let private_text = super::super::render_text_page_with_access(
        server_admin_contract::AdminPage::OpenApi,
        super::super::AdminSsrText::try_from(String::from("Specification")).expect("48a0fc36 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        super::super::AdminSsrText::try_from(String::from("contract text")).expect("b7d3640e typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        &admin,
        &branding,
    );
    assert!(private_text.as_ref().contains(">contract text</pre>"));
    assert!(
        private_text
            .as_ref()
            .contains(server_admin_contract::AdminHtmlAction::SignOut.get())
    );
}
