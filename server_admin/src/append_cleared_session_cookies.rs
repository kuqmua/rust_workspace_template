pub(crate) fn append_cleared_session_cookies(
    response: &mut crate::AxumAdminResponse,
    state: &crate::AdminAuthSvcState,
) -> Result<(), crate::AdminError> {
    [
        crate::AdminCookieKind::Access,
        crate::AdminCookieKind::Refresh,
        crate::AdminCookieKind::Csrf,
    ]
    .into_iter()
    .try_for_each(|kind| {
        let cookie = crate::clear_admin_cookie(kind, state.cookie_secure);
        http::HeaderValue::from_str(cookie.as_ref())
            .map(|header| {
                response
                    .0
                    .headers_mut()
                    .append(http::header::SET_COOKIE, header)
            })
            .map(drop)
            .map_err(|error| {
                crate::AdminError::header(crate::HttpAdminHeaderValueError::from(error))
            })
    })
}
