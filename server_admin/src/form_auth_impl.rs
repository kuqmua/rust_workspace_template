pub(crate) fn form_auth_impl(
    mut auth: crate::admin_auth_req::AdminAuthReq,
) -> Result<crate::admin_auth_req::AdminAuthReq, crate::admin_error::AdminError> {
    if !crate::authorization_origin_is_present_and_allowed::authorization_origin_is_present_and_allowed(
        auth.state.as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
    )
    .get()
    {
        return Err(crate::admin_error::AdminError::Csrf);
    }
    let token = crate::find_admin_cookie::find_admin_cookie(
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        crate::admin_cookie_kind::AdminCookieKind::Csrf,
    )
    .ok_or(crate::admin_error::AdminError::Csrf)?;
    let value = http::HeaderValue::from_str(token.as_ref()).map_err(|error| {
        crate::admin_error::AdminError::header(
            crate::http_admin_header_value_error::HttpAdminHeaderValueError::from(error),
        )
    })?;
    let _previous = auth.headers.0.insert(
        http::HeaderName::from_static(constants_str::X_CSRF_TOKEN_ALT),
        value,
    );
    Ok(auth)
}
