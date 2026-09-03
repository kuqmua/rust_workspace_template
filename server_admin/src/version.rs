#[proc_macro_frontend_contract::route_error(AdminVersionPageError)]
#[allow(
    clippy::single_call_fn,
    reason = "version remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn version(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> axum::response::Response {
    match crate::page_context_impl::page_context_impl(&admin_auth_request).await {
        Ok((_admin, _branding, password_change_required)) if *password_change_required => {
            axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile.get(),
            ))
        }
        Ok((admin, branding, _password_change_required)) => match (
            frontend::admin_ssr_text::AdminSsrText::try_from(constants_str::VERSION_ALT.to_owned()),
            frontend::admin_ssr_text::AdminSsrText::try_from(
                git_info::project_git_info_value::project_git_info_value()
                    .commit()
                    .to_string(),
            ),
        ) {
            (Ok(title), Ok(text)) => crate::html_response_impl::html_response_impl(
                frontend::render_text_page_with_access::render_text_page_with_access(
                    server_admin_contract::admin_page::AdminPage::Version,
                    title,
                    text,
                    &admin,
                    &branding,
                ),
            ),
            (Err(_error), _) | (_, Err(_error)) => {
                axum::response::IntoResponse::into_response(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(error) => crate::html_page_error_impl::html_page_error_impl(error),
    }
}
