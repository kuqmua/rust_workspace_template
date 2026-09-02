#[proc_macro_frontend_contract::route_error(AdminHtmlRolePermissionsError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn role_permissions(
    auth: crate::admin_auth_req::AdminAuthReq,
    form: crate::axum_admin_form::AxumAdminForm<crate::role_permissions_form::RolePermissionsForm>,
) -> axum::response::Response {
    crate::assignment_form_action::assignment_form_action(
        auth,
        crate::assignment_form_target::AssignmentFormTarget::RolePermissions(form.into_inner()),
    )
    .await
}
