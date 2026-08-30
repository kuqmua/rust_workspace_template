#[frontend_contract_macros::route_error(AdminHtmlUserPasswordError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn user_password(
    auth: crate::admin_auth_req::AdminAuthReq,
    crate::axum_admin_form::AxumAdminForm(form): crate::axum_admin_form::AxumAdminForm<
        crate::user_password_form::UserPasswordForm,
    >,
) -> axum::response::Response {
    crate::user_mutation_form_action::user_mutation_form_action(
        auth,
        crate::user_mutation_form_target::UserMutationFormTarget::Password(form),
    )
    .await
}
