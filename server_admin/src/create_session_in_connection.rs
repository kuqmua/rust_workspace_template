pub(crate) async fn create_session_in_connection(
    state: &crate::admin_auth_svc_state::AdminAuthSvcState,
    user_id: server_admin_core::admin_user_id::AdminUserId,
    context_hash: &crate::admin_token_hash::AdminTokenHash,
    mut connection: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
) -> Result<
    crate::admin_session_bundle::AdminSessionBundle,
    crate::admin_session_error::AdminSessionError,
> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| {
            crate::admin_unix_token_stream::AdminUnixTokenStream::from(duration.as_secs())
        })
        .map_err(|_error| crate::admin_session_error::AdminSessionError::SystemClock)?;
    let session_uuid = uuid::Uuid::new_v4();
    let session_id = crate::admin_session_id::AdminSessionId::from(
        server_admin_core::uuid_admin_value::UuidAdminValue::from(session_uuid),
    );
    let refresh_generated = crate::admin_generated_token::AdminGeneratedToken::generate()
        .map_err(crate::admin_session_error::AdminSessionError::SecretText)?;
    let refresh_hash =
        crate::authorization_hash_refresh_token_with_context::authorization_hash_refresh_token_with_context(
            refresh_generated.token(),
            context_hash,
        )
        .map_err(crate::admin_session_error::AdminSessionError::SecretText)?;
    let refresh_token = crate::admin_refresh_token::AdminRefreshToken::new(
        crate::admin_opaque_token::AdminOpaqueToken::new(refresh_generated.token().clone_secret()),
    );
    let csrf_generated = crate::admin_generated_token::AdminGeneratedToken::generate()
        .map_err(crate::admin_session_error::AdminSessionError::SecretText)?;
    let token_identifier = server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(
        server_admin_core::uuid_admin_value::UuidAdminValue::from(session_uuid)
            .get()
            .to_string(),
    )
    .map(crate::admin_opaque_token::AdminOpaqueToken::new)
    .map_err(crate::admin_secret_text_error::AdminSecretTextError::from)
    .map_err(crate::admin_session_error::AdminSessionError::SecretText)?;
    let token_identifier_hash = crate::hash_opaque_token::hash_opaque_token(&token_identifier)
        .map_err(crate::admin_session_error::AdminSessionError::SecretText)?;
    let expires_at = crate::admin_unix_token_stream::AdminUnixTokenStream::from(
        now.get().saturating_add(state.access_ttl.get()),
    );
    let claims = crate::admin_access_claims::AdminAccessClaims::new(
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
    .map_err(crate::jsonwebtoken_admin_error::JsonwebtokenAdminError::from)
    .map_err(crate::admin_access_token_error::AdminAccessTokenError::from)
    .map_err(crate::admin_session_error::AdminSessionError::AccessToken)
    .and_then(|value| {
        crate::std_admin_access_token::StdAdminAccessToken::try_from(value)
            .map_err(crate::admin_secret_text_error::AdminSecretTextError::from)
            .map_err(crate::admin_session_error::AdminSessionError::SecretText)
    })?;
    let session_offset = i64::try_from(
        state
            .session_limit
            .get()
            .saturating_sub(constants_usize::ONE),
    )
    .unwrap_or(i64::MAX);
    let _access_result = sqlx::query(
        constants_str::integration_fixtures::SERVER_ADMIN_REVOKE_EXCESS_ACCESS_SESSIONS_SQL,
    )
    .bind(user_id.get())
    .bind(session_offset)
    .execute(connection.as_mut())
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map_err(crate::admin_session_error::AdminSessionError::Pg)?;
    let _refresh_result = sqlx::query(
        constants_str::integration_fixtures::SERVER_ADMIN_REVOKE_EXCESS_REFRESH_TOKENS_SQL,
    )
    .bind(user_id.get())
    .bind(session_offset)
    .execute(connection.as_mut())
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map_err(crate::admin_session_error::AdminSessionError::Pg)?;
    sqlx::query(constants_str::integration_fixtures::SERVER_ADMIN_INSERT_ACCESS_SESSION_SQL)
        .bind(session_id.get().get())
        .bind(user_id.get())
        .bind(token_identifier_hash.expose().as_ref())
        .bind(context_hash.expose().as_ref())
        .bind(csrf_generated.hash().expose().as_ref())
        .bind(i64::try_from(state.access_ttl.get()).unwrap_or(i64::MAX))
        .execute(connection.as_mut())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_session_error::AdminSessionError::Pg)
        .map(drop)?;
    sqlx::query(constants_str::integration_fixtures::SERVER_ADMIN_INSERT_REFRESH_TOKEN_SQL)
        .bind(server_admin_core::uuid_admin_value::UuidAdminValue::from(uuid::Uuid::new_v4()).get())
        .bind(user_id.get())
        .bind(refresh_hash.expose().as_ref())
        .bind(i64::try_from(state.refresh_ttl.get()).unwrap_or(i64::MAX))
        .execute(connection.as_mut())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_session_error::AdminSessionError::Pg)
        .map(drop)?;
    Ok(crate::admin_session_bundle::AdminSessionBundle {
        access_token,
        csrf_token: crate::admin_opaque_token::AdminOpaqueToken::new(
            csrf_generated.token().clone_secret(),
        ),
        refresh_token,
        session_id,
    })
}
