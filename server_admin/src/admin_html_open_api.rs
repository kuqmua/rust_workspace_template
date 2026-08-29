#[frontend_contract_macros::route_error(AdminOpenApiPageError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn admin_html_open_api(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> axum::response::Response {
    let branding_result =
        crate::settings_branding_view_ref::settings_branding_view_ref(&auth).await;
    let authorized =
        crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
            auth.state.as_ref(),
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
            auth.peer,
            server_admin_contract::admin_permission::AdminPermission::OpenApiRead.as_str(),
            server_admin_core::std_admin_bool::StdAdminBool::from(false),
        )
        .await;
    match (authorized, branding_result) {
        (Ok(admin), Ok(branding)) => {
            let admin =
                match crate::authenticated_admin_contract::authenticated_admin_contract(&admin) {
                    Ok(value) => value,
                    Err(error) => {
                        return crate::html_page_error_impl::html_page_error_impl(error);
                    }
                };
            let document =
                utoipa::openapi::OpenApi::from(crate::generated_open_api::generated_open_api());
            match serde_json::to_string_pretty(&document) {
                Ok(text) => match (
                    server_admin_frontend::admin_ssr_text::AdminSsrText::try_from(
                        constants_str::test_fixtures::OPENAPI_DOCUMENT.to_owned(),
                    ),
                    server_admin_frontend::admin_ssr_text::AdminSsrText::try_from(text),
                ) {
                    (Ok(title), Ok(text)) => crate::html_response_impl::html_response_impl(
                        server_admin_frontend::render_text_page_with_access::render_text_page_with_access(
                            server_admin_contract::admin_page::AdminPage::OpenApi,
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
