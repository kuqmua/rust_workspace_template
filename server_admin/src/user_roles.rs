#[proc_macro_frontend_contract::route_error(AdminHtmlUserRolesError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn user_roles(
    auth: crate::admin_auth_req::AdminAuthReq,
    form: crate::axum_admin_form::AxumAdminForm<crate::user_roles_form::UserRolesForm>,
) -> axum::response::Response {
    crate::assignment_form_action::assignment_form_action(
        auth,
        crate::assignment_form_target::AssignmentFormTarget::UserRoles(form.into_inner()),
    )
    .await
}
