#![allow(clippy::single_call_fn)] // route inventory registers this authentication operation once

pub(super) async fn authn_refresh(
    auth: super::AdminAuthReq,
    peer: super::AdminPeerAddr,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let state = auth.state;
    let headers = auth.headers;
    if !super::authorization_origin_is_present_and_allowed::authorization_origin_is_present_and_allowed(
        state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
    )
    .get()
    {
        super::authn_apply_refresh_failure_delay::authn_apply_refresh_failure_delay(
            state.as_ref().policy.failure_delay,
        )
        .await;
        return Err(super::AdminError::Authentication);
    }
    let peer_subject = super::super::StdAdminString::try_from(peer.0.as_ref().ip().to_string())
        .map_err(|_error| super::AdminError::Validation)?;
    super::rate_limit::enforce_rate_limit(
        state.as_ref(),
        super::rate_limit::AdminRateLimitScope::RefreshIp,
        &peer_subject,
        state.as_ref().policy.refresh_limit,
        state.as_ref().policy.refresh_window,
    )
    .await?;
    let Some(raw_token) = super::super::find_admin_cookie(
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        super::super::AdminCookieKind::Refresh,
    ) else {
        super::authn_apply_refresh_failure_delay::authn_apply_refresh_failure_delay(
            state.as_ref().policy.failure_delay,
        )
        .await;
        return Err(super::AdminError::Authentication);
    };
    let token = super::super::SecrecyAdminString::try_from(raw_token.as_ref().to_owned())
        .map(super::super::AdminOpaqueToken::new)
        .map_err(super::super::AdminSecretTextError::from)
        .map_err(super::AdminError::authentication_secret_text)?;
    let context_hash =
        super::authorization_session_context_hash::authorization_session_context_hash(
            super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
            peer,
        )
        .map_err(super::AdminError::authentication_secret_text)?;
    let token_hash =
        super::authorization_hash_refresh_token_with_context::authorization_hash_refresh_token_with_context(
            &token,
            &context_hash,
        )
        .map_err(super::AdminError::authentication_secret_text)?;
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    let optional_user_id =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_LOCK_REFRESH_TOKEN_USER_SQL)
            .bind(token_hash.expose().as_ref())
            .fetch_optional(&mut *tx)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .and_then(|value| {
                value
                    .map(super::super::AdminUserId::try_from)
                    .transpose()
                    .map_err(crate::domain_types::SqlxAdminError::from)
            })
            .map_err(super::AdminError::from)?;
    let Some(user_id) = optional_user_id else {
        tx.commit().await.map_err(super::AdminError::from)?;
        super::authn_apply_refresh_failure_delay::authn_apply_refresh_failure_delay(
            state.as_ref().policy.failure_delay,
        )
        .await;
        return Err(super::AdminError::Authentication);
    };
    let admin_user_id = user_id;
    crate::adapters::repository::revoke_refresh_token::revoke_refresh_token(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &token_hash,
        admin_user_id,
    )
    .await
    .map_err(super::AdminError::from)?;
    let session = super::create_session_in_connection::create_session_in_connection(
        state.as_ref(),
        admin_user_id,
        &context_hash,
        super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
    )
    .await
    .map_err(super::AdminError::session)?;
    let login =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_ACTIVE_USER_LOGIN_SQL)
            .bind(admin_user_id.get())
            .fetch_optional(&mut *tx)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .map_err(super::AdminError::from)?
            .map(server_admin_contract::domain_types::AdminLogin::try_from)
            .transpose()
            .map_err(|_error| super::AdminError::Validation)?
            .ok_or(super::AdminError::Authentication)?;
    super::persistence::record_audit_success_in_connection(
        super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::persistence::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Refresh,
            login: &login,
            resource: super::super::AdminAuditResource::Session,
            resource_id: super::persistence::AdminAuditResourceId::Session(session.session_id()),
            user_id: admin_user_id,
        },
    )
    .await?;
    let authenticated = super::persistence::load_authenticated_admin_from_db(
        &mut super::persistence::AdminDbRef::Connection(
            crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        ),
        admin_user_id,
        session.session_id(),
    )
    .await?;
    let authenticated_contract = super::authenticated_admin_contract(&authenticated)?;
    let mut response = super::shared::json_response::json_response(
        server_admin_contract::domain_types::AdminSignInRes::new(authenticated_contract),
    );
    super::append_session_cookies::append_session_cookies(&mut response, state.as_ref(), &session)?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(response)
}
