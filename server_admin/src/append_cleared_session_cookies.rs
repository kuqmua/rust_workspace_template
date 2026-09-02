pub(crate) fn append_cleared_session_cookies(
    axum_admin_response: &mut crate::axum_admin_response::AxumAdminResponse,
    admin_auth_svc_state: &crate::admin_auth_svc_state::AdminAuthSvcState,
) -> Result<(), crate::admin_error::AdminError> {
    [
        crate::admin_cookie_kind::AdminCookieKind::Access,
        crate::admin_cookie_kind::AdminCookieKind::Refresh,
        crate::admin_cookie_kind::AdminCookieKind::Csrf,
    ]
    .into_iter()
    .try_for_each(|kind| {
        let cookie = crate::clear_admin_cookie::clear_admin_cookie(
            kind,
            *admin_auth_svc_state.get_cookie_secure(),
        );
        http::HeaderValue::from_str(cookie.as_ref())
            .map(|header| {
                axum_admin_response
                    .get_inner_mut()
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
