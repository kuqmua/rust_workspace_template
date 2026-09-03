#[proc_macro_frontend_contract::route_error(AdminHtmlRolePermissionsError)]
#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) async fn role_permissions(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_form: crate::axum_admin_form::AxumAdminForm<
        crate::role_permissions_form::RolePermissionsForm,
    >,
) -> axum::response::Response {
    crate::assignment_form_action::assignment_form_action(
        admin_auth_request,
        crate::assignment_form_target::AssignmentFormTarget::RolePermissions(
            axum_admin_form.into_inner(),
        ),
    )
    .await
}
