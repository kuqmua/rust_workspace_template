#[proc_macro_frontend_contract::route_error(AdminHtmlDeleteUserError)]
#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) async fn delete_user(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_form: crate::axum_admin_form::AxumAdminForm<crate::user_id_form::UserIdForm>,
) -> axum::response::Response {
    crate::delete_confirmed_entity::delete_confirmed_entity(
        admin_auth_request,
        crate::confirmed_delete_target::ConfirmedDeleteTarget::User(axum_admin_form.into_inner()),
    )
    .await
}
