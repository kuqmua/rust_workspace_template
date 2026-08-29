pub(crate) fn append_session_cookies(
    response: &mut crate::axum_admin_response::AxumAdminResponse,
    state: &crate::admin_auth_svc_state::AdminAuthSvcState,
    session: &crate::admin_session_bundle::AdminSessionBundle,
) -> Result<(), crate::admin_error::AdminError> {
    let access = crate::build_admin_cookie::build_admin_cookie(
        crate::admin_cookie_kind::AdminCookieKind::Access,
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            session.access_token.as_ref().as_str(),
        ),
        crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(state.access_ttl.get()),
        state.cookie_secure,
    );
    let csrf = crate::build_admin_cookie::build_admin_cookie(
        crate::admin_cookie_kind::AdminCookieKind::Csrf,
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            session.csrf_token.expose().as_ref(),
        ),
        crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(state.access_ttl.get()),
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
                crate::admin_error::AdminError::header(
                    crate::http_admin_header_value_error::HttpAdminHeaderValueError::from(error),
                )
            })
    })?;
    let refresh = crate::build_admin_cookie::build_admin_cookie(
        crate::admin_cookie_kind::AdminCookieKind::Refresh,
        session.refresh_token.expose(),
        crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(
            state.refresh_ttl.get(),
        ),
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
        .map_err(|error| {
            crate::admin_error::AdminError::header(
                crate::http_admin_header_value_error::HttpAdminHeaderValueError::from(error),
            )
        })
}
