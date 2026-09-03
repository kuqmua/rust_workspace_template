#[proc_macro_frontend_contract_route_error::route_error(AdminSignInPageError)]
#[allow(
    clippy::single_call_fn,
    reason = "sign in page remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn sign_in_page(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> axum::response::Response {
    match crate::settings_branding_view::settings_branding_view(admin_auth_request).await {
        Ok(branding) => crate::html_response_impl::html_response_impl(
            frontend::render_sign_in::render_sign_in(None, Some(&branding)),
        ),
        Err(error) => crate::html_page_error_impl::html_page_error_impl(error),
    }
}
