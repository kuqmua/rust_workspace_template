#[test]
fn crud() {
    let permissions = [
        server_admin_contract::domain_types::AdminPermission::UsersUpdate,
        server_admin_contract::domain_types::AdminPermission::UsersDelete,
        server_admin_contract::domain_types::AdminPermission::RolesUpdate,
        server_admin_contract::domain_types::AdminPermission::RolesDelete,
    ]
    .into_iter()
    .map(|permission| {
        server_admin_contract::domain_types::AdminPermissionValue::try_from(
            permission.as_str().get().to_owned(),
        )
        .expect("b53ad55d crud_pages_render_dedicated_forms_and_navigation invariant must hold")
    })
    .collect::<Vec<_>>();
    let admin = server_admin_contract::domain_types::AuthenticatedAdmin::new(
        server_admin_contract::domain_types::AdminDisplayName::try_from(String::from(
            constants_str::VALUE_BC3743C7,
        ))
        .expect("ee23b99d crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::domain_types::AdminUserId::try_from(constants_i64::ONE).expect(
            "f56d7f68 crud_pages_render_dedicated_forms_and_navigation invariant must hold",
        ),
        server_admin_contract::domain_types::AdminLogin::try_from(String::from(
            constants_str::VALUE_09BBF5B6,
        ))
        .expect("67827f9a crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::domain_types::AdminPermissionValues::try_from(permissions).expect(
            "9a38c3da crud_pages_render_dedicated_forms_and_navigation invariant must hold",
        ),
        server_admin_contract::domain_types::AdminRoleNames::try_from(Vec::new()).expect(
            "3bbf55bf crud_pages_render_dedicated_forms_and_navigation invariant must hold",
        ),
    );
    let branding = super::test_branding();
    let users = server_admin_contract::domain_types::AdminUsersPage::new(
        server_admin_contract::domain_types::AdminUserSummaries::try_from(vec![
            server_admin_contract::domain_types::AdminUserSummary::new(
                server_admin_contract::domain_types::AdminDisplayName::try_from(String::from(
                    constants_str::VALUE_4A66448F,
                ))
                .expect(constants_str::VALUE_7AB6D7B3),
                server_admin_contract::domain_types::AdminUserId::try_from(2i64)
                    .expect(constants_str::VALUE_D72B7CBC),
                server_admin_contract::domain_types::AdminBool::from(false),
                server_admin_contract::domain_types::AdminLogin::try_from(String::from(
                    constants_str::VALUE_A7CEAFCE,
                ))
                .expect(constants_str::VALUE_28A32AE4),
                server_admin_contract::domain_types::AdminRoleIds::try_from(Vec::new())
                    .expect(constants_str::VALUE_C97DFCA8),
            ),
        ])
        .expect("53d4cc88 crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::domain_types::AdminRoleSummaries::try_from(Vec::new()).expect(
            "8de8dbde crud_pages_render_dedicated_forms_and_navigation invariant must hold",
        ),
        server_admin_contract::domain_types::AdminPageTotal::from(1u64),
    );
    let roles = server_admin_contract::domain_types::AdminRolesPage::new(
        server_admin_contract::domain_types::AdminRoleSummaries::try_from(vec![
            server_admin_contract::domain_types::AdminRoleSummary::new(
                server_admin_contract::domain_types::AdminRoleId::try_from(3i64)
                    .expect(constants_str::VALUE_3C8B6392),
                server_admin_contract::domain_types::AdminBool::from(false),
                server_admin_contract::domain_types::AdminRoleName::try_from(String::from(
                    constants_str::VALUE_6186A0EE,
                ))
                .expect(constants_str::VALUE_5D15A9A0),
                server_admin_contract::domain_types::AdminPermissionIds::try_from(Vec::new())
                    .expect(constants_str::VALUE_97520E5E),
            ),
        ])
        .expect("5ffb690c crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::domain_types::AdminPermissionSummaries::try_from(Vec::new()).expect(
            "b1a319f1 crud_pages_render_dedicated_forms_and_navigation invariant must hold",
        ),
        server_admin_contract::domain_types::AdminPageTotal::from(1u64),
    );

    let user_create = crate::render_user_create(&admin, &branding);
    assert!(user_create.as_ref().contains("Create user"));
    assert!(
        user_create
            .as_ref()
            .contains(server_admin_contract::domain_types::AdminHtmlAction::UserCreate.get())
    );
    let user_manage = crate::render_user_manage(&users, &admin, &branding);
    assert!(user_manage.as_ref().contains("managed_user"));
    assert!(
        user_manage
            .as_ref()
            .contains(server_admin_contract::domain_types::AdminHtmlAction::UserUpdate.get())
    );
    assert!(
        user_manage
            .as_ref()
            .contains(server_admin_contract::domain_types::AdminHtmlAction::UserDelete.get())
    );

    let role_create = crate::render_role_create(&admin, &branding);
    assert!(role_create.as_ref().contains("Create role"));
    let role_manage = crate::render_role_manage(&roles, &admin, &branding);
    assert!(role_manage.as_ref().contains("managed_role"));
    assert!(
        role_manage
            .as_ref()
            .contains(server_admin_contract::domain_types::AdminHtmlAction::RoleUpdate.get())
    );
    assert!(
        role_manage
            .as_ref()
            .contains(server_admin_contract::domain_types::AdminHtmlAction::RoleDelete.get())
    );
}
