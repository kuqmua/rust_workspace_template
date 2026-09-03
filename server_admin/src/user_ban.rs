#[proc_macro_frontend_contract::route_error(AdminHtmlUserBanError)]
#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) async fn user_ban(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_form: crate::axum_admin_form::AxumAdminForm<crate::user_ban_form::UserBanForm>,
) -> axum::response::Response {
    crate::user_mutation_form_action::user_mutation_form_action(
        admin_auth_request,
        crate::user_mutation_form_target::UserMutationFormTarget::Ban(axum_admin_form.into_inner()),
    )
    .await
}
