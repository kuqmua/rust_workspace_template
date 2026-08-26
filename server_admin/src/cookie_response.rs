#[allow(clippy::single_call_fn)] // authentication flows create and rotate the long-lived refresh cookie
pub(super) fn append_session_cookies(
    response: &mut super::AxumAdminResponse,
    state: &super::AdminAuthSvcState,
    session: &super::AdminSessionBundle,
) -> Result<(), super::AdminError> {
    let access = super::super::build_admin_cookie(
        super::super::AdminCookieKind::Access,
        super::super::StdAdminStrRef::from(session.access_token.as_ref().as_str()),
        super::super::AdminCookieMaxAgeSeconds::from(state.access_ttl.0),
        state.cookie_secure,
    );
    let csrf = super::super::build_admin_cookie(
        super::super::AdminCookieKind::Csrf,
        super::super::StdAdminStrRef::from(session.csrf_token.expose().as_ref()),
        super::super::AdminCookieMaxAgeSeconds::from(state.access_ttl.0),
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
                super::AdminError::header(super::HttpAdminHeaderValueError::from(error))
            })
    })?;
    let refresh = super::super::build_admin_cookie(
        super::super::AdminCookieKind::Refresh,
        session.refresh_token.expose(),
        super::super::AdminCookieMaxAgeSeconds::from(state.refresh_ttl.0),
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
        .map_err(|error| super::AdminError::header(super::HttpAdminHeaderValueError::from(error)))
}
pub(super) fn append_cleared_session_cookies(
    response: &mut super::AxumAdminResponse,
    state: &super::AdminAuthSvcState,
) -> Result<(), super::AdminError> {
    [
        super::super::AdminCookieKind::Access,
        super::super::AdminCookieKind::Refresh,
        super::super::AdminCookieKind::Csrf,
    ]
    .into_iter()
    .try_for_each(|kind| {
        let cookie = super::super::clear_admin_cookie(kind, state.cookie_secure);
        http::HeaderValue::from_str(cookie.as_ref())
            .map(|header| {
                response
                    .0
                    .headers_mut()
                    .append(http::header::SET_COOKIE, header)
            })
            .map(drop)
            .map_err(|error| {
                super::AdminError::header(super::HttpAdminHeaderValueError::from(error))
            })
    })
}
