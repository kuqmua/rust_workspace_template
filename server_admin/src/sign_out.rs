#[frontend_contract::route_error(AdminHtmlSignOutError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn sign_out(auth: crate::AdminAuthReq) -> axum::response::Response {
    match crate::form_auth_impl::form_auth_impl(auth) {
        Ok(auth) => match crate::authn_sign_out::authn_sign_out(auth).await {
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
