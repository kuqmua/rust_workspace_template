#[test]
fn test_static_pages() {
    let admin = crate::domain_types_ssr_tests::test_admin();
    let branding = crate::domain_types_ssr_tests::test_branding();
    let query = server_admin_contract::admin_table_query::AdminTableQuery::default();
    let permission_id = server_admin_contract::admin_permission_id::AdminPermissionId::try_from(7i64).expect("6bc2a15e typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold");
    let permissions = server_admin_contract::admin_permissions_page::AdminPermissionsPage::new(
        server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(vec![
            server_admin_contract::admin_permission_summary::AdminPermissionSummary::new(
                permission_id,
                server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(String::from(constants_str::VALUE_C6919F81))
                    .expect(constants_str::VALUE_8431554A),
            ),
        ])
        .expect("0ca582e4 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );
    let permissions_html = crate::render_admin_permissions_page::render_admin_permissions_page(
        &permissions,
        &query,
        &admin,
        &branding,
    );
    assert!(permissions_html.as_ref().contains("data-label=\"id\""));
    assert!(permissions_html.as_ref().contains(">7</td>"));
    assert!(permissions_html.as_ref().contains(">users.read</td>"));

    let role_id = server_admin_contract::admin_role_id::AdminRoleId::try_from(3i64).expect("b751e0a4 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold");
    let users = server_admin_contract::admin_users_page::AdminUsersPage::new(
        server_admin_contract::admin_user_summaries::AdminUserSummaries::try_from(vec![
            server_admin_contract::admin_user_summary::AdminUserSummary::new(
                server_admin_contract::admin_display_name::AdminDisplayName::try_from(String::from(constants_str::VALUE_F0F7361D))
                    .expect(constants_str::VALUE_2A7FA5B7),
                server_admin_contract::admin_user_id::AdminUserId::try_from(2i64).expect(constants_str::VALUE_BE49A05A),
                server_admin_contract::admin_bool::AdminBool::from(true),
                server_admin_contract::admin_login::AdminLogin::try_from(String::from(constants_str::VALUE_81B637D8)).expect(constants_str::VALUE_51266978),
                server_admin_contract::admin_role_ids::AdminRoleIds::try_from(vec![role_id]).expect(constants_str::VALUE_53D69E69),
            ),
        ])
        .expect("39ad70e2 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(vec![
            server_admin_contract::admin_role_summary::AdminRoleSummary::new(
                role_id,
                server_admin_contract::admin_bool::AdminBool::from(false),
                server_admin_contract::admin_role_name::AdminRoleName::try_from(String::from(constants_str::VALUE_2D70999A))
                    .expect(constants_str::VALUE_4DDA1CCE),
                server_admin_contract::admin_permission_ids::AdminPermissionIds::try_from(vec![permission_id])
                    .expect(constants_str::VALUE_A0034DA1),
            ),
        ])
        .expect("2a9f75c1 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );
    let users_html = crate::render_users::render_users(&users, &query, &admin, &branding);
    assert!(users_html.as_ref().contains("data-label=\"login\""));
    assert!(users_html.as_ref().contains(">bob</td>"));
    assert!(users_html.as_ref().contains("data-label=\"banned\""));
    assert!(users_html.as_ref().contains(">true</td>"));
    assert!(users_html.as_ref().contains(">reviewer</td>"));

    let roles = server_admin_contract::admin_roles_page::AdminRolesPage::new(
        server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(users.roles().to_vec())
            .expect("7ce41b06 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(permissions.items().to_vec())
            .expect("c306d98a typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );
    let roles_html = crate::render_roles::render_roles(&roles, &query, &admin, &branding);
    assert!(roles_html.as_ref().contains("data-label=\"name\""));
    assert!(roles_html.as_ref().contains(">reviewer</td>"));
    assert!(roles_html.as_ref().contains(">users.read</td>"));

    let sessions = server_admin_contract::admin_sessions_page::AdminSessionsPage::new(
        server_admin_contract::admin_session_views::AdminSessionViews::try_from(vec![
            server_admin_contract::admin_session_view::AdminSessionView::new(
                server_admin_contract::admin_session_timestamp::AdminSessionTimestamp::try_from(String::from(
                    constants_str::VALUE_27A52C1B,
                ))
                .expect(constants_str::VALUE_BDAF3F76),
                server_admin_contract::admin_session_timestamp::AdminSessionTimestamp::try_from(String::from(
                    constants_str::VALUE_ADCD791F,
                ))
                .expect(constants_str::VALUE_87F569B4),
                server_admin_contract::admin_session_identifier::AdminSessionIdentifier::try_from(String::from(constants_str::VALUE_84097828))
                    .expect(constants_str::VALUE_B8C5ABEC),
                server_admin_contract::admin_bool::AdminBool::from(true),
            ),
        ])
        .expect("bc30f861 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );
    let sessions_html = crate::render_admin_sessions_page::render_admin_sessions_page(
        &sessions, &query, &admin, &branding,
    );
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

    let profile_html =
        crate::render_admin_profile_page::render_admin_profile_page(&admin, &branding);
    assert!(profile_html.as_ref().contains(">operator, auditor</p>"));
    assert!(profile_html.as_ref().contains("name=\"current_password\""));

    let public_text = crate::render_text_page::render_text_page(
        server_admin_contract::admin_page::AdminPage::Metrics,
        crate::admin_ssr_text::AdminSsrText::try_from(String::from(constants_str::METRICS_ALT)).expect("e5a204bd typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        crate::admin_ssr_text::AdminSsrText::try_from(String::from(constants_str::VALUE_242C81E4)).expect("107cde83 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
    );
    assert!(public_text.as_ref().contains("&lt;ready&gt;"));
    let private_text = crate::render_text_page_with_access::render_text_page_with_access(
        server_admin_contract::admin_page::AdminPage::OpenApi,
        crate::admin_ssr_text::AdminSsrText::try_from(String::from(constants_str::VALUE_39732416)).expect("48a0fc36 typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        crate::admin_ssr_text::AdminSsrText::try_from(String::from(constants_str::VALUE_95ADE925)).expect("b7d3640e typed_static_pages_render_rows_actions_roles_and_escaped_text invariant must hold"),
        &admin,
        &branding,
    );
    assert!(private_text.as_ref().contains(">contract text</pre>"));
    assert!(
        private_text
            .as_ref()
            .contains(server_admin_contract::admin_html_action::AdminHtmlAction::SignOut.get())
    );
}
