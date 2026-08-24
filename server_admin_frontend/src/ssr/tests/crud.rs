#[test]
fn crud_pages_render_dedicated_forms_and_navigation() {
    let permissions = [
        server_admin_contract::AdminPermission::UsersUpdate,
        server_admin_contract::AdminPermission::UsersDelete,
        server_admin_contract::AdminPermission::RolesUpdate,
        server_admin_contract::AdminPermission::RolesDelete,
    ]
    .into_iter()
    .map(|permission| {
        server_admin_contract::AdminPermissionValue::try_from(permission.as_str().get().to_owned())
            .expect("b53ad55d crud_pages_render_dedicated_forms_and_navigation invariant must hold")
    })
    .collect::<Vec<_>>();
    let admin = server_admin_contract::AuthenticatedAdmin::new(
        server_admin_contract::AdminDisplayName::try_from(String::from("CRUD Admin")).expect(
            "ee23b99d crud_pages_render_dedicated_forms_and_navigation invariant must hold",
        ),
        server_admin_contract::AdminUserId::try_from(i64_constants::ONE).expect(
            "f56d7f68 crud_pages_render_dedicated_forms_and_navigation invariant must hold",
        ),
        server_admin_contract::AdminLogin::try_from(String::from("crud_admin")).expect(
            "67827f9a crud_pages_render_dedicated_forms_and_navigation invariant must hold",
        ),
        server_admin_contract::AdminPermissionValues::try_from(permissions).expect(
            "9a38c3da crud_pages_render_dedicated_forms_and_navigation invariant must hold",
        ),
        server_admin_contract::AdminRoleNames::try_from(Vec::new()).expect(
            "3bbf55bf crud_pages_render_dedicated_forms_and_navigation invariant must hold",
        ),
    );
    let branding = super::test_branding();
    let users = server_admin_contract::AdminUsersPage::new(
        server_admin_contract::AdminUserSummaries::try_from(vec![
            server_admin_contract::AdminUserSummary::new(
                server_admin_contract::AdminDisplayName::try_from(String::from("Managed User"))
                    .expect("6aedc5dd crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
                server_admin_contract::AdminUserId::try_from(2i64).expect("157804e9 crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
                server_admin_contract::AdminBool::from(false),
                server_admin_contract::AdminLogin::try_from(String::from("managed_user"))
                    .expect("a533b9db crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
                server_admin_contract::AdminRoleIds::try_from(Vec::new()).expect("124e4f65 crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
            ),
        ])
        .expect("53d4cc88 crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::AdminRoleSummaries::try_from(Vec::new()).expect("8de8dbde crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::AdminPageTotal::from(1u64),
    );
    let roles = server_admin_contract::AdminRolesPage::new(
        server_admin_contract::AdminRoleSummaries::try_from(vec![
            server_admin_contract::AdminRoleSummary::new(
                server_admin_contract::AdminRoleId::try_from(3i64).expect("03214ad5 crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
                server_admin_contract::AdminBool::from(false),
                server_admin_contract::AdminRoleName::try_from(String::from("managed_role"))
                    .expect("3db6d7a7 crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
                server_admin_contract::AdminPermissionIds::try_from(Vec::new()).expect("4a6a7b5a crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
            ),
        ])
        .expect("5ffb690c crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::AdminPermissionSummaries::try_from(Vec::new()).expect("b1a319f1 crud_pages_render_dedicated_forms_and_navigation invariant must hold"),
        server_admin_contract::AdminPageTotal::from(1u64),
    );

    let user_create = super::super::render_user_create(&admin, &branding);
    assert!(user_create.as_ref().contains("Create user"));
    assert!(
        user_create
            .as_ref()
            .contains(server_admin_contract::AdminHtmlAction::UserCreate.get())
    );
    let user_manage = super::super::render_user_manage(&users, &admin, &branding);
    assert!(user_manage.as_ref().contains("managed_user"));
    assert!(
        user_manage
            .as_ref()
            .contains(server_admin_contract::AdminHtmlAction::UserUpdate.get())
    );
    assert!(
        user_manage
            .as_ref()
            .contains(server_admin_contract::AdminHtmlAction::UserDelete.get())
    );

    let role_create = super::super::render_role_create(&admin, &branding);
    assert!(role_create.as_ref().contains("Create role"));
    let role_manage = super::super::render_role_manage(&roles, &admin, &branding);
    assert!(role_manage.as_ref().contains("managed_role"));
    assert!(
        role_manage
            .as_ref()
            .contains(server_admin_contract::AdminHtmlAction::RoleUpdate.get())
    );
    assert!(
        role_manage
            .as_ref()
            .contains(server_admin_contract::AdminHtmlAction::RoleDelete.get())
    );
}
