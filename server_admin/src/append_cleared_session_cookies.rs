pub(crate) fn append_cleared_session_cookies(
    response: &mut crate::axum_admin_response::AxumAdminResponse,
    state: &crate::admin_auth_svc_state::AdminAuthSvcState,
) -> Result<(), crate::admin_error::AdminError> {
    [
        crate::admin_cookie_kind::AdminCookieKind::Access,
        crate::admin_cookie_kind::AdminCookieKind::Refresh,
        crate::admin_cookie_kind::AdminCookieKind::Csrf,
    ]
    .into_iter()
    .try_for_each(|kind| {
        let cookie = crate::clear_admin_cookie::clear_admin_cookie(kind, state.cookie_secure);
        http::HeaderValue::from_str(cookie.as_ref())
            .map(|header| {
                response
                    .0
                    .headers_mut()
                    .append(http::header::SET_COOKIE, header)
            })
            .map(drop)
            .map_err(|error| {
                crate::admin_error::AdminError::header(
                    crate::http_admin_header_value_error::HttpAdminHeaderValueError::from(error),
                )
            })
    })
}
