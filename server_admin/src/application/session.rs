#![allow(clippy::single_call_fn)] // public facade preserves session API while this module owns persistence and rotation
#[allow(clippy::single_call_fn)] // clock failure mapping remains isolated from session persistence
fn unix_now() -> Result<super::super::AdminUnixTokenStream, super::AdminSessionError> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| super::super::AdminUnixTokenStream::from(duration.as_secs()))
        .map_err(|_error| super::AdminSessionError::SystemClock)
}
#[allow(clippy::single_call_fn)] // token identifier conversion keeps secret construction explicit
fn opaque_token_from_uuid(
    value: super::super::UuidAdminValue,
) -> Result<super::super::AdminOpaqueToken, super::super::AdminSecretTextError> {
    Ok(
        super::super::SecrecyAdminString::try_from(value.get().to_string())
            .map(super::super::AdminOpaqueToken::new)?,
    )
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
    let token_identifier = opaque_token_from_uuid(super::super::UuidAdminValue::from(session_uuid))
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
    .map_err(|error| {
        super::AdminSessionError::AccessToken(super::super::AdminAccessTokenError(
            super::super::JsonwebtokenAdminError::from(error),
        ))
    })?;
    crate::adapters::repository::sessions::enforce_session_limit(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(connection.as_mut()),
        user_id,
        state.session_limit,
        super::super::StdAdminBool::from(true),
    )
    .await
    .map_err(super::AdminSessionError::Pg)?;
    crate::adapters::repository::sessions::insert_access_session(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(connection.as_mut()),
        session_id,
        user_id,
        &token_identifier_hash,
        context_hash,
        csrf_generated.hash(),
        state.access_ttl,
    )
    .await
    .map_err(super::AdminSessionError::Pg)?;
    crate::adapters::repository::sessions::insert_refresh_token(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(connection.as_mut()),
        super::super::UuidAdminValue::from(uuid::Uuid::new_v4()),
        user_id,
        &refresh_hash,
        state.refresh_ttl,
    )
    .await
    .map_err(super::AdminSessionError::Pg)?;
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
