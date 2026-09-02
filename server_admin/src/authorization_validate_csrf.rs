pub(crate) async fn authorization_validate_csrf(
    admin_auth_svc_state: &crate::admin_auth_svc_state::AdminAuthSvcState,
    http_admin_header_map_ref: crate::http_admin_header_map_ref::HttpAdminHeaderMapRef<'_>,
    runtime_authenticated_admin: &crate::runtime_authenticated_admin::RuntimeAuthenticatedAdmin,
) -> Result<(), crate::admin_error::AdminError> {
    if !crate::authorization_origin_is_present_and_allowed::authorization_origin_is_present_and_allowed(
        admin_auth_svc_state, http_admin_header_map_ref,
    )
    .get()
    {
        return Err(crate::admin_error::AdminError::Csrf);
    }
    let provided = http_admin_header_map_ref
        .get()
        .get(http::HeaderName::from_static(
            constants_str::X_CSRF_TOKEN_ALT,
        ))
        .and_then(|value| value.to_str().ok())
        .ok_or(crate::admin_error::AdminError::Csrf)?;
    let provided_token =
        server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(provided.to_owned())
            .map(crate::admin_opaque_token::AdminOpaqueToken::new)
            .map_err(crate::admin_secret_text_error::AdminSecretTextError::from)
            .map_err(crate::admin_error::AdminError::csrf_secret_text)?;
    let provided_hash = crate::hash_opaque_token::hash_opaque_token(&provided_token)
        .map_err(crate::admin_error::AdminError::csrf_secret_text)?;
    let expected = sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_CSRF_HASH_SQL)
        .bind(runtime_authenticated_admin.get_session_id().get().get())
        .bind(runtime_authenticated_admin.get_id().get())
        .fetch_optional(admin_auth_svc_state.get_pool().as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .and_then(|value| {
            value
                .map(|hash| {
                    server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(hash)
                        .map(crate::admin_token_hash::AdminTokenHash::new)
                        .map_err(|error| {
                            crate::sqlx_admin_error::SqlxAdminError::from(sqlx::Error::Protocol(
                                error.to_string(),
                            ))
                        })
                })
                .transpose()
        })
        .map_err(crate::admin_error::AdminError::postgresql)?
        .ok_or(crate::admin_error::AdminError::Csrf)?;
    let provided_text = provided_hash.expose();
    let provided_secret =
        match server_runtime_core::secret_text_ref::SecretTextRef::try_from(provided_text.get()) {
            Ok(secret) => secret,
            Err(_error) => return Err(crate::admin_error::AdminError::Csrf),
        };
    let expected_text = expected.expose();
    let expected_secret =
        match server_runtime_core::secret_text_ref::SecretTextRef::try_from(expected_text.get()) {
            Ok(secret) => secret,
            Err(_error) => return Err(crate::admin_error::AdminError::Csrf),
        };
    if server_runtime_core::secret_texts_match::secret_texts_match(expected_secret, provided_secret)
        != server_runtime_core::secret_text_match::SecretTextMatch::Equal
    {
        return Err(crate::admin_error::AdminError::Csrf);
    }
    Ok(())
}
