#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "CSR and SSR targets each compile one call site and Leptos cell rendering requires attribute traits in lexical scope"
)]

pub(crate) fn admin_user_roles(
    item: &server_admin_contract::domain_types::AdminUserSummary,
    page: &server_admin_contract::domain_types::AdminUsersPage,
) -> impl leptos::prelude::IntoView + use<> {
    let names = String::from(super::text::join_text(
        page.roles()
            .iter()
            .filter(|role| item.role_ids().contains(&role.id()))
            .map(server_admin_contract::domain_types::AdminRoleSummary::name)
            .map(|name| name.as_ref().as_str()),
    ));
    leptos::view! { <crate::domain_types::with_owner::table::TableCell data_label="roles">{names}</crate::domain_types::with_owner::table::TableCell> }
}

pub(crate) fn admin_role_permissions(
    item: &server_admin_contract::domain_types::AdminRoleSummary,
    page: &server_admin_contract::domain_types::AdminRolesPage,
) -> impl leptos::prelude::IntoView + use<> {
    let names = String::from(super::text::join_text(
        page.permissions()
            .iter()
            .filter(|permission| item.permission_ids().contains(&permission.id()))
            .map(server_admin_contract::domain_types::AdminPermissionSummary::name)
            .map(|name| name.as_ref().as_str()),
    ));
    leptos::view! { <crate::domain_types::with_owner::table::TableCell data_label="permissions">{names}</crate::domain_types::with_owner::table::TableCell> }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::RenderHtml;

    #[test]
    fn user_roles_render_only_matching_names_in_catalog_order() {
        let assigned_role = server_admin_contract::domain_types::AdminRoleId::try_from(2i64).expect(
            "d8124a6f user_roles_render_only_matching_names_in_catalog_order invariant must hold",
        );
        let item = server_admin_contract::domain_types::AdminUserSummary::new(
            server_admin_contract::domain_types::AdminDisplayName::try_from(String::from(constants_str::VALUE_3BC51062))
                .expect("63f1b9e4 user_roles_render_only_matching_names_in_catalog_order invariant must hold"),
            server_admin_contract::domain_types::AdminUserId::try_from(constants_i64::ONE).expect("9ac2e751 user_roles_render_only_matching_names_in_catalog_order invariant must hold"),
            server_admin_contract::domain_types::AdminBool::from(false),
            server_admin_contract::domain_types::AdminLogin::try_from(String::from(constants_str::VALUE_2BD806C9)).expect("4e70c31d user_roles_render_only_matching_names_in_catalog_order invariant must hold"),
            server_admin_contract::domain_types::AdminRoleIds::try_from(vec![assigned_role]).expect("5b38d0a2 user_roles_render_only_matching_names_in_catalog_order invariant must hold"),
        );
        let page = server_admin_contract::domain_types::AdminUsersPage::new(
            server_admin_contract::domain_types::AdminUserSummaries::try_from(vec![item]).expect("7f294cb8 user_roles_render_only_matching_names_in_catalog_order invariant must hold"),
            server_admin_contract::domain_types::AdminRoleSummaries::try_from(vec![
                server_admin_contract::domain_types::AdminRoleSummary::new(
                    server_admin_contract::domain_types::AdminRoleId::try_from(constants_i64::ONE).expect(constants_str::VALUE_8B745867),
                    server_admin_contract::domain_types::AdminBool::from(false),
                    server_admin_contract::domain_types::AdminRoleName::try_from(String::from(constants_str::VALUE_3D094196))
                        .expect(constants_str::VALUE_E6FE267E),
                    server_admin_contract::domain_types::AdminPermissionIds::try_from(Vec::new())
                        .expect(constants_str::VALUE_6FA51050),
                ),
                server_admin_contract::domain_types::AdminRoleSummary::new(
                    assigned_role,
                    server_admin_contract::domain_types::AdminBool::from(false),
                    server_admin_contract::domain_types::AdminRoleName::try_from(String::from(constants_str::VALUE_1553CC62))
                        .expect(constants_str::VALUE_591027EA),
                    server_admin_contract::domain_types::AdminPermissionIds::try_from(Vec::new())
                        .expect(constants_str::VALUE_EA24866B),
                ),
            ])
            .expect("b670de23 user_roles_render_only_matching_names_in_catalog_order invariant must hold"),
            server_admin_contract::domain_types::AdminPageTotal::from(1u64),
        );

        let html =
            super::admin_user_roles(page.items().first().expect("fd36a81c user_roles_render_only_matching_names_in_catalog_order invariant must hold"), &page).to_html();
        assert!(html.contains(">editor</td>"));
        assert!(!html.contains("reader"));
    }

    #[test]
    fn role_permissions_render_matching_names_with_stable_separator() {
        let first_permission =
            server_admin_contract::domain_types::AdminPermissionId::try_from(constants_i64::ONE).expect("61bc809e role_permissions_render_matching_names_with_stable_separator invariant must hold");
        let second_permission =
            server_admin_contract::domain_types::AdminPermissionId::try_from(2i64).expect("0f72ad46 role_permissions_render_matching_names_with_stable_separator invariant must hold");
        let item = server_admin_contract::domain_types::AdminRoleSummary::new(
            server_admin_contract::domain_types::AdminRoleId::try_from(constants_i64::ONE).expect("392bd170 role_permissions_render_matching_names_with_stable_separator invariant must hold"),
            server_admin_contract::domain_types::AdminBool::from(false),
            server_admin_contract::domain_types::AdminRoleName::try_from(String::from(constants_str::PG_CRUD_OPERATOR_FIELD))
                .expect("d5a91f28 role_permissions_render_matching_names_with_stable_separator invariant must hold"),
            server_admin_contract::domain_types::AdminPermissionIds::try_from(vec![
                first_permission,
                second_permission,
            ])
            .expect("ab705ec1 role_permissions_render_matching_names_with_stable_separator invariant must hold"),
        );
        let page = server_admin_contract::domain_types::AdminRolesPage::new(
            server_admin_contract::domain_types::AdminRoleSummaries::try_from(vec![item]).expect("f91e53b6 role_permissions_render_matching_names_with_stable_separator invariant must hold"),
            server_admin_contract::domain_types::AdminPermissionSummaries::try_from(vec![
                server_admin_contract::domain_types::AdminPermissionSummary::new(
                    first_permission,
                    server_admin_contract::domain_types::AdminPermissionValue::try_from(String::from(
                        constants_str::VALUE_C6919F81,
                    ))
                    .expect(constants_str::VALUE_286F37C4),
                ),
                server_admin_contract::domain_types::AdminPermissionSummary::new(
                    second_permission,
                    server_admin_contract::domain_types::AdminPermissionValue::try_from(String::from(
                        constants_str::VALUE_8B8674FD,
                    ))
                    .expect(constants_str::VALUE_CD09FF18),
                ),
            ])
            .expect("349ca278 role_permissions_render_matching_names_with_stable_separator invariant must hold"),
            server_admin_contract::domain_types::AdminPageTotal::from(1u64),
        );

        let html =
            super::admin_role_permissions(page.items().first().expect("719cb4e0 role_permissions_render_matching_names_with_stable_separator invariant must hold"), &page).to_html();
        assert!(html.contains(">users.read, users.write</td>"));
    }
}
