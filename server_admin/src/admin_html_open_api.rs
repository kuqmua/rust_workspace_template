#[frontend_contract::domain_types::route_error(AdminOpenApiPageError)]
pub(crate) async fn admin_html_open_api(auth: crate::AdminAuthReq) -> axum::response::Response {
    let branding_result =
        crate::settings_branding_view_ref::settings_branding_view_ref(&auth).await;
    let authorized =
        crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
            auth.state.as_ref(),
            crate::domain_types::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
            auth.peer,
            crate::domain_types::AdminPermission::OpenApiRead.as_str(),
            crate::domain_types::StdAdminBool::from(false),
        )
        .await;
    match (authorized, branding_result) {
        (Ok(admin), Ok(branding)) => {
            let admin = match crate::authenticated_admin_contract(&admin) {
                Ok(value) => value,
                Err(error) => {
                    return crate::html_page_error_impl::html_page_error_impl(error);
                }
            };
            let document = utoipa::openapi::OpenApi::from(
                crate::domain_types::generated_tables::generated_open_api(),
            );
            match serde_json::to_string_pretty(&document) {
                Ok(text) => match (
                    server_admin_frontend::domain_types::ssr::AdminSsrText::try_from(
                        constants_str::OPENAPI_DOCUMENT.to_owned(),
                    ),
                    server_admin_frontend::domain_types::ssr::AdminSsrText::try_from(text),
                ) {
                    (Ok(title), Ok(text)) => crate::html_response_impl::html_response_impl(
                        server_admin_frontend::domain_types::ssr::render_text_page_with_access(
                            server_admin_contract::domain_types::AdminPage::OpenApi,
                            title,
                            text,
                            &admin,
                            &branding,
                        ),
                    ),
                    (Err(_error), _) | (_, Err(_error)) => {
                        axum::response::IntoResponse::into_response(
                            http::StatusCode::INTERNAL_SERVER_ERROR,
                        )
                    }
                },
                Err(_error) => axum::response::IntoResponse::into_response(
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                ),
            }
        }
        (Err(error), _) | (_, Err(error)) => {
            crate::html_page_error_impl::html_page_error_impl(error)
        }
    }
}
