#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "CSR and SSR targets each compile one call site and Leptos cell rendering requires attribute traits in lexical scope"
)]

pub(crate) fn admin_user_roles(
    item: &server_admin_contract::AdminUserSummary,
    page: &server_admin_contract::AdminUsersPage,
) -> impl leptos::prelude::IntoView + use<> {
    let names = String::from(super::text::join_txt(
        page.roles()
            .iter()
            .filter(|role| item.role_ids().contains(&role.id()))
            .map(server_admin_contract::AdminRoleSummary::name)
            .map(|name| name.as_ref().as_str()),
    ));
    leptos::view! { <crate::ui::table::TableCell data_label="roles">{names}</crate::ui::table::TableCell> }
}

pub(crate) fn admin_role_permissions(
    item: &server_admin_contract::AdminRoleSummary,
    page: &server_admin_contract::AdminRolesPage,
) -> impl leptos::prelude::IntoView + use<> {
    let names = String::from(super::text::join_txt(
        page.permissions()
            .iter()
            .filter(|permission| item.permission_ids().contains(&permission.id()))
            .map(server_admin_contract::AdminPermissionSummary::name)
            .map(|name| name.as_ref().as_str()),
    ));
    leptos::view! { <crate::ui::table::TableCell data_label="permissions">{names}</crate::ui::table::TableCell> }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::RenderHtml;

    #[test]
    fn user_roles_render_only_matching_names_in_catalog_order() {
        let assigned_role = server_admin_contract::AdminRoleId::try_from(2i64).expect("d8124a6f");
        let item = server_admin_contract::AdminUserSummary::new(
            server_admin_contract::AdminDisplayName::try_from(String::from("Alice"))
                .expect("63f1b9e4"),
            server_admin_contract::AdminUserId::try_from(1i64).expect("9ac2e751"),
            server_admin_contract::AdminBool::from(false),
            server_admin_contract::AdminLogin::try_from(String::from("alice")).expect("4e70c31d"),
            server_admin_contract::AdminRoleIds::try_from(vec![assigned_role]).expect("5b38d0a2"),
        );
        let page = server_admin_contract::AdminUsersPage::new(
            server_admin_contract::AdminUserSummaries::try_from(vec![item]).expect("7f294cb8"),
            server_admin_contract::AdminRoleSummaries::try_from(vec![
                server_admin_contract::AdminRoleSummary::new(
                    server_admin_contract::AdminRoleId::try_from(1i64).expect("a014de95"),
                    server_admin_contract::AdminBool::from(false),
                    server_admin_contract::AdminRoleName::try_from(String::from("reader"))
                        .expect("2d6b15c9"),
                    server_admin_contract::AdminPermissionIds::try_from(Vec::new())
                        .expect("c9437f10"),
                ),
                server_admin_contract::AdminRoleSummary::new(
                    assigned_role,
                    server_admin_contract::AdminBool::from(false),
                    server_admin_contract::AdminRoleName::try_from(String::from("editor"))
                        .expect("e52c7a84"),
                    server_admin_contract::AdminPermissionIds::try_from(Vec::new())
                        .expect("18af630d"),
                ),
            ])
            .expect("b670de23"),
            server_admin_contract::AdminPageTotal::from(1u64),
        );

        let html =
            super::admin_user_roles(page.items().first().expect("fd36a81c"), &page).to_html();
        assert!(html.contains(">editor</td>"));
        assert!(!html.contains("reader"));
    }

    #[test]
    fn role_permissions_render_matching_names_with_stable_separator() {
        let first_permission =
            server_admin_contract::AdminPermissionId::try_from(1i64).expect("61bc809e");
        let second_permission =
            server_admin_contract::AdminPermissionId::try_from(2i64).expect("0f72ad46");
        let item = server_admin_contract::AdminRoleSummary::new(
            server_admin_contract::AdminRoleId::try_from(1i64).expect("392bd170"),
            server_admin_contract::AdminBool::from(false),
            server_admin_contract::AdminRoleName::try_from(String::from("operator"))
                .expect("d5a91f28"),
            server_admin_contract::AdminPermissionIds::try_from(vec![
                first_permission,
                second_permission,
            ])
            .expect("ab705ec1"),
        );
        let page = server_admin_contract::AdminRolesPage::new(
            server_admin_contract::AdminRoleSummaries::try_from(vec![item]).expect("f91e53b6"),
            server_admin_contract::AdminPermissionSummaries::try_from(vec![
                server_admin_contract::AdminPermissionSummary::new(
                    first_permission,
                    server_admin_contract::AdminPermissionValue::try_from(String::from(
                        "users.read",
                    ))
                    .expect("274cd6a9"),
                ),
                server_admin_contract::AdminPermissionSummary::new(
                    second_permission,
                    server_admin_contract::AdminPermissionValue::try_from(String::from(
                        "users.write",
                    ))
                    .expect("80e14fb3"),
                ),
            ])
            .expect("349ca278"),
            server_admin_contract::AdminPageTotal::from(1u64),
        );

        let html =
            super::admin_role_permissions(page.items().first().expect("719cb4e0"), &page).to_html();
        assert!(html.contains(">users.read, users.write</td>"));
    }
}
