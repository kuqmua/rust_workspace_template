pub(crate) async fn create_session_in_connection(
    state: &crate::AdminAuthSvcState,
    user_id: crate::AdminUserId,
    context_hash: &crate::AdminTokenHash,
    mut connection: crate::repository::SqlxAdminRepositoryConnectionMutRef<'_>,
) -> Result<crate::AdminSessionBundle, crate::AdminSessionError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| crate::AdminUnixTokenStream::from(duration.as_secs()))
        .map_err(|_error| crate::AdminSessionError::SystemClock)?;
    let session_uuid = uuid::Uuid::new_v4();
    let session_id = crate::AdminSessionId::from(crate::UuidAdminValue::from(session_uuid));
    let refresh_generated =
        crate::AdminGeneratedToken::generate().map_err(crate::AdminSessionError::SecretText)?;
    let refresh_hash =
        crate::authorization_hash_refresh_token_with_context::authorization_hash_refresh_token_with_context(
            refresh_generated.token(),
            context_hash,
        )
        .map_err(crate::AdminSessionError::SecretText)?;
    let refresh_token = crate::AdminRefreshToken::new(crate::AdminOpaqueToken::new(
        refresh_generated.token().clone_secret(),
    ));
    let csrf_generated =
        crate::AdminGeneratedToken::generate().map_err(crate::AdminSessionError::SecretText)?;
    let token_identifier = crate::SecrecyAdminString::try_from(
        crate::UuidAdminValue::from(session_uuid).get().to_string(),
    )
    .map(crate::AdminOpaqueToken::new)
    .map_err(crate::AdminSecretTextError::from)
    .map_err(crate::AdminSessionError::SecretText)?;
    let token_identifier_hash = crate::hash_opaque_token::hash_opaque_token(&token_identifier)
        .map_err(crate::AdminSessionError::SecretText)?;
    let expires_at =
        crate::AdminUnixTokenStream::from(now.get().saturating_add(state.access_ttl.get()));
    let claims = crate::AdminAccessClaims::new(
        user_id,
        session_id,
        now,
        expires_at,
        state.issuer.clone(),
        state.audience.clone(),
    );
    let access_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &state.encoding_key.0,
    )
    .map_err(crate::JsonwebtokenAdminError::from)
    .map_err(crate::AdminAccessTokenError::from)
    .map_err(crate::AdminSessionError::AccessToken)
    .and_then(|value| {
        crate::StdAdminAccessToken::try_from(value)
            .map_err(crate::AdminSecretTextError::from)
            .map_err(crate::AdminSessionError::SecretText)
    })?;
    let session_offset = i64::try_from(
        state
            .session_limit
            .get()
            .saturating_sub(constants_usize::ONE),
    )
    .unwrap_or(i64::MAX);
    let _access_result = sqlx::query(constants_str::SERVER_ADMIN_REVOKE_EXCESS_ACCESS_SESSIONS_SQL)
        .bind(user_id.get())
        .bind(session_offset)
        .execute(connection.as_mut())
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(crate::AdminSessionError::Pg)?;
    let _refresh_result = sqlx::query(constants_str::SERVER_ADMIN_REVOKE_EXCESS_REFRESH_TOKENS_SQL)
        .bind(user_id.get())
        .bind(session_offset)
        .execute(connection.as_mut())
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(crate::AdminSessionError::Pg)?;
    sqlx::query(constants_str::SERVER_ADMIN_INSERT_ACCESS_SESSION_SQL)
        .bind(session_id.get().get())
        .bind(user_id.get())
        .bind(token_identifier_hash.expose().as_ref())
        .bind(context_hash.expose().as_ref())
        .bind(csrf_generated.hash().expose().as_ref())
        .bind(i64::try_from(state.access_ttl.get()).unwrap_or(i64::MAX))
        .execute(connection.as_mut())
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(crate::AdminSessionError::Pg)
        .map(drop)?;
    sqlx::query(constants_str::SERVER_ADMIN_INSERT_REFRESH_TOKEN_SQL)
        .bind(crate::UuidAdminValue::from(uuid::Uuid::new_v4()).get())
        .bind(user_id.get())
        .bind(refresh_hash.expose().as_ref())
        .bind(i64::try_from(state.refresh_ttl.get()).unwrap_or(i64::MAX))
        .execute(connection.as_mut())
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(crate::AdminSessionError::Pg)
        .map(drop)?;
    Ok(crate::AdminSessionBundle {
        access_token,
        csrf_token: crate::AdminOpaqueToken::new(csrf_generated.token().clone_secret()),
        refresh_token,
        session_id,
    })
}
