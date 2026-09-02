#[proc_macro_frontend_contract::route_error(AdminHtmlSignInError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn sign_in(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_peer_addr: crate::admin_peer_addr::AdminPeerAddr,
    axum_admin_form: crate::axum_admin_form::AxumAdminForm<crate::sign_in_form::SignInForm>,
) -> axum::response::Response {
    let branding =
        crate::settings_branding_view_ref::settings_branding_view_ref(&admin_auth_request)
            .await
            .ok();
    match crate::authn_sign_in::authn_sign_in(
        admin_auth_request,
        admin_peer_addr,
        crate::admin_sign_in_json::AdminSignInJson::from(
            server_admin_contract::admin_sign_in_request::AdminSignInRequest::new(
                axum_admin_form.get_login().clone(),
                axum_admin_form.get_password().clone(),
            ),
        ),
    )
    .await
    {
        Ok(response) => {
            let source = response.get_inner();
            let mut target =
                axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                    server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
                ));
            source
                .headers()
                .get_all(http::header::SET_COOKIE)
                .iter()
                .cloned()
                .for_each(|value| {
                    let _appended = target.headers_mut().append(http::header::SET_COOKIE, value);
                });
            target
        }
        Err(_error) => {
            let message_result = frontend::admin_ssr_error_message::AdminSsrErrorMessage::try_from(
                String::from(constants_str::SIGN_IN_FAILED),
            );
            match message_result {
                Ok(error_message) => axum::response::IntoResponse::into_response((
                    http::StatusCode::UNAUTHORIZED,
                    axum::response::Html(String::from(frontend::render_sign_in::render_sign_in(
                        Some(error_message),
                        branding.as_ref(),
                    ))),
                )),
                Err(_message_error) => axum::response::IntoResponse::into_response(
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                ),
            }
        }
    }
}
