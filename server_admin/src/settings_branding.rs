#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn settings_branding(
    auth: crate::AdminAuthReq,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    crate::settings_branding_view::settings_branding_view(auth)
        .await
        .map(crate::shared::json_response::json_response)
}
