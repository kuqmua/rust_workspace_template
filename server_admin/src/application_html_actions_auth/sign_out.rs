#[frontend_contract::domain_types::route_error(AdminHtmlSignOutError)]
pub(super) async fn sign_out(
    auth: super::super::super::super::AdminAuthReq,
) -> axum::response::Response {
    match super::super::super::form_auth_impl::form_auth_impl(auth) {
        Ok(auth) => match super::super::super::super::authn_sign_out::authn_sign_out(auth).await {
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
