#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(crate) fn admin_user_roles(
    item: &server_admin_contract::admin_user_summary::AdminUserSummary,
    page: &server_admin_contract::admin_users_page::AdminUsersPage,
) -> impl leptos::prelude::IntoView + use<> {
    let names = String::from(crate::join_text::join_text(
        page.roles()
            .iter()
            .filter(|role| item.role_ids().contains(&role.id()))
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
                    server_admin_contract::admin_permission_ids::AdminPermissionIds::try_from(
                        Vec::new(),
                    )
                    .expect(constants_str::VALUE_6FA51050),
                ),
                server_admin_contract::admin_role_summary::AdminRoleSummary::new(
                    assigned_role,
                    server_admin_contract::admin_bool::AdminBool::from(false),
                    server_admin_contract::admin_role_name::AdminRoleName::try_from(String::from(
                        constants_str::VALUE_1553CC62,
                    ))
                    .expect(constants_str::VALUE_591027EA),
                    server_admin_contract::admin_permission_ids::AdminPermissionIds::try_from(
                        Vec::new(),
                    )
                    .expect(constants_str::VALUE_EA24866B),
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

    #[test]
    fn test_role_permissions_render_matching_names_with_stable_separator() {
        let first_permission =
            server_admin_contract::admin_permission_id::AdminPermissionId::try_from(
                constants_i64::ONE,
            )
            .expect(constants_str::DIAGNOSTIC_61BC809E);
        let second_permission =
            server_admin_contract::admin_permission_id::AdminPermissionId::try_from(2i64)
                .expect(constants_str::DIAGNOSTIC_0F72AD46);
        let item = server_admin_contract::admin_role_summary::AdminRoleSummary::new(
            server_admin_contract::admin_role_id::AdminRoleId::try_from(constants_i64::ONE)
                .expect(constants_str::DIAGNOSTIC_392BD170),
            server_admin_contract::admin_bool::AdminBool::from(false),
            server_admin_contract::admin_role_name::AdminRoleName::try_from(String::from(
                constants_str::PG_CRUD_OPERATOR_FIELD,
            ))
            .expect(constants_str::DIAGNOSTIC_D5A91F28),
            server_admin_contract::admin_permission_ids::AdminPermissionIds::try_from(vec![
                first_permission,
                second_permission,
            ])
            .expect(constants_str::DIAGNOSTIC_AB705EC1),
        );
        let page = server_admin_contract::admin_roles_page::AdminRolesPage::new(
            server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(vec![item]).expect(constants_str::DIAGNOSTIC_F91E53B6),
            server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(vec![
                server_admin_contract::admin_permission_summary::AdminPermissionSummary::new(
                    first_permission,
                    server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(String::from(
                        constants_str::VALUE_C6919F81,
                    ))
                    .expect(constants_str::VALUE_286F37C4),
                ),
                server_admin_contract::admin_permission_summary::AdminPermissionSummary::new(
                    second_permission,
                    server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(String::from(
                        constants_str::VALUE_8B8674FD,
                    ))
                    .expect(constants_str::VALUE_CD09FF18),
                ),
            ])
            .expect(constants_str::DIAGNOSTIC_349CA278),
            server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
        );

        let html = crate::admin_role_permissions::admin_role_permissions(
            page.items()
                .first()
                .expect(constants_str::DIAGNOSTIC_719CB4E0),
            &page,
        )
        .to_html();
        assert!(html.contains(constants_str::VALUE_18C354DD));
    }
}
