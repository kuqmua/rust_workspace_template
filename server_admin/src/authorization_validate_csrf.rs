pub(crate) async fn authorization_validate_csrf(
    state: &crate::AdminAuthSvcState,
    headers: crate::HttpAdminHeaderMapRef<'_>,
    authenticated: &crate::AuthenticatedAdmin,
) -> Result<(), crate::AdminError> {
    if !crate::authorization_origin_is_present_and_allowed::authorization_origin_is_present_and_allowed(
        state, headers,
    )
    .get()
    {
        return Err(crate::AdminError::Csrf);
    }
    let provided = headers
        .get()
        .get(http::HeaderName::from_static(
            constants_str::X_CSRF_TOKEN_ALT,
        ))
        .and_then(|value| value.to_str().ok())
        .ok_or(crate::AdminError::Csrf)?;
    let provided_token = crate::SecrecyAdminString::try_from(provided.to_owned())
        .map(crate::AdminOpaqueToken::new)
        .map_err(crate::AdminSecretTextError::from)
        .map_err(crate::AdminError::csrf_secret_text)?;
    let provided_hash = crate::hash_opaque_token::hash_opaque_token(&provided_token)
        .map_err(crate::AdminError::csrf_secret_text)?;
    let expected = sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_CSRF_HASH_SQL)
        .bind(authenticated.session_id.get().get())
        .bind(authenticated.id.get())
        .fetch_optional(state.pool.as_ref())
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .and_then(|value| {
            value
                .map(|hash| {
                    crate::domain_types::SecrecyAdminString::try_from(hash)
                        .map(crate::domain_types::AdminTokenHash::new)
                        .map_err(|error| {
                            crate::domain_types::SqlxAdminError::from(sqlx::Error::Protocol(
                                error.to_string(),
                            ))
                        })
                })
                .transpose()
        })
        .map_err(crate::AdminError::postgresql)?
        .ok_or(crate::AdminError::Csrf)?;
    let provided_text = provided_hash.expose();
    let provided_secret =
        match server_runtime_http::domain_types::SecretTextRef::try_from(provided_text.get()) {
            Ok(secret) => secret,
            Err(_error) => return Err(crate::AdminError::Csrf),
        };
    let expected_text = expected.expose();
    let expected_secret =
        match server_runtime_http::domain_types::SecretTextRef::try_from(expected_text.get()) {
            Ok(secret) => secret,
            Err(_error) => return Err(crate::AdminError::Csrf),
        };
    if server_runtime_http::domain_types::secret_texts_match(expected_secret, provided_secret)
        != server_runtime_http::domain_types::SecretTextMatch::Equal
    {
        return Err(crate::AdminError::Csrf);
    }
    Ok(())
}
