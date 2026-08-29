#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn settings_branding(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    crate::settings_branding_view::settings_branding_view(auth)
        .await
        .map(crate::json_response::json_response)
}
