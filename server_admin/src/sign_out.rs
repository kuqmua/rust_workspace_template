#[proc_macro_frontend_contract::route_error(AdminHtmlSignOutError)]
#[allow(
    clippy::single_call_fn,
    reason = "sign out remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn sign_out(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> axum::response::Response {
    match crate::form_auth_impl::form_auth_impl(admin_auth_request) {
        Ok(auth) => match crate::authn_sign_out::authn_sign_out(auth).await {
            Ok(response) => {
                let mut target =
                    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                        server_admin_contract::admin_frontend_path::AdminFrontendPath::SignIn.get(),
                    ));
                response
                    .get_inner()
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
