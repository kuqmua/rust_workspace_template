#![allow(clippy::single_call_fn)] // route inventory registers this branding operation once

pub(super) async fn branding(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    super::settings_branding_view::branding_view(auth)
        .await
        .map(super::shared::json_response::json_response)
}
