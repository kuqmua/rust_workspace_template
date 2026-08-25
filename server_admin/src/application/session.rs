#![allow(clippy::single_call_fn)] // public facade preserves session API while this module owns persistence and rotation
pub(super) async fn create_session_in_connection(
    state: &super::AdminAuthSvcState,
    user_id: super::super::AdminUserId,
    context_hash: &super::super::AdminTokenHash,
    mut connection: super::SqlxAdminPgConnectionRef<'_>,
) -> Result<super::AdminSessionBundle, super::AdminSessionError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| super::super::AdminUnixTokenStream::from(duration.as_secs()))
        .map_err(|_error| super::AdminSessionError::SystemClock)?;
    let session_uuid = uuid::Uuid::new_v4();
    let session_id =
        super::super::AdminSessionId::from(super::super::UuidAdminValue::from(session_uuid));
    let refresh_generated = super::super::AdminGeneratedToken::generate()
        .map_err(super::AdminSessionError::SecretText)?;
    let refresh_hash =
        super::hash_refresh_token_with_context(refresh_generated.token(), context_hash)
            .map_err(super::AdminSessionError::SecretText)?;
    let refresh_token = super::super::AdminRefreshToken::new(super::super::AdminOpaqueToken::new(
        super::super::SecrecyAdminString::from(secrecy::SecretBox::new(Box::new(
            secrecy::ExposeSecret::expose_secret(refresh_generated.token().0.as_ref()).clone(),
        ))),
    ));
    let csrf_generated = super::super::AdminGeneratedToken::generate()
        .map_err(super::AdminSessionError::SecretText)?;
    let token_identifier = super::super::SecrecyAdminString::try_from(
        super::super::UuidAdminValue::from(session_uuid)
            .get()
            .to_string(),
    )
    .map(super::super::AdminOpaqueToken::new)
    .map_err(super::super::AdminSecretTextError::from)
    .map_err(super::AdminSessionError::SecretText)?;
    let token_identifier_hash = super::super::hash_opaque_token(&token_identifier)
        .map_err(super::AdminSessionError::SecretText)?;
    let expires_at =
        super::super::AdminUnixTokenStream::from(now.0.saturating_add(state.access_ttl.0));
    let claims = super::super::AdminAccessClaims::new(
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
    .map(super::super::StdAdminAccessToken)
    .map_err(super::super::JsonwebtokenAdminError::from)
    .map_err(super::super::AdminAccessTokenError::from)
    .map_err(super::AdminSessionError::AccessToken)?;
    let session_offset =
        i64::try_from(usize::from(state.session_limit).saturating_sub(constants_usize::ONE))
            .unwrap_or(i64::MAX);
    let _access_result = sqlx::query(constants_str::SERVER_ADMIN_REVOKE_EXCESS_ACCESS_SESSIONS_SQL)
        .bind(user_id.get())
        .bind(session_offset)
        .execute(connection.as_mut())
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(super::AdminSessionError::Pg)?;
    let _refresh_result = sqlx::query(constants_str::SERVER_ADMIN_REVOKE_EXCESS_REFRESH_TOKENS_SQL)
        .bind(user_id.get())
        .bind(session_offset)
        .execute(connection.as_mut())
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(super::AdminSessionError::Pg)?;
    sqlx::query(constants_str::SERVER_ADMIN_INSERT_ACCESS_SESSION_SQL)
        .bind(session_id.get().get())
        .bind(user_id.get())
        .bind(token_identifier_hash.expose().as_ref())
        .bind(context_hash.expose().as_ref())
        .bind(csrf_generated.hash().expose().as_ref())
        .bind(i64::try_from(u64::from(state.access_ttl)).unwrap_or(i64::MAX))
        .execute(connection.as_mut())
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(super::AdminSessionError::Pg)
        .map(drop)?;
    sqlx::query(constants_str::SERVER_ADMIN_INSERT_REFRESH_TOKEN_SQL)
        .bind(super::super::UuidAdminValue::from(uuid::Uuid::new_v4()).get())
        .bind(user_id.get())
        .bind(refresh_hash.expose().as_ref())
        .bind(i64::try_from(u64::from(state.refresh_ttl)).unwrap_or(i64::MAX))
        .execute(connection.as_mut())
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(super::AdminSessionError::Pg)
        .map(drop)?;
    Ok(super::AdminSessionBundle {
        access_token,
        csrf_token: super::super::AdminOpaqueToken::new(super::super::SecrecyAdminString::from(
            secrecy::SecretBox::new(Box::new(
                secrecy::ExposeSecret::expose_secret(csrf_generated.token().0.as_ref()).clone(),
            )),
        )),
        refresh_token,
        session_id,
    })
}
