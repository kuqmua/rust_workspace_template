#[allow(
    clippy::single_call_fn,
    reason = "admin user roles remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn admin_user_roles(
    admin_user_summary: &server_admin_contract::admin_user_summary::AdminUserSummary,
    admin_users_page: &server_admin_contract::admin_users_page::AdminUsersPage,
) -> impl leptos::prelude::IntoView + use<> {
    let names = String::from(crate::join_text::join_text(
        admin_users_page
            .roles()
            .iter()
            .filter(|role| admin_user_summary.role_ids().contains(&role.id()))
            .map(server_admin_contract::admin_role_summary::AdminRoleSummary::name)
            .map(|name| name.as_ref().as_str()),
    ));
    leptos::view! { <crate::table_cell::TableCell data_label="roles">{names}</crate::table_cell::TableCell> }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::RenderHtml;

    #[test]
    fn test_user_roles_render_only_matching_names_in_catalog_order() {
        let assigned_role = server_admin_contract::admin_role_id::AdminRoleId::try_from(2i64)
            .expect(constants_str::DIAGNOSTIC_D8124A6F);
        let item = server_admin_contract::admin_user_summary::AdminUserSummary::new(
            server_admin_contract::admin_display_name::AdminDisplayName::try_from(String::from(
                constants_str::VALUE_3BC51062,
            ))
            .expect(constants_str::DIAGNOSTIC_63F1B9E4),
            server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
                .expect(constants_str::DIAGNOSTIC_9AC2E751),
            server_admin_contract::admin_bool::AdminBool::from(false),
            server_admin_contract::admin_login::AdminLogin::try_from(String::from(
                constants_str::VALUE_2BD806C9,
            ))
            .expect(constants_str::DIAGNOSTIC_4E70C31D),
            server_admin_contract::admin_role_ids::AdminRoleIds::try_from(vec![assigned_role])
                .expect(constants_str::DIAGNOSTIC_5B38D0A2),
        );
        let page = server_admin_contract::admin_users_page::AdminUsersPage::new(
            server_admin_contract::admin_user_summaries::AdminUserSummaries::try_from(vec![item])
                .expect(constants_str::DIAGNOSTIC_7F294CB8),
            server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(vec![
                server_admin_contract::admin_role_summary::AdminRoleSummary::new(
                    server_admin_contract::admin_role_id::AdminRoleId::try_from(constants_i64::ONE)
                        .expect(constants_str::VALUE_8B745867),
                    server_admin_contract::admin_bool::AdminBool::from(false),
                    server_admin_contract::admin_role_name::AdminRoleName::try_from(String::from(
                        constants_str::VALUE_3D094196,
                    ))
                    .expect(constants_str::VALUE_E6FE267E),
                    server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::default(),
                    server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::default(),
                ),
                server_admin_contract::admin_role_summary::AdminRoleSummary::new(
                    assigned_role,
                    server_admin_contract::admin_bool::AdminBool::from(false),
                    server_admin_contract::admin_role_name::AdminRoleName::try_from(String::from(
                        constants_str::VALUE_1553CC62,
                    ))
                    .expect(constants_str::VALUE_591027EA),
                    server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::default(),
                    server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::default(),
                ),
            ])
            .expect(constants_str::DIAGNOSTIC_B670DE23),
            server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
        );

        let html = crate::admin_user_roles::admin_user_roles(
            page.items()
                .first()
                .expect(constants_str::DIAGNOSTIC_FD36A81C),
            &page,
        )
        .to_html();
        assert!(html.contains(constants_str::VALUE_34E9C590));
        assert!(!html.contains(constants_str::VALUE_3D094196));
    }
}
