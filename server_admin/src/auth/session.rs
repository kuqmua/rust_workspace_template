#![allow(clippy::single_call_fn)] // public facade preserves session API while this module owns persistence and rotation
#[allow(clippy::single_call_fn)] // clock failure mapping remains isolated from session persistence
fn unix_now() -> Result<super::super::AdminUnixTokenStream, super::AdminSessionError> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| super::super::AdminUnixTokenStream::from(duration.as_secs()))
        .map_err(|_error| super::AdminSessionError::SystemClock)
}
#[allow(clippy::single_call_fn)] // token identifier conversion keeps secret construction explicit
fn opaque_token_from_uuid(value: super::super::UuidAdminValue) -> super::super::AdminOpaqueToken {
    super::super::AdminOpaqueToken::new(super::super::SecrecyAdminString::from(
        secrecy::SecretBox::new(Box::new(value.0.to_string())),
    ))
}
pub(super) async fn create_session_in_connection(
    state: &super::AdminAuthSvcState,
    user_id: super::super::AdminUserId,
    context_hash: &super::super::AdminTokenHash,
    mut connection: super::SqlxAdminPgConnectionRef<'_>,
) -> Result<super::AdminSessionBundle, super::AdminSessionError> {
    let now = unix_now()?;
    let session_uuid = uuid::Uuid::new_v4();
    let session_id =
        super::super::AdminSessionId::from(super::super::UuidAdminValue::from(session_uuid));
    let refresh_id = uuid::Uuid::new_v4();
    let refresh_generated = super::super::AdminGeneratedToken::generate();
    let refresh_hash =
        super::hash_refresh_token_with_context(refresh_generated.token(), context_hash);
    let refresh_token = super::super::AdminRefreshToken::new(super::super::AdminOpaqueToken::new(
        super::super::SecrecyAdminString::from(secrecy::SecretBox::new(Box::new(
            secrecy::ExposeSecret::expose_secret(refresh_generated.token().0.as_ref()).to_owned(),
        ))),
    ));
    let csrf_generated = super::super::AdminGeneratedToken::generate();
    let token_identifier_hash = super::super::hash_opaque_token(&opaque_token_from_uuid(
        super::super::UuidAdminValue::from(session_uuid),
    ));
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
    .map_err(|error| {
        super::AdminSessionError::AccessToken(super::super::AdminAccessTokenError(
            super::super::JsonwebtokenAdminError::from(error),
        ))
    })?;
    let session_offset =
        i64::try_from(state.session_limit.0.saturating_sub(1usize)).unwrap_or(i64::MAX);
    let _expired_access = sqlx::query("UPDATE admin_access_sessions SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL AND id IN (SELECT id FROM admin_access_sessions WHERE user_id = $1 AND revoked_at IS NULL ORDER BY created_at DESC OFFSET $2)")
        .bind(user_id.0)
        .bind(session_offset)
        .execute(connection.as_mut())
        .await
        .map_err(|error| super::AdminSessionError::Pg(super::super::SqlxAdminError::from(error)))?;
    let _expired_refresh = sqlx::query("UPDATE admin_refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL AND id IN (SELECT id FROM admin_refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL ORDER BY created_at DESC OFFSET $2)")
        .bind(user_id.0)
        .bind(session_offset)
        .execute(connection.as_mut())
        .await
        .map_err(|error| super::AdminSessionError::Pg(super::super::SqlxAdminError::from(error)))?;
    let _access_result = sqlx::query(
        "INSERT INTO admin_access_sessions (id, user_id, token_identifier_hash, token_context_hash, csrf_token_hash, expires_at) VALUES ($1, $2, $3, $4, $5, NOW() + ($6 * INTERVAL '1 second'))",
    )
    .bind(session_uuid)
    .bind(user_id.0)
    .bind(secrecy::ExposeSecret::expose_secret(token_identifier_hash.0.as_ref()))
    .bind(secrecy::ExposeSecret::expose_secret(context_hash.0.as_ref()))
    .bind(secrecy::ExposeSecret::expose_secret(csrf_generated.hash().0.as_ref()))
    .bind(i64::try_from(state.access_ttl.0).unwrap_or(i64::MAX))
    .execute(connection.as_mut())
    .await
    .map_err(|error| super::AdminSessionError::Pg(super::super::SqlxAdminError::from(error)))?;
    let _refresh_result = sqlx::query(
        "INSERT INTO admin_refresh_tokens (id, user_id, token_hash, expires_at) VALUES ($1, $2, $3, NOW() + ($4 * INTERVAL '1 second'))",
    )
    .bind(refresh_id)
    .bind(user_id.0)
    .bind(secrecy::ExposeSecret::expose_secret(refresh_hash.0.as_ref()))
    .bind(i64::try_from(state.refresh_ttl.0).unwrap_or(i64::MAX))
    .execute(connection.as_mut())
    .await
    .map_err(|error| super::AdminSessionError::Pg(super::super::SqlxAdminError::from(error)))?;
    Ok(super::AdminSessionBundle {
        access_token,
        csrf_token: super::super::AdminOpaqueToken::new(super::super::SecrecyAdminString::from(
            secrecy::SecretBox::new(Box::new(
                secrecy::ExposeSecret::expose_secret(csrf_generated.token().0.as_ref()).to_owned(),
            )),
        )),
        refresh_token,
        session_id,
    })
}
