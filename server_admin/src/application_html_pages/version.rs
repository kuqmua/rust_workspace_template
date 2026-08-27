#[frontend_contract::domain_types::route_error(AdminVersionPageError)]
pub(in crate::domain_types::auth::html) async fn version(
    auth: super::super::super::AdminAuthReq,
) -> axum::response::Response {
    match super::super::page_context_impl::page_context_impl(&auth).await {
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
                git_info::domain_types::project_git_info()
                    .commit()
                    .to_string(),
            ),
        ) {
            (Ok(title), Ok(text)) => super::super::html_response_impl::html_response_impl(
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
        Err(error) => super::super::html_page_error_impl::html_page_error_impl(error),
    }
}
