#[test]
fn test_crud() {
    let rules = [
        server_admin_contract::admin_rule::AdminRule::UsersUpdate,
        server_admin_contract::admin_rule::AdminRule::UsersDelete,
        server_admin_contract::admin_rule::AdminRule::RolesUpdate,
        server_admin_contract::admin_rule::AdminRule::RolesDelete,
    ]
    .into_iter()
    .map(|rule| {
        server_admin_contract::admin_rule_value::AdminRuleValue::try_from(
            rule.as_str().get().to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_B53AD55D)
    })
    .collect::<Vec<_>>();
    let admin = server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
        server_admin_contract::admin_display_name::AdminDisplayName::try_from(String::from(
            constants_str::VALUE_BC3743C7,
        ))
        .expect(constants_str::DIAGNOSTIC_EE23B99D),
        server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
            .expect(constants_str::DIAGNOSTIC_F56D7F68),
        server_admin_contract::admin_login::AdminLogin::try_from(String::from(
            constants_str::VALUE_09BBF5B6,
        ))
        .expect(constants_str::DIAGNOSTIC_67827F9A),
        server_admin_contract::admin_rule_values::AdminRuleValues::try_from(rules)
            .expect(constants_str::DIAGNOSTIC_9A38C3DA),
        server_admin_contract::admin_role_names::AdminRoleNames::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_3BBF55BF),
    );
    let branding = crate::domain_types_ssr_tests::test_branding();
    let users = server_admin_contract::admin_users_page::AdminUsersPage::new(
        server_admin_contract::admin_user_summaries::AdminUserSummaries::try_from(vec![
            server_admin_contract::admin_user_summary::AdminUserSummary::new(
                server_admin_contract::admin_display_name::AdminDisplayName::try_from(
                    String::from(constants_str::ADMIN_DOCUMENT_UNSAFE_TITLE_FIXTURE),
                )
                .expect(constants_str::VALUE_7AB6D7B3),
                server_admin_contract::admin_user_id::AdminUserId::try_from(2i64)
                    .expect(constants_str::VALUE_D72B7CBC),
                server_admin_contract::admin_bool::AdminBool::from(false),
                server_admin_contract::admin_login::AdminLogin::try_from(String::from(
                    constants_str::VALUE_A7CEAFCE,
                ))
                .expect(constants_str::VALUE_28A32AE4),
                server_admin_contract::admin_role_ids::AdminRoleIds::try_from(Vec::new())
                    .expect(constants_str::VALUE_C97DFCA8),
            ),
        ])
        .expect(constants_str::DIAGNOSTIC_53D4CC88),
        server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_8DE8DBDE),
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );
    let roles = server_admin_contract::admin_roles_page::AdminRolesPage::new(
        server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(vec![
            server_admin_contract::admin_role_summary::AdminRoleSummary::new(
                server_admin_contract::admin_role_id::AdminRoleId::try_from(3i64)
                    .expect(constants_str::VALUE_3C8B6392),
                server_admin_contract::admin_bool::AdminBool::from(false),
                server_admin_contract::admin_role_name::AdminRoleName::try_from(String::from(
                    constants_str::VALUE_6186A0EE,
                ))
                .expect(constants_str::VALUE_5D15A9A0),
                server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::default(),
                server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::default(),
            ),
        ])
        .expect(constants_str::DIAGNOSTIC_5FFB690C),
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );
    let users_html = crate::render_users::render_users(
        &users,
        &server_admin_contract::admin_table_query::AdminTableQuery::default(),
        &admin,
        &branding,
    );
    let false_cell =
        constants_str::VALUE_AB48587B.replace(constants_str::TRUE, constants_str::FALSE);
    assert!(
        constants_str::ADMIN_DOCUMENT_ESCAPED_TITLE_FIXTURE
            .split_once('>')
            .and_then(|(_, content)| content.rsplit_once('<'))
            .is_some_and(|(escaped, _)| {
                users_html.as_ref().contains(
                    constants_str::VALUE_AB48587B
                        .replace(constants_str::TRUE, escaped)
                        .as_str(),
                )
            })
    );
    assert!(
        !users_html
            .as_ref()
            .contains(constants_str::ADMIN_DOCUMENT_UNSAFE_TITLE_FIXTURE)
    );
    assert!(
        !users_html
            .as_ref()
            .contains(constants_str::ADMIN_DOCUMENT_UNSAFE_SCRIPT_TAG)
    );
    assert!(users_html.as_ref().contains(false_cell.as_str()));
    assert!(!users_html.as_ref().contains(constants_str::VALUE_AB48587B));
    assert!(!users_html.as_ref().contains(
        server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersCreate.get()
    ));
    assert!(users.items().first().is_some_and(|user| {
        users_html.as_ref().contains(
            server_admin_contract::admin_route_path::AdminRoutePath::user_update_path(user.id())
                .as_ref(),
        )
    }));
    let roles_html = crate::render_roles::render_roles(
        &roles,
        &server_admin_contract::admin_table_query::AdminTableQuery::default(),
        &admin,
        &branding,
    );
    assert!(roles_html.as_ref().contains(false_cell.as_str()));
    assert!(!roles_html.as_ref().contains(constants_str::VALUE_AB48587B));
    assert!(!roles_html.as_ref().contains(
        server_admin_contract::admin_frontend_path::AdminFrontendPath::RolesCreate.get()
    ));
    assert!(roles.items().first().is_some_and(|role| {
        roles_html.as_ref().contains(
            server_admin_contract::admin_route_path::AdminRoutePath::role_update_path(role.id())
                .as_ref(),
        )
    }));
    let create_rule_result = server_admin_contract::admin_rule_value::AdminRuleValue::try_from(
        server_admin_contract::admin_rule::AdminRule::UsersCreate
            .as_str()
            .get()
            .to_owned(),
    );
    assert_eq!(create_rule_result.as_ref().err(), None);
    let Ok(create_rule) = create_rule_result else {
        return;
    };
    let role_create_rule_result = server_admin_contract::admin_rule_value::AdminRuleValue::try_from(
        server_admin_contract::admin_rule::AdminRule::RolesCreate
            .as_str()
            .get()
            .to_owned(),
    );
    assert_eq!(role_create_rule_result.as_ref().err(), None);
    let Ok(role_create_rule) = role_create_rule_result else {
        return;
    };
    let create_rules_result =
        server_admin_contract::admin_rule_values::AdminRuleValues::try_from(vec![
            create_rule,
            role_create_rule,
        ]);
    assert_eq!(create_rules_result.as_ref().err(), None);
    let Ok(create_rules) = create_rules_result else {
        return;
    };
    let create_roles_result =
        server_admin_contract::admin_role_names::AdminRoleNames::try_from(Vec::new());
    assert_eq!(create_roles_result.as_ref().err(), None);
    let Ok(create_roles) = create_roles_result else {
        return;
    };
    let create_admin = server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
        admin.display_name().clone(),
        *admin.id(),
        admin.login().clone(),
        create_rules,
        create_roles,
    );
    let create_users_html = crate::render_users::render_users(
        &users,
        &server_admin_contract::admin_table_query::AdminTableQuery::default(),
        &create_admin,
        &branding,
    );
    assert!(create_users_html.as_ref().contains(
        server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersCreate.get()
    ));
    assert!(users.items().first().is_some_and(|user| {
        !create_users_html.as_ref().contains(
            server_admin_contract::admin_route_path::AdminRoutePath::user_update_path(user.id())
                .as_ref(),
        )
    }));
    let create_roles_html = crate::render_roles::render_roles(
        &roles,
        &server_admin_contract::admin_table_query::AdminTableQuery::default(),
        &create_admin,
        &branding,
    );
    assert!(create_roles_html.as_ref().contains(
        server_admin_contract::admin_frontend_path::AdminFrontendPath::RolesCreate.get()
    ));
    assert!(roles.items().first().is_some_and(|role| {
        !create_roles_html.as_ref().contains(
            server_admin_contract::admin_route_path::AdminRoutePath::role_update_path(role.id())
                .as_ref(),
        )
    }));
    let user_create = crate::render_user_create::render_user_create(&admin, &branding);
    assert!(
        !user_create
            .as_ref()
            .contains(constants_str::ADMIN_UI_ADD_A_USER_ACCOUNT_WITH_INITIAL_CREDENTIALS)
    );
    assert!(
        user_create
            .as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_VALUE_F06DA128)
    );
    assert!(
        user_create
            .as_ref()
            .contains(server_admin_contract::admin_html_action::AdminHtmlAction::UserCreate.get())
    );
    let user_update = crate::render_user_update::render_user_update(*admin.id(), &admin, &branding);
    assert!(
        user_update.as_ref().contains(
            server_admin_contract::admin_route_path::AdminRoutePath::user_update_action_path(
                *admin.id()
            )
            .as_ref()
        )
    );
    assert!(user_update.as_ref().contains(constants_str::USER_ID));
    assert!(user_update.as_ref().contains(constants_str::HIDDEN));
    let user_manage = crate::render_user_manage::render_user_manage(&users, &admin, &branding);
    assert!(user_manage.as_ref().contains(constants_str::VALUE_A7CEAFCE));
    assert!(
        user_manage
            .as_ref()
            .contains(server_admin_contract::admin_html_action::AdminHtmlAction::UserUpdate.get())
    );
    assert!(
        user_manage
            .as_ref()
            .contains(server_admin_contract::admin_html_action::AdminHtmlAction::UserDelete.get())
    );

    let role_create = crate::render_role_create::render_role_create(&admin, &branding);
    assert!(
        role_create
            .as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_CREATE_ROLE)
    );
    let role_update = crate::render_role_update::render_role_update(None, &admin, &branding);
    assert!(
        role_update
            .as_ref()
            .contains(server_admin_contract::admin_html_action::AdminHtmlAction::RoleUpdate.get())
    );
    assert!(role_update.as_ref().contains(constants_str::ROLE_ID));
    let role_identifier =
        server_admin_contract::admin_role_id::AdminRoleId::try_from(constants_i64::ONE);
    assert_eq!(role_identifier.iter().count(), constants_usize::ONE);
    if let Ok(admin_role_id) = role_identifier {
        let html =
            crate::render_role_update::render_role_update(Some(admin_role_id), &admin, &branding);
        let action =
            server_admin_contract::admin_route_path::AdminRoutePath::role_update_action_path(
                admin_role_id,
            );
        assert!(html.as_ref().contains(action.as_ref()));
    }
    let role_manage = crate::render_role_manage::render_role_manage(&roles, &admin, &branding);
    assert!(role_manage.as_ref().contains(constants_str::VALUE_6186A0EE));
    assert!(
        role_manage
            .as_ref()
            .contains(server_admin_contract::admin_html_action::AdminHtmlAction::RoleUpdate.get())
    );
    assert!(
        role_manage
            .as_ref()
            .contains(server_admin_contract::admin_html_action::AdminHtmlAction::RoleDelete.get())
    );
}

#[test]
fn test_role_management_respects_independent_permissions_and_system_roles() {
    let base = crate::domain_types_ssr_tests::test_admin();
    let branding = crate::domain_types_ssr_tests::test_branding();
    assert!((0usize..4usize).all(|mask| {
        let rules = [
            server_admin_contract::admin_rule::AdminRule::RolesUpdate,
            server_admin_contract::admin_rule::AdminRule::RolesDelete,
        ].into_iter().enumerate().filter(|(index, _rule)| mask & (1usize << index) != 0usize)
            .map(|(_index, rule)| server_admin_contract::admin_rule_value::AdminRuleValue::try_from(rule.as_str().get().to_owned()))
            .collect::<Result<Vec<_>, _>>();
        rules.is_ok_and(|values| server_admin_contract::admin_rule_values::AdminRuleValues::try_from(values).is_ok_and(|admin_rule_values| {
            let Ok(admin_role_names) = server_admin_contract::admin_role_names::AdminRoleNames::try_from(base.roles().to_vec()) else { return false; };
            let admin = server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(base.display_name().clone(), *base.id(), base.login().clone(), admin_rule_values, admin_role_names);
            [false, true].into_iter().all(|is_system| {
                server_admin_contract::admin_role_id::AdminRoleId::try_from(constants_i64::ONE).is_ok_and(|admin_role_id| {
                    server_admin_contract::admin_role_name::AdminRoleName::try_from(constants_str::X.to_owned()).is_ok_and(|admin_role_name| {
                        let role = server_admin_contract::admin_role_summary::AdminRoleSummary::new(admin_role_id, server_admin_contract::admin_bool::AdminBool::from(is_system), admin_role_name, server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::default(), server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::default());
                        server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(vec![role]).is_ok_and(|admin_role_summaries| {
                            let page = server_admin_contract::admin_roles_page::AdminRolesPage::new(admin_role_summaries, server_admin_contract::admin_page_total::AdminPageTotal::from(1u64));
                            let html = crate::render_role_manage::render_role_manage(&page, &admin, &branding);
                            let has_action = |action| html.as_ref().split('"').zip(html.as_ref().split('"').skip(1usize)).any(|(name, value)| {
                                name.strip_suffix('=').is_some_and(|prefix| prefix.split_ascii_whitespace().last() == Some(stringify!(action))) && value == action
                            });
                            assert_eq!(has_action(server_admin_contract::admin_html_action::AdminHtmlAction::RoleUpdate.get()), mask & 1usize != 0usize);
                            assert_eq!(has_action(server_admin_contract::admin_html_action::AdminHtmlAction::RoleDelete.get()), mask & 2usize != 0usize && !is_system);
                            assert_eq!(html.as_ref().matches(format!(">{}<", constants_str::ADMIN_UI_SYSTEM_ROLE).as_str()).count(), usize::from(is_system));
                            assert_eq!(html.as_ref().matches(format!(">{}<", constants_str::ADMIN_UI_CUSTOM_ROLE).as_str()).count(), usize::from(!is_system));
                            true
                        })
                    })
                })
            })
        }))
    }));
}
