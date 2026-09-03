#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) async fn settings_branding(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    crate::settings_branding_view::settings_branding_view(admin_auth_request)
        .await
        .map(crate::json_response::json_response)
}
