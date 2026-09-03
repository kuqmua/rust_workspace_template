#[proc_macro_frontend_contract_route_error::route_error(AdminHtmlUserPasswordError)]
#[allow(
    clippy::single_call_fn,
    reason = "user password remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn user_password(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_form: crate::axum_admin_form::AxumAdminForm<
        crate::user_password_form::UserPasswordForm,
    >,
) -> axum::response::Response {
    crate::user_mutation_form_action::user_mutation_form_action(
        admin_auth_request,
        crate::user_mutation_form_target::UserMutationFormTarget::Password(
            axum_admin_form.into_inner(),
        ),
    )
    .await
}
