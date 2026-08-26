pub(super) fn form_auth_impl(
    mut auth: super::super::AdminAuthReq,
) -> Result<super::super::AdminAuthReq, super::super::AdminError> {
    if !super::super::authorization_origin_is_present_and_allowed::authorization_origin_is_present_and_allowed(
        auth.state.as_ref(),
        super::super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
    )
    .get()
    {
        return Err(super::super::AdminError::Csrf);
    }
    let token = super::super::super::find_admin_cookie(
        super::super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        super::super::super::AdminCookieKind::Csrf,
    )
    .ok_or(super::super::AdminError::Csrf)?;
    let value = http::HeaderValue::from_str(token.as_ref()).map_err(|error| {
        super::super::AdminError::header(super::super::HttpAdminHeaderValueError::from(error))
    })?;
    let _previous = auth.headers.0.insert(
        http::HeaderName::from_static(constants_str::X_CSRF_TOKEN_ALT),
        value,
    );
    Ok(auth)
}
