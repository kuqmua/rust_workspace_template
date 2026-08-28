pub(crate) async fn settings_branding(
    auth: crate::AdminAuthReq,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    crate::settings_branding_view::settings_branding_view(auth)
        .await
        .map(crate::shared::json_response::json_response)
}
