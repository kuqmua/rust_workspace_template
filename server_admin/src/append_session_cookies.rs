pub(crate) fn append_session_cookies(
    response: &mut crate::AxumAdminResponse,
    state: &crate::AdminAuthSvcState,
    session: &crate::AdminSessionBundle,
) -> Result<(), crate::AdminError> {
    let access = crate::build_admin_cookie(
        crate::AdminCookieKind::Access,
        crate::StdAdminStrRef::from(session.access_token.as_ref().as_str()),
        crate::AdminCookieMaxAgeSeconds::from(state.access_ttl.get()),
        state.cookie_secure,
    );
    let csrf = crate::build_admin_cookie(
        crate::AdminCookieKind::Csrf,
        crate::StdAdminStrRef::from(session.csrf_token.expose().as_ref()),
        crate::AdminCookieMaxAgeSeconds::from(state.access_ttl.get()),
        state.cookie_secure,
    );
    [access, csrf].into_iter().try_for_each(|cookie| {
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
    })?;
    let refresh = crate::build_admin_cookie(
        crate::AdminCookieKind::Refresh,
        session.refresh_token.expose(),
        crate::AdminCookieMaxAgeSeconds::from(state.refresh_ttl.get()),
        state.cookie_secure,
    );
    http::HeaderValue::from_str(refresh.as_ref())
        .map(|header| {
            response
                .0
                .headers_mut()
                .append(http::header::SET_COOKIE, header)
        })
        .map(drop)
        .map_err(|error| crate::AdminError::header(crate::HttpAdminHeaderValueError::from(error)))
}
