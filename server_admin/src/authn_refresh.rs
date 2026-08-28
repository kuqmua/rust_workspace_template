#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn authn_refresh(
    auth: crate::AdminAuthReq,
    peer: crate::AdminPeerAddr,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let state = auth.state;
    let headers = auth.headers;
    if !crate::authorization_origin_is_present_and_allowed::authorization_origin_is_present_and_allowed(
        state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(headers.as_ref()),
    )
    .get()
    {
        crate::authn_apply_refresh_failure_delay::authn_apply_refresh_failure_delay(
            state.as_ref().policy.failure_delay,
        )
        .await;
        return Err(crate::AdminError::Authentication);
    }
    let peer_subject = crate::StdAdminString::try_from(peer.0.as_ref().ip().to_string())
        .map_err(|_error| crate::AdminError::Validation)?;
    crate::rate_limit::enforce_rate_limit(
        state.as_ref(),
        crate::rate_limit::AdminRateLimitScope::RefreshIp,
        &peer_subject,
        state.as_ref().policy.refresh_limit,
        state.as_ref().policy.refresh_window,
    )
    .await?;
    let Some(raw_token) = crate::find_admin_cookie(
        crate::HttpAdminHeaderMapRef::from(headers.as_ref()),
        crate::AdminCookieKind::Refresh,
    ) else {
        crate::authn_apply_refresh_failure_delay::authn_apply_refresh_failure_delay(
            state.as_ref().policy.failure_delay,
        )
        .await;
        return Err(crate::AdminError::Authentication);
    };
    let token = crate::SecrecyAdminString::try_from(raw_token.as_ref().to_owned())
        .map(crate::AdminOpaqueToken::new)
        .map_err(crate::AdminSecretTextError::from)
        .map_err(crate::AdminError::authentication_secret_text)?;
    let context_hash =
        crate::authorization_session_context_hash::authorization_session_context_hash(
            crate::HttpAdminHeaderMapRef::from(headers.as_ref()),
            peer,
        )
        .map_err(crate::AdminError::authentication_secret_text)?;
    let token_hash =
        crate::authorization_hash_refresh_token_with_context::authorization_hash_refresh_token_with_context(
            &token,
            &context_hash,
        )
        .map_err(crate::AdminError::authentication_secret_text)?;
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::AdminError::from)?;
    let optional_user_id =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_LOCK_REFRESH_TOKEN_USER_SQL)
            .bind(token_hash.expose().as_ref())
            .fetch_optional(&mut *tx)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .and_then(|value| {
                value
                    .map(crate::AdminUserId::try_from)
                    .transpose()
                    .map_err(crate::domain_types::SqlxAdminError::from)
            })
            .map_err(crate::AdminError::from)?;
    let Some(user_id) = optional_user_id else {
        tx.commit().await.map_err(crate::AdminError::from)?;
        crate::authn_apply_refresh_failure_delay::authn_apply_refresh_failure_delay(
            state.as_ref().policy.failure_delay,
        )
        .await;
        return Err(crate::AdminError::Authentication);
    };
    let admin_user_id = user_id;
    crate::repository::revoke_refresh_token::revoke_refresh_token(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &token_hash,
        admin_user_id,
    )
    .await
    .map_err(crate::AdminError::from)?;
    let session = crate::create_session_in_connection::create_session_in_connection(
        state.as_ref(),
        admin_user_id,
        &context_hash,
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
    )
    .await
    .map_err(crate::AdminError::session)?;
    let login =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_ACTIVE_USER_LOGIN_SQL)
            .bind(admin_user_id.get())
            .fetch_optional(&mut *tx)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .map_err(crate::AdminError::from)?
            .map(server_admin_contract::domain_types::AdminLogin::try_from)
            .transpose()
            .map_err(|_error| crate::AdminError::Validation)?
            .ok_or(crate::AdminError::Authentication)?;
    crate::persistence::record_audit_success_in_connection(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        crate::persistence::AdminAuditSuccessRef {
            action: crate::AdminAuditAction::Refresh,
            login: &login,
            resource: crate::AdminAuditResource::Session,
            resource_id: crate::persistence::AdminAuditResourceId::Session(session.session_id()),
            user_id: admin_user_id,
        },
    )
    .await?;
    let authenticated = crate::persistence::load_authenticated_admin_from_db(
        &mut crate::persistence::AdminDbRef::Connection(
            crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        ),
        admin_user_id,
        session.session_id(),
    )
    .await?;
    let authenticated_contract = crate::authenticated_admin_contract(&authenticated)?;
    let mut response = crate::shared::json_response::json_response(
        server_admin_contract::domain_types::AdminSignInRes::new(authenticated_contract),
    );
    crate::append_session_cookies::append_session_cookies(&mut response, state.as_ref(), &session)?;
    tx.commit().await.map_err(crate::AdminError::from)?;
    Ok(response)
}
