pub(crate) fn append_session_cookies(
    axum_admin_response: &mut crate::axum_admin_response::AxumAdminResponse,
    admin_auth_svc_state: &crate::admin_auth_svc_state::AdminAuthSvcState,
    admin_session_bundle: &crate::admin_session_bundle::AdminSessionBundle,
) -> Result<(), crate::admin_error::AdminError> {
    let access = crate::build_admin_cookie::build_admin_cookie(
        crate::admin_cookie_kind::AdminCookieKind::Access,
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            admin_session_bundle.access_token().as_ref().as_str(),
        ),
        crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(
            admin_auth_svc_state.get_access_ttl().get(),
        ),
        *admin_auth_svc_state.get_cookie_secure(),
    );
    let csrf = crate::build_admin_cookie::build_admin_cookie(
        crate::admin_cookie_kind::AdminCookieKind::Csrf,
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            admin_session_bundle.csrf_token().expose().as_ref(),
        ),
        crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(
            admin_auth_svc_state.get_access_ttl().get(),
        ),
        *admin_auth_svc_state.get_cookie_secure(),
    );
    [access, csrf].into_iter().try_for_each(|cookie| {
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
    })?;
    let refresh = crate::build_admin_cookie::build_admin_cookie(
        crate::admin_cookie_kind::AdminCookieKind::Refresh,
        admin_session_bundle.refresh_token().expose(),
        crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(
            admin_auth_svc_state.get_refresh_ttl().get(),
        ),
        *admin_auth_svc_state.get_cookie_secure(),
    );
    http::HeaderValue::from_str(refresh.as_ref())
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
}
