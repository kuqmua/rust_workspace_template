#[frontend_contract::route_error(AdminVersionPageError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn version(auth: crate::AdminAuthReq) -> axum::response::Response {
    match crate::page_context_impl::page_context_impl(&auth).await {
        Ok((_admin, _branding, password_change_required)) if *password_change_required => {
            axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                server_admin_contract::domain_types::AdminFrontendPath::Profile.get(),
            ))
        }
        Ok((admin, branding, _password_change_required)) => match (
            server_admin_frontend::domain_types::ssr::AdminSsrText::try_from(
                constants_str::VERSION_ALT.to_owned(),
            ),
            server_admin_frontend::domain_types::ssr::AdminSsrText::try_from(
                git_info::project_git_info_value().commit().to_string(),
            ),
        ) {
            (Ok(title), Ok(text)) => crate::html_response_impl::html_response_impl(
                server_admin_frontend::domain_types::ssr::render_text_page_with_access(
                    server_admin_contract::domain_types::AdminPage::Version,
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
