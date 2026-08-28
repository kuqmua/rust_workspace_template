#[frontend_contract::domain_types::route_error(AdminHtmlSignInError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn sign_in(
    auth: crate::AdminAuthReq,
    peer: crate::AdminPeerAddr,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::SignInForm>,
) -> axum::response::Response {
    let branding = crate::settings_branding_view_ref::settings_branding_view_ref(&auth)
        .await
        .ok();
    match crate::authn_sign_in::authn_sign_in(
        auth,
        peer,
        crate::AdminSignInJson(server_admin_contract::domain_types::AdminSignInReq::new(
            form.login,
            form.password,
        )),
    )
    .await
    {
        Ok(response) => {
            let source = response.0;
            let mut target =
                axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                    server_admin_contract::domain_types::AdminFrontendPath::Users.get(),
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
            let message_result =
                server_admin_frontend::domain_types::ssr::AdminSsrErrorMessage::try_from(
                    String::from(constants_str::SIGN_IN_FAILED),
                );
            match message_result {
                Ok(error_message) => axum::response::IntoResponse::into_response((
                    http::StatusCode::UNAUTHORIZED,
                    axum::response::Html(String::from(
                        server_admin_frontend::domain_types::ssr::render_sign_in(
                            Some(error_message),
                            branding.as_ref(),
                        ),
                    )),
                )),
                Err(_message_error) => axum::response::IntoResponse::into_response(
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                ),
            }
        }
    }
}
