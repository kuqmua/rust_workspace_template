#[test]
fn test_static_pages() {
    let admin = crate::domain_types_ssr_tests::test_admin();
    let branding = crate::domain_types_ssr_tests::test_branding();
    let query = server_admin_contract::admin_table_query::AdminTableQuery::default();
    let rule_id = server_admin_contract::admin_rule_id::AdminRuleId::try_from(7i64)
        .expect(constants_str::DIAGNOSTIC_6BC2A15E);
    let rules = server_admin_contract::admin_rules_page::AdminRulesPage::new(
        server_admin_contract::admin_rule_summaries::AdminRuleSummaries::try_from(vec![
            server_admin_contract::admin_rule_summary::AdminRuleSummary::new(
                rule_id,
                server_admin_contract::admin_rule_value::AdminRuleValue::try_from(String::from(
                    constants_str::VALUE_C6919F81,
                ))
                .expect(constants_str::VALUE_8431554A),
                server_admin_contract::admin_rule_timestamp::AdminRuleTimestamp::from(
                    server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::try_from(
                        String::from(constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT),
                    )
                    .expect(constants_str::VALUE_A0034DA1),
                ),
            ),
        ])
        .expect(constants_str::DIAGNOSTIC_0CA582E4),
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );
    let rules_html =
        crate::render_admin_rules_page::render_admin_rules_page(&rules, &query, &admin, &branding);
    assert!(rules_html.as_ref().contains(constants_str::VALUE_E72513C4));
    assert!(rules_html.as_ref().contains(constants_str::VALUE_F424B0B2));
    assert!(rules_html.as_ref().contains(constants_str::VALUE_785F0083));

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
                server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::try_from(
                    String::from(constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT),
                )
                .expect(constants_str::VALUE_A0034DA1),
                server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::try_from(
                    String::from(constants_str::ADMIN_FIXTURE_SESSION_CREATED_AT),
                )
                .expect(constants_str::VALUE_A0034DA1),
            ),
        ])
        .expect(constants_str::DIAGNOSTIC_2A9F75C1),
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );
    let users_html = crate::render_users::render_users(&users, &query, &admin, &branding);
    assert!(users_html.as_ref().contains(constants_str::VALUE_A39478BF));
    assert!(users_html.as_ref().contains(constants_str::VALUE_45FF0D8E));
    assert!(users_html.as_ref().contains(constants_str::VALUE_CCC0FA20));
    assert!(users_html.as_ref().contains(constants_str::VALUE_AB48587B));
    assert!(users_html.as_ref().contains(constants_str::VALUE_DA7048B9));

    let roles = server_admin_contract::admin_roles_page::AdminRolesPage::new(
        server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(
            users.roles().to_vec(),
        )
        .expect(constants_str::DIAGNOSTIC_7CE41B06),
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );
    let roles_html = crate::render_roles::render_roles(&roles, &query, &admin, &branding);
    assert!(roles_html.as_ref().contains(constants_str::VALUE_91121F81));
    assert!(roles_html.as_ref().contains(constants_str::VALUE_DA7048B9));
    assert!(
        roles_html
            .as_ref()
            .contains(constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT)
    );
    assert!(
        roles_html
            .as_ref()
            .contains(constants_str::ADMIN_FIXTURE_SESSION_CREATED_AT)
    );

    let custom_role = roles
        .items()
        .first()
        .expect(constants_str::DIAGNOSTIC_E4AA599B)
        .clone();
    let path =
        server_admin_contract::admin_route_path::AdminRoutePath::role_update_path(custom_role.id());
    let system_role = server_admin_contract::admin_role_summary::AdminRoleSummary::new(
        custom_role.id(),
        server_admin_contract::admin_bool::AdminBool::from(true),
        custom_role.name().clone(),
        custom_role.created_at().clone(),
        custom_role.updated_at().clone(),
    );
    let mixed_roles = server_admin_contract::admin_roles_page::AdminRolesPage::new(
        server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(vec![
            custom_role,
            system_role,
        ])
        .expect(constants_str::DIAGNOSTIC_36A5B161),
        server_admin_contract::admin_page_total::AdminPageTotal::from(2u64),
    );
    let update_admin = server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
        admin.display_name().clone(),
        *admin.id(),
        admin.login().clone(),
        server_admin_contract::admin_rule_values::AdminRuleValues::try_from(vec![
            server_admin_contract::admin_rule_value::AdminRuleValue::try_from(
                server_admin_contract::admin_rule::AdminRule::RolesUpdate
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::DIAGNOSTIC_4A51AAB0),
        ])
        .expect(constants_str::DIAGNOSTIC_29910666),
        server_admin_contract::admin_role_names::AdminRoleNames::try_from(admin.roles().to_vec())
            .expect(constants_str::DIAGNOSTIC_A6ACFFA1),
    );
    let mixed_roles_html =
        crate::render_roles::render_roles(&mixed_roles, &query, &update_admin, &branding);
    assert_eq!(
        mixed_roles_html
            .as_ref()
            .matches(constants_str::VALUE_AB48587B)
            .count(),
        constants_usize::ONE
    );
    assert_eq!(
        mixed_roles_html
            .as_ref()
            .matches(
                constants_str::VALUE_AB48587B
                    .replace(constants_str::TRUE, constants_str::FALSE)
                    .as_str()
            )
            .count(),
        constants_usize::ONE
    );
    assert_eq!(
        mixed_roles_html.as_ref().matches(path.as_ref()).count(),
        constants_usize::TWO
    );

    let mixed_roles_manage_html =
        crate::render_role_manage::render_role_manage(&mixed_roles, &admin, &branding);
    assert!(
        mixed_roles_manage_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_SYSTEM_ROLE)
    );
    assert!(
        mixed_roles_manage_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_CUSTOM_ROLE)
    );
    let banned_users_manage_html =
        crate::render_user_manage::render_user_manage(&users, &admin, &branding);
    assert!(
        banned_users_manage_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_BANNED)
    );

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
    assert!(
        sessions_html
            .as_ref()
            .contains(constants_str::VALUE_0BDF914E)
    );
    assert!(
        sessions_html
            .as_ref()
            .contains(constants_str::VALUE_706A5FC3)
    );
    assert!(
        sessions_html
            .as_ref()
            .contains(constants_str::VALUE_3B0143B5)
    );
    assert!(
        sessions_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_VALUE_B86DD350)
    );

    let valid_session_id_result =
        server_admin_contract::admin_session_identifier::AdminSessionIdentifier::try_from(
            String::from(constants_str::TEST_ACCESS_SESSION_ID),
        );
    assert_eq!(valid_session_id_result.as_ref().err(), None);
    let Ok(valid_session_id) = valid_session_id_result else {
        return;
    };
    let read_session_id_result =
        server_admin_contract::admin_access_session_id::AdminAccessSessionId::try_from(
            String::from(constants_str::TEST_ACCESS_SESSION_ID),
        );
    assert_eq!(read_session_id_result.as_ref().err(), None);
    let Ok(read_session_id) = read_session_id_result else {
        return;
    };
    let read_path = server_admin_contract::admin_route_path::AdminRoutePath::from(read_session_id);
    assert!(!sessions_html.as_ref().contains(read_path.as_ref()));
    assert!(!sessions.items().is_empty());
    let Some(session_template) = sessions.items().first() else {
        return;
    };
    let valid_sessions_result =
        server_admin_contract::admin_session_views::AdminSessionViews::try_from(vec![
            server_admin_contract::admin_session_view::AdminSessionView::new(
                session_template.created_at().clone(),
                session_template.expires_at().clone(),
                valid_session_id,
                server_admin_contract::admin_bool::AdminBool::from(false),
            ),
        ]);
    assert_eq!(valid_sessions_result.as_ref().err(), None);
    let Ok(valid_sessions) = valid_sessions_result else {
        return;
    };
    let valid_sessions_page = server_admin_contract::admin_sessions_page::AdminSessionsPage::new(
        valid_sessions,
        server_admin_contract::admin_page_total::AdminPageTotal::from(constants_u64::ONE),
    );
    let valid_sessions_html = crate::render_admin_sessions_page::render_admin_sessions_page(
        &valid_sessions_page,
        &query,
        &admin,
        &branding,
    );
    assert!(valid_sessions_html.as_ref().contains(read_path.as_ref()));
    assert!(
        valid_sessions_html
            .as_ref()
            .contains(constants_str::TEST_ACCESS_SESSION_ID)
    );
    assert_eq!(
        valid_sessions_html
            .as_ref()
            .matches(&session_template.created_at().to_string())
            .count(),
        constants_usize::ONE
    );
    assert_eq!(
        valid_sessions_html
            .as_ref()
            .matches(&session_template.expires_at().to_string())
            .count(),
        constants_usize::ONE
    );

    let profile_html =
        crate::render_admin_profile_page::render_admin_profile_page(&admin, &branding);
    assert!(
        profile_html
            .as_ref()
            .contains(constants_str::VALUE_4645FB8E)
    );
    let public_text = crate::render_text_page::render_text_page(
        server_admin_contract::admin_page::AdminPage::Metrics,
        crate::admin_ssr_text::AdminSsrText::try_from(String::from(
            constants_str::ADMIN_UI_EXPECT_METRICS_ALT,
        ))
        .expect(constants_str::DIAGNOSTIC_E5A204BD),
        crate::admin_ssr_text::AdminSsrText::try_from(String::from(constants_str::VALUE_242C81E4))
            .expect(constants_str::DIAGNOSTIC_107CDE83),
    );
    assert!(public_text.as_ref().contains(constants_str::VALUE_5216C7F2));
    let private_text = crate::render_text_page_with_access::render_text_page_with_access(
        server_admin_contract::admin_page::AdminPage::OpenApi,
        crate::admin_ssr_text::AdminSsrText::try_from(String::from(constants_str::VALUE_39732416))
            .expect(constants_str::DIAGNOSTIC_48A0FC36),
        crate::admin_ssr_text::AdminSsrText::try_from(String::from(constants_str::VALUE_95ADE925))
            .expect(constants_str::DIAGNOSTIC_B7D3640E),
        &admin,
        &branding,
    );
    assert!(
        private_text
            .as_ref()
            .contains(constants_str::VALUE_2A72E715)
    );
    assert!(
        private_text
            .as_ref()
            .contains(server_admin_contract::admin_html_action::AdminHtmlAction::SignOut.get())
    );
}

#[test]
fn test_authenticated_version_page_preserves_version_paragraph_and_sign_out_action() {
    let admin = crate::domain_types_ssr_tests::test_admin();
    let branding = crate::domain_types_ssr_tests::test_branding();
    let title = crate::admin_ssr_text::AdminSsrText::try_from(String::from(
        constants_str::ADMIN_UI_VERSION,
    ))
    .unwrap_or_else(crate::admin_ssr_text::AdminSsrText::from);
    let text = crate::admin_ssr_text::AdminSsrText::try_from(String::from(constants_str::X))
        .unwrap_or_else(crate::admin_ssr_text::AdminSsrText::from);
    let html = crate::render_text_page_with_access::render_text_page_with_access(
        server_admin_contract::admin_page::AdminPage::Version,
        title,
        text,
        &admin,
        &branding,
    );
    assert!(
        html.as_ref()
            .contains(constants_str::ADMIN_VERSION_PARAGRAPH_FIXTURE)
    );
    assert!(
        html.as_ref()
            .contains(server_admin_contract::admin_html_action::AdminHtmlAction::SignOut.get())
    );
}

#[test]
fn test_authenticated_text_pages_preserve_openapi_class_selection_and_code_content() {
    let admin = crate::domain_types_ssr_tests::test_admin();
    let branding = crate::domain_types_ssr_tests::test_branding();
    [
        (server_admin_contract::admin_page::AdminPage::OpenApi, true),
        (server_admin_contract::admin_page::AdminPage::Metrics, false),
    ]
    .into_iter()
    .fold((), |(), (admin_page, openapi_class)| {
        let title = crate::admin_ssr_text::AdminSsrText::try_from(String::from(constants_str::X))
            .unwrap_or_else(crate::admin_ssr_text::AdminSsrText::from);
        let text = crate::admin_ssr_text::AdminSsrText::try_from(String::from(
            constants_str::VALUE_95ADE925,
        ))
        .unwrap_or_else(crate::admin_ssr_text::AdminSsrText::from);
        let html = crate::render_text_page_with_access::render_text_page_with_access(
            admin_page, title, text, &admin, &branding,
        );
        assert_eq!(
            html.as_ref()
                .contains(constants_str::ADMIN_OPENAPI_PAGE_CLASS_FIXTURE),
            openapi_class
        );
        assert_eq!(
            html.as_ref()
                .contains(constants_str::ADMIN_OPENAPI_PAGE_CLASS_PREFIX_FIXTURE),
            openapi_class
        );
        assert!(html.as_ref().contains(constants_str::VALUE_2A72E715));
    });
}
