pub(super) async fn validate_csrf(
    state: &super::AdminAuthSvcState,
    headers: super::super::HttpAdminHeaderMapRef<'_>,
    authenticated: &super::AuthenticatedAdmin,
) -> Result<(), super::AdminError> {
    if !super::authorization_origin_is_present_and_allowed::origin_is_present_and_allowed(
        state, headers,
    )
    .get()
    {
        return Err(super::AdminError::Csrf);
    }
    let provided = headers
        .get()
        .get(http::HeaderName::from_static(
            constants_str::X_CSRF_TOKEN_ALT,
        ))
        .and_then(|value| value.to_str().ok())
        .ok_or(super::AdminError::Csrf)?;
    let provided_token = super::super::SecrecyAdminString::try_from(provided.to_owned())
        .map(super::super::AdminOpaqueToken::new)
        .map_err(super::super::AdminSecretTextError::from)
        .map_err(super::AdminError::csrf_secret_text)?;
    let provided_hash = super::super::hash_opaque_token::hash_opaque_token(&provided_token)
        .map_err(super::AdminError::csrf_secret_text)?;
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
        .map_err(super::AdminError::postgresql)?
        .ok_or(super::AdminError::Csrf)?;
    let provided_text = provided_hash.expose();
    let provided_secret =
        match server_runtime_http::domain_types::SecretTextRef::try_from(provided_text.get()) {
            Ok(secret) => secret,
            Err(_error) => return Err(super::AdminError::Csrf),
        };
    let expected_text = expected.expose();
    let expected_secret =
        match server_runtime_http::domain_types::SecretTextRef::try_from(expected_text.get()) {
            Ok(secret) => secret,
            Err(_error) => return Err(super::AdminError::Csrf),
        };
    if server_runtime_http::domain_types::secret_texts_match(expected_secret, provided_secret)
        != server_runtime_http::domain_types::SecretTextMatch::Equal
    {
        return Err(super::AdminError::Csrf);
    }
    Ok(())
}
