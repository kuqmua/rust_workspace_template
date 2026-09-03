#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) async fn account_me(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    crate::account_me_context_view_ref::account_me_context_view_ref(&admin_auth_request)
        .await
        .map(|context| crate::json_response::json_response(context.0))
}
