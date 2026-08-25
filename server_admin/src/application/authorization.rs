pub(super) fn session_context_hash(
    headers: super::super::HttpAdminHeaderMapRef<'_>,
    peer: super::AdminPeerAddr,
) -> Result<super::super::AdminTokenHash, super::super::AdminSecretTextError> {
    let mut context = String::with_capacity(352usize);
    context.push_str(constants_str::CLIENT_ADDRESS);
    let client_address = peer.0.as_ref().ip().to_string();
    context.extend(client_address.chars().take(256usize));
    context.push_str(constants_str::USER_AGENT);
    let user_agent = headers
        .0
        .get(http::header::USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|candidate| {
            !candidate.is_empty() && candidate.len() <= constants_usize::VALUE_8_192
        });
    match user_agent {
        Some(normalized_user_agent) => {
            context.extend(normalized_user_agent.chars().take(256usize));
        }
        None => context.push_str(constants_str::UNKNOWN_USER_AGENT),
    }
    let token = super::super::SecrecyAdminString::try_from(context)
        .map(super::super::AdminOpaqueToken::new)?;
    super::super::hash_opaque_token::hash_opaque_token(&token)
}
pub(super) fn hash_refresh_token_with_context(
    token: &super::super::AdminOpaqueToken,
    context_hash: &super::super::AdminTokenHash,
) -> Result<super::super::AdminTokenHash, super::super::AdminSecretTextError> {
    let token_text = secrecy::ExposeSecret::expose_secret(token.0.as_ref());
    let context_hash_text = secrecy::ExposeSecret::expose_secret(context_hash.0.as_ref());
    let mut token_with_context =
        String::with_capacity(token_text.len().saturating_add(context_hash_text.len()));
    token_with_context.push_str(token_text);
    token_with_context.push_str(context_hash_text);
    let combined_token = super::super::SecrecyAdminString::try_from(token_with_context)
        .map(super::super::AdminOpaqueToken::new)?;
    super::super::hash_opaque_token::hash_opaque_token(&combined_token)
}
pub(super) fn origin_is_present_and_allowed(
    state: &super::AdminAuthSvcState,
    headers: super::super::HttpAdminHeaderMapRef<'_>,
) -> super::super::StdAdminBool {
    super::super::StdAdminBool::from(bool::from(
        server_runtime_http::domain_types::request_origin_allowed(
            server_runtime_http::domain_types::HttpOriginHeadersRef::from(headers.0),
            &state.allowed_origins,
        ),
    ))
}
pub(super) async fn authenticate(
    state: &super::AdminAuthSvcState,
    headers: super::super::HttpAdminHeaderMapRef<'_>,
    peer: super::AdminPeerAddr,
) -> Result<super::AuthenticatedAdmin, super::AdminError> {
    let token = super::super::find_admin_cookie(headers, super::super::AdminCookieKind::Access)
        .ok_or(super::AdminError::Authentication)?;
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[state.issuer.as_ref()]);
    validation.set_audience(&[state.audience.as_ref()]);
    let claims = state
        .decoding_keys
        .as_ref()
        .iter()
        .find_map(|decoding_key| {
            jsonwebtoken::decode::<super::super::AdminAccessClaims>(
                token.as_ref(),
                decoding_key,
                &validation,
            )
            .ok()
            .map(|data| data.claims)
        })
        .ok_or(super::AdminError::Authentication)?;
    let context_hash =
        session_context_hash(headers, peer).map_err(super::AdminError::secret_text)?;
    let active =
        sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_ACTIVE_ACCESS_SESSION_SQL)
            .bind(claims.session_id().get().get())
            .bind(claims.user_id().get())
            .bind(context_hash.expose().as_ref())
            .fetch_one(state.pool.as_ref())
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .map(crate::domain_types::StdAdminBool::from)
            .map_err(super::AdminError::pg)?;
    if !active.get() {
        return Err(super::AdminError::Authentication);
    }
    super::persistence::load_authenticated_admin(state, claims.user_id(), claims.session_id()).await
}
pub(super) async fn validate_csrf(
    state: &super::AdminAuthSvcState,
    headers: super::super::HttpAdminHeaderMapRef<'_>,
    authenticated: &super::AuthenticatedAdmin,
) -> Result<(), super::AdminError> {
    if !origin_is_present_and_allowed(state, headers).get() {
        return Err(super::AdminError::Csrf);
    }
    let provided = headers
        .0
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
        .map_err(super::AdminError::pg)?
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
pub(crate) async fn authorize_generated_request(
    state: &super::AdminAuthSvcState,
    headers: super::super::HttpAdminHeaderMapRef<'_>,
    peer: super::AdminPeerAddr,
    permission: server_admin_contract::domain_types::AdminPermissionStrRef<'_>,
    mutates: super::super::StdAdminBool,
) -> Result<super::AuthenticatedAdmin, super::AdminError> {
    let authenticated = authenticate(state, headers, peer).await?;
    if *authenticated.password_change_required {
        return Err(super::AdminError::Authorization);
    }
    let required_permission = super::super::AdminPermission::try_from(permission.as_ref())
        .map_err(|_error| super::AdminError::Authorization)?;
    if !authenticated
        .permissions
        .as_ref()
        .contains(&required_permission)
    {
        return Err(super::AdminError::Authorization);
    }
    if mutates.get() {
        let subject = super::super::StdAdminString::try_from(authenticated.id.get().to_string())
            .map_err(|_error| super::AdminError::Validation)?;
        super::rate_limit::enforce_rate_limit(
            state,
            super::rate_limit::AdminRateLimitScope::Mutation,
            &subject,
            state.policy.mutation_limit,
            state.policy.mutation_window,
        )
        .await?;
        validate_csrf(state, headers, &authenticated).await?;
    }
    Ok(authenticated)
}
