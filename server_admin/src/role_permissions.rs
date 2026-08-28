#[frontend_contract::domain_types::route_error(AdminHtmlRolePermissionsError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn role_permissions(
    auth: crate::AdminAuthReq,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::RolePermissionsForm>,
) -> axum::response::Response {
    crate::assignment_action(
        auth,
        &form.expected_permission_ids,
        form.selected,
        crate::permission_ids_impl::permission_ids_impl,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
        server_admin_contract::domain_types::AdminSetRolePermissionsReq::new,
        crate::AxumAdminPath(crate::role_path_impl::role_path_impl(form.role_id)),
        crate::mutations_set_permissions::mutations_set_permissions,
    )
    .await
}
