#[frontend_contract_macros::route_error(AdminSignInPageError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn sign_in_page(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> axum::response::Response {
    match crate::settings_branding_view::settings_branding_view(auth).await {
        Ok(branding) => crate::html_response_impl::html_response_impl(
            server_admin_frontend::render_sign_in::render_sign_in(None, Some(&branding)),
        ),
        Err(error) => crate::html_page_error_impl::html_page_error_impl(error),
    }
}
