#[test]
fn test_static_pages() {
    let admin = crate::domain_types_ssr_tests::test_admin();
    let branding = crate::domain_types_ssr_tests::test_branding();
    let query = server_admin_contract::admin_table_query::AdminTableQuery::default();
    let permission_id =
        server_admin_contract::admin_permission_id::AdminPermissionId::try_from(7i64)
            .expect(constants_str::DIAGNOSTIC_6BC2A15E);
    let permissions = server_admin_contract::admin_permissions_page::AdminPermissionsPage::new(
        server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(
            vec![
                server_admin_contract::admin_permission_summary::AdminPermissionSummary::new(
                    permission_id,
                    server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
                        String::from(constants_str::VALUE_C6919F81),
                    )
                    .expect(constants_str::VALUE_8431554A),
                ),
            ],
        )
        .expect(constants_str::DIAGNOSTIC_0CA582E4),
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

    let role_id = server_admin_contract::admin_role_id::AdminRoleId::try_from(3i64)
        .expect(constants_str::DIAGNOSTIC_B751E0A4);
    let users = server_admin_contract::admin_users_page::AdminUsersPage::new(
        server_admin_contract::admin_user_summaries::AdminUserSummaries::try_from(vec![
            server_admin_contract::admin_user_summary::AdminUserSummary::new(
                server_admin_contract::admin_display_name::AdminDisplayName::try_from(
                    String::from(constants_str::VALUE_F0F7361D),
                )
                .expect(constants_str::VALUE_2A7FA5B7),
                server_admin_contract::admin_user_id::AdminUserId::try_from(2i64)
                    .expect(constants_str::VALUE_BE49A05A),
                server_admin_contract::admin_bool::AdminBool::from(true),
                server_admin_contract::admin_login::AdminLogin::try_from(String::from(
                    constants_str::VALUE_81B637D8,
                ))
                .expect(constants_str::VALUE_51266978),
                server_admin_contract::admin_role_ids::AdminRoleIds::try_from(vec![role_id])
                    .expect(constants_str::VALUE_53D69E69),
            ),
        ])
        .expect(constants_str::DIAGNOSTIC_39AD70E2),
        server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(vec![
            server_admin_contract::admin_role_summary::AdminRoleSummary::new(
                role_id,
                server_admin_contract::admin_bool::AdminBool::from(false),
                server_admin_contract::admin_role_name::AdminRoleName::try_from(String::from(
                    constants_str::VALUE_2D70999A,
                ))
                .expect(constants_str::VALUE_4DDA1CCE),
                server_admin_contract::admin_permission_ids::AdminPermissionIds::try_from(vec![
                    permission_id,
                ])
                .expect(constants_str::VALUE_A0034DA1),
            ),
        ])
        .expect(constants_str::DIAGNOSTIC_2A9F75C1),
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );
    let users_html = crate::render_users::render_users(&users, &query, &admin, &branding);
    assert!(users_html.as_ref().contains("data-label=\"login\""));
    assert!(users_html.as_ref().contains(">bob</td>"));
    assert!(users_html.as_ref().contains("data-label=\"banned\""));
    assert!(users_html.as_ref().contains(">true</td>"));
    assert!(users_html.as_ref().contains(">reviewer</td>"));

    let roles = server_admin_contract::admin_roles_page::AdminRolesPage::new(
        server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(
            users.roles().to_vec(),
        )
        .expect(constants_str::DIAGNOSTIC_7CE41B06),
        server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(
            permissions.items().to_vec(),
        )
        .expect(constants_str::DIAGNOSTIC_C306D98A),
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );
    let roles_html = crate::render_roles::render_roles(&roles, &query, &admin, &branding);
    assert!(roles_html.as_ref().contains("data-label=\"name\""));
    assert!(roles_html.as_ref().contains(">reviewer</td>"));
    assert!(roles_html.as_ref().contains(">users.read</td>"));

    let sessions = server_admin_contract::admin_sessions_page::AdminSessionsPage::new(
        server_admin_contract::admin_session_views::AdminSessionViews::try_from(vec![
            server_admin_contract::admin_session_view::AdminSessionView::new(
                server_admin_contract::admin_session_timestamp::AdminSessionTimestamp::try_from(
                    String::from(constants_str::VALUE_27A52C1B),
                )
                .expect(constants_str::VALUE_BDAF3F76),
                server_admin_contract::admin_session_timestamp::AdminSessionTimestamp::try_from(
                    String::from(constants_str::VALUE_ADCD791F),
                )
                .expect(constants_str::VALUE_87F569B4),
                server_admin_contract::admin_session_identifier::AdminSessionIdentifier::try_from(
                    String::from(constants_str::VALUE_84097828),
                )
                .expect(constants_str::VALUE_B8C5ABEC),
                server_admin_contract::admin_bool::AdminBool::from(true),
            ),
        ])
        .expect(constants_str::DIAGNOSTIC_BC30F861),
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
        crate::admin_ssr_text::AdminSsrText::try_from(String::from(constants_str::METRICS_ALT))
            .expect(constants_str::DIAGNOSTIC_E5A204BD),
        crate::admin_ssr_text::AdminSsrText::try_from(String::from(constants_str::VALUE_242C81E4))
            .expect(constants_str::DIAGNOSTIC_107CDE83),
    );
    assert!(public_text.as_ref().contains("&lt;ready&gt;"));
    let private_text = crate::render_text_page_with_access::render_text_page_with_access(
        server_admin_contract::admin_page::AdminPage::OpenApi,
        crate::admin_ssr_text::AdminSsrText::try_from(String::from(constants_str::VALUE_39732416))
            .expect(constants_str::DIAGNOSTIC_48A0FC36),
        crate::admin_ssr_text::AdminSsrText::try_from(String::from(constants_str::VALUE_95ADE925))
            .expect(constants_str::DIAGNOSTIC_B7D3640E),
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
