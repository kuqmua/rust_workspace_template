#[frontend_contract::domain_types::route_error(AdminSignInPageError)]
pub(in crate::domain_types::auth::html) async fn sign_in_page(
    auth: super::super::super::AdminAuthReq,
) -> axum::response::Response {
    match super::super::super::settings_branding_view::settings_branding_view(auth).await {
        Ok(branding) => super::super::html_response_impl::html_response_impl(
            server_admin_frontend::domain_types::ssr::render_sign_in(None, Some(&branding)),
        ),
        Err(error) => super::super::html_page_error_impl::html_page_error_impl(error),
    }
}
