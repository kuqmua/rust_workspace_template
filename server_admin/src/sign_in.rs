#[frontend_contract_macros::route_error(AdminHtmlSignInError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn sign_in(
    auth: crate::admin_auth_req::AdminAuthReq,
    peer: crate::admin_peer_addr::AdminPeerAddr,
    crate::axum_admin_form::AxumAdminForm(form): crate::axum_admin_form::AxumAdminForm<
        crate::sign_in_form::SignInForm,
    >,
) -> axum::response::Response {
    let branding = crate::settings_branding_view_ref::settings_branding_view_ref(&auth)
        .await
        .ok();
    match crate::authn_sign_in::authn_sign_in(
        auth,
        peer,
        crate::admin_sign_in_json::AdminSignInJson(
            server_admin_contract::admin_sign_in_req::AdminSignInReq::new(
                form.login,
                form.password,
            ),
        ),
    )
    .await
    {
        Ok(response) => {
            let source = response.0;
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
            let message_result =
                server_admin_frontend::admin_ssr_error_message::AdminSsrErrorMessage::try_from(
                    String::from(constants_str::test_fixtures::SIGN_IN_FAILED),
                );
            match message_result {
                Ok(error_message) => axum::response::IntoResponse::into_response((
                    http::StatusCode::UNAUTHORIZED,
                    axum::response::Html(String::from(
                        server_admin_frontend::render_sign_in::render_sign_in(
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
