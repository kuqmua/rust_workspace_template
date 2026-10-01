#[proc_macro_frontend_contract_route_error::route_error(AdminHtmlUpdateUserRecordError)]
#[allow(
    clippy::single_call_fn,
    reason = "typed frontend route registration requires a named endpoint function"
)]
pub(crate) async fn update_user_record(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_contract::admin_user_id::AdminUserId,
    >,
    axum_admin_form: crate::axum_admin_form::AxumAdminForm<crate::update_user_form::UpdateUserForm>,
) -> axum::response::Response {
    if axum_admin_path.get_inner() != axum_admin_form.get_user_id() {
        return axum::response::IntoResponse::into_response(
            crate::admin_error::AdminError::Validation,
        );
    }
    crate::update_user::update_user(admin_auth_request, axum_admin_form)
        .await
        .unwrap_or_else(axum::response::IntoResponse::into_response)
}
