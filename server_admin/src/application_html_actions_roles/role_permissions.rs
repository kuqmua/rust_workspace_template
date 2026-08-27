#[frontend_contract::domain_types::route_error(AdminHtmlRolePermissionsError)]
pub(super) async fn role_permissions(
    auth: super::super::super::super::AdminAuthReq,
    super::super::super::super::AxumAdminForm(form): super::super::super::super::AxumAdminForm<
        super::super::super::forms::RolePermissionsForm,
    >,
) -> axum::response::Response {
    super::super::assignment_action(
        auth,
        &form.expected_permission_ids,
        form.selected,
        super::super::super::permission_ids_impl::permission_ids_impl,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
        server_admin_contract::domain_types::AdminSetRolePermissionsReq::new,
        super::super::super::super::AxumAdminPath(
            super::super::super::role_path_impl::role_path_impl(form.role_id),
        ),
        super::super::super::super::roles::mutations_set_permissions::mutations_set_permissions,
    )
    .await
}
