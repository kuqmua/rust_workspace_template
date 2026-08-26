#[frontend_contract::domain_types::route_error(AdminHtmlSignOutError)]
pub(super) async fn sign_out(auth: super::super::super::AdminAuthReq) -> axum::response::Response {
    match super::super::form_auth(auth) {
        Ok(auth) => match super::super::super::authn::sign_out(auth).await {
            Ok(response) => {
                let mut target =
                    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                        server_admin_contract::domain_types::AdminFrontendPath::SignIn.get(),
                    ));
                response
                    .0
                    .headers()
                    .get_all(http::header::SET_COOKIE)
                    .iter()
                    .cloned()
                    .for_each(|value| {
                        let _appended =
                            target.headers_mut().append(http::header::SET_COOKIE, value);
                    });
                target
            }
            Err(error) => axum::response::IntoResponse::into_response(error),
        },
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

#[frontend_contract::domain_types::route_error(AdminHtmlChangePasswordError)]
pub(super) async fn change_password(
    auth: super::super::super::AdminAuthReq,
    super::super::super::AxumAdminForm(form): super::super::super::AxumAdminForm<
        super::super::forms::ChangePasswordForm,
    >,
) -> axum::response::Response {
    match super::super::form_auth(auth) {
        Ok(auth) => {
            let request = server_admin_contract::domain_types::AdminChangeOwnPasswordReq::new(
                form.current_password,
                form.new_password,
            );
            match super::super::super::account::change_own_password(
                auth,
                super::super::super::AxumAdminJson(request),
            )
            .await
            {
                Ok(_response) => super::super::success_redirect(
                    server_admin_contract::domain_types::AdminFrontendPath::Profile,
                ),
                Err(error) => axum::response::IntoResponse::into_response(error),
            }
        }
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

#[frontend_contract::domain_types::route_error(AdminHtmlSignInError)]
pub(super) async fn sign_in(
    auth: super::super::super::AdminAuthReq,
    peer: super::super::super::AdminPeerAddr,
    super::super::super::AxumAdminForm(form): super::super::super::AxumAdminForm<
        super::super::forms::SignInForm,
    >,
) -> axum::response::Response {
    let branding = super::super::super::settings::branding_view_ref(&auth)
        .await
        .ok();
    match super::super::super::authn::sign_in(
        auth,
        peer,
        super::super::super::AdminSignInJson(
            server_admin_contract::domain_types::AdminSignInReq::new(form.login, form.password),
        ),
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
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminHtmlAction::SignIn, sign_in),
    (server_admin_contract::domain_types::AdminHtmlAction::SignOut, sign_out),
    (server_admin_contract::domain_types::AdminHtmlAction::ProfilePassword, change_password),
)]
struct AdminHtmlAuthActionRouteRegistry;

pub(super) fn router() -> super::super::super::AxumAdminStateRouter {
    super::super::super::AxumAdminStateRouter::from(AdminHtmlAuthActionRouteRegistry::router())
}
