#[proc_macro_frontend_contract_route_error::route_error(AdminHtmlRoleRulesError)]
#[allow(
    clippy::single_call_fn,
    reason = "role rules remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn role_rules(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_form: crate::axum_admin_form::AxumAdminForm<crate::role_rules_form::RoleRulesForm>,
) -> axum::response::Response {
    crate::assignment_form_action::assignment_form_action(
        admin_auth_request,
        crate::assignment_form_target::AssignmentFormTarget::RoleRules(
            axum_admin_form.into_inner(),
        ),
    )
    .await
}
