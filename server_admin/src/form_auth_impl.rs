pub(crate) fn form_auth_impl(
    mut auth: crate::AdminAuthReq,
) -> Result<crate::AdminAuthReq, crate::AdminError> {
    if !crate::authorization_origin_is_present_and_allowed::authorization_origin_is_present_and_allowed(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
    )
    .get()
    {
        return Err(crate::AdminError::Csrf);
    }
    let token = crate::find_admin_cookie(
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        crate::AdminCookieKind::Csrf,
    )
    .ok_or(crate::AdminError::Csrf)?;
    let value = http::HeaderValue::from_str(token.as_ref()).map_err(|error| {
        crate::AdminError::header(crate::HttpAdminHeaderValueError::from(error))
    })?;
    let _previous = auth.headers.0.insert(
        http::HeaderName::from_static(constants_str::X_CSRF_TOKEN_ALT),
        value,
    );
    Ok(auth)
}
