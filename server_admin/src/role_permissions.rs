#[frontend_contract_macros::route_error(AdminHtmlRolePermissionsError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn role_permissions(
    auth: crate::admin_auth_req::AdminAuthReq,
    crate::axum_admin_form::AxumAdminForm(form): crate::axum_admin_form::AxumAdminForm<
        crate::role_permissions_form::RolePermissionsForm,
    >,
) -> axum::response::Response {
    crate::assignment_action::assignment_action(
        auth,
        &form.expected_permission_ids,
        form.selected,
        crate::permission_ids_impl::permission_ids_impl,
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles,
        server_admin_contract::admin_set_role_permissions_req::AdminSetRolePermissionsReq::new,
        crate::axum_admin_path::AxumAdminPath(crate::role_path_impl::role_path_impl(form.role_id)),
        crate::mutations_set_permissions::mutations_set_permissions,
    )
    .await
}
