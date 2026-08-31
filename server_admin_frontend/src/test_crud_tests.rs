#[test]
fn test_crud() {
    let permissions = [
        server_admin_contract::admin_permission::AdminPermission::UsersUpdate,
        server_admin_contract::admin_permission::AdminPermission::UsersDelete,
        server_admin_contract::admin_permission::AdminPermission::RolesUpdate,
        server_admin_contract::admin_permission::AdminPermission::RolesDelete,
    ]
    .into_iter()
    .map(|permission| {
        server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
            permission.as_str().get().to_owned(),
        )
        .expect("b53ad55d crud_pages_render_dedicated_forms_and_navigation invariant must hold")
    })
    .collect::<Vec<_>>();
    let admin = server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
        server_admin_contract::admin_display_name::AdminDisplayName::try_from(String::from(
            constants_str::VALUE_BC3743C7,
        ))
        .expect("ee23b99d crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE).expect(
            "f56d7f68 crud_pages_render_dedicated_forms_and_navigation invariant must hold",
        ),
        server_admin_contract::admin_login::AdminLogin::try_from(String::from(
            constants_str::VALUE_09BBF5B6,
        ))
        .expect("67827f9a crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::admin_permission_values::AdminPermissionValues::try_from(
            permissions,
        )
        .expect("9a38c3da crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::admin_role_names::AdminRoleNames::try_from(Vec::new()).expect(
            "3bbf55bf crud_pages_render_dedicated_forms_and_navigation invariant must hold",
        ),
    );
    let branding = crate::domain_types_ssr_tests::test_branding();
    let users = server_admin_contract::admin_users_page::AdminUsersPage::new(
        server_admin_contract::admin_user_summaries::AdminUserSummaries::try_from(vec![
            server_admin_contract::admin_user_summary::AdminUserSummary::new(
                server_admin_contract::admin_display_name::AdminDisplayName::try_from(
                    String::from(constants_str::VALUE_4A66448F),
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
        .expect("53d4cc88 crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(Vec::new())
            .expect(
                "8de8dbde crud_pages_render_dedicated_forms_and_navigation invariant must hold",
            ),
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
                server_admin_contract::admin_permission_ids::AdminPermissionIds::try_from(
                    Vec::new(),
                )
                .expect(constants_str::VALUE_97520E5E),
            ),
        ])
        .expect("5ffb690c crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(
            Vec::new(),
        )
        .expect("b1a319f1 crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );

    let user_create = crate::render_user_create::render_user_create(&admin, &branding);
    assert!(user_create.as_ref().contains("Create user"));
    assert!(
        user_create
            .as_ref()
            .contains(server_admin_contract::admin_html_action::AdminHtmlAction::UserCreate.get())
    );
    let user_manage = crate::render_user_manage::render_user_manage(&users, &admin, &branding);
    assert!(user_manage.as_ref().contains("managed_user"));
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
    assert!(role_create.as_ref().contains("Create role"));
    let role_manage = crate::render_role_manage::render_role_manage(&roles, &admin, &branding);
    assert!(role_manage.as_ref().contains("managed_role"));
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
