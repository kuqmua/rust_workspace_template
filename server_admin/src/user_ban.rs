#[proc_macro_frontend_contract::route_error(AdminHtmlUserBanError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn user_ban(
    auth: crate::admin_auth_req::AdminAuthReq,
    form: crate::axum_admin_form::AxumAdminForm<crate::user_ban_form::UserBanForm>,
) -> axum::response::Response {
    crate::user_mutation_form_action::user_mutation_form_action(
        auth,
        crate::user_mutation_form_target::UserMutationFormTarget::Ban(form.into_inner()),
    )
    .await
}
