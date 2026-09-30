#[allow(
    clippy::single_call_fn,
    reason = "the HTML sign-in error response is directly unit tested for status preservation"
)]
pub(crate) fn sign_in_error_response(
    admin_error: crate::admin_error::AdminError,
    admin_branding_view: Option<&server_admin_contract::admin_branding_view::AdminBrandingView>,
) -> axum::response::Response {
    let status = axum::response::IntoResponse::into_response(admin_error).status();
    let message_result = frontend_admin::admin_ssr_error_message::AdminSsrErrorMessage::try_from(
        String::from(constants_str::SIGN_IN_FAILED),
    );
    match message_result {
        Ok(error_message) => axum::response::IntoResponse::into_response((
            status,
            axum::response::Html(String::from(
                frontend_admin::render_sign_in::render_sign_in(
                    Some(error_message),
                    admin_branding_view,
                ),
            )),
        )),
        Err(_message_error) => {
            axum::response::IntoResponse::into_response(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sign_in_error_response_preserves_rate_limit_status() {
        let response =
            super::sign_in_error_response(crate::admin_error::AdminError::RateLimited, None);
        assert_eq!(response.status(), http::StatusCode::TOO_MANY_REQUESTS);
    }

    #[test]
    fn test_sign_in_error_response_preserves_authentication_status() {
        let response =
            super::sign_in_error_response(crate::admin_error::AdminError::Authentication, None);
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
    }
}
