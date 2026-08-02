#![allow(clippy::single_call_fn)] // route inventory and HTML composition each register focused authentication operations once

pub(super) async fn sign_in(
    auth: super::AdminAuthReq,
    peer: super::AdminPeerAddr,
    request_json: super::AdminSignInJson,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let state = auth.state;
    let headers = auth.headers;
    if !super::origin_is_present_and_allowed(
        state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
    )
    .get()
    {
        return Err(super::AdminError::Authentication);
    }
    let request = request_json.0;
    let (contract_login, contract_password) = request.into_parts();
    let login = super::super::AdminLogin::try_from(contract_login.into_inner())
        .map_err(|_error| super::AdminError::Validation)?;
    let password = super::admin_password_from_contract(contract_password)
        .map_err(super::AdminError::password_text)?;
    let peer_subject = super::super::StdAdminString::try_from(peer.0.as_ref().ip().to_string())
        .map_err(|_error| super::AdminError::Validation)?;
    super::rate_limit::enforce_rate_limit(
        state.as_ref(),
        super::rate_limit::AdminRateLimitScope::SignInIp,
        &peer_subject,
        state.as_ref().policy.sign_in_ip_limit,
        state.as_ref().policy.sign_in_window,
    )
    .await?;
    let pair_subject = super::super::StdAdminString::try_from(format!(
        "{}|{}",
        peer.0.as_ref().ip(),
        login.as_ref()
    ))
    .map_err(|_error| super::AdminError::Validation)?;
    super::rate_limit::enforce_rate_limit(
        state.as_ref(),
        super::rate_limit::AdminRateLimitScope::SignInIpLogin,
        &pair_subject,
        state.as_ref().policy.sign_in_limit,
        state.as_ref().policy.sign_in_window,
    )
    .await?;
    let recent_failures = super::super::repository::users::recent_login_failure_count(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(state.as_ref().pool.as_ref()),
        &login,
    )
    .await
    .map_err(super::AdminError::from)?;
    if recent_failures
        .reached(state.as_ref().policy.failure_threshold)
        .get()
    {
        return Err(super::AdminError::RateLimited);
    }
    let optional_user = super::super::repository::users::find_sign_in_user(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(state.as_ref().pool.as_ref()),
        &login,
    )
    .await
    .map_err(super::AdminError::from)?;
    let Some(sign_in_user) = optional_user else {
        drop(
            state
                .as_ref()
                .password_hasher
                .hash(password)
                .await
                .map_err(super::AdminError::password_hash)?,
        );
        super::record_login_attempt(
            state.as_ref(),
            &login,
            peer,
            super::super::StdAdminBool::from(false),
        )
        .await?;
        return Err(super::AdminError::Authentication);
    };
    let (admin_user_id, password_hash, is_banned) = sign_in_user.into_parts();
    let verified = state
        .as_ref()
        .password_hasher
        .verify(password, password_hash)
        .await
        .map_err(|_error| super::AdminError::Authentication)?;
    if !verified.get() || is_banned.get() {
        super::record_login_attempt(
            state.as_ref(),
            &login,
            peer,
            super::super::StdAdminBool::from(false),
        )
        .await?;
        return Err(super::AdminError::Authentication);
    }
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::record_login_attempt(
        state.as_ref(),
        &login,
        peer,
        super::super::StdAdminBool::from(true),
    )
    .await?;
    let context_hash = super::session_context_hash(
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        peer,
    )
    .map_err(super::AdminError::secret_text)?;
    let session = super::create_session_in_connection(
        state.as_ref(),
        admin_user_id,
        &context_hash,
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
    )
    .await
    .map_err(super::AdminError::session)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::SignIn,
            login: &login,
            resource: super::super::AdminAuditResource::Session,
            resource_id: super::AdminAuditResourceId::Session(session.session_id()),
            user_id: admin_user_id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    let authenticated =
        super::load_authenticated_admin(state.as_ref(), admin_user_id, session.session_id())
            .await?;
    let authenticated_contract = super::authenticated_admin_contract(&authenticated)?;
    let mut response = super::shared::json_response(server_admin_contract::AdminSignInRes::new(
        authenticated_contract,
    ));
    super::append_session_cookies(&mut response, state.as_ref(), &session)?;
    Ok(response)
}
pub(super) async fn refresh(
    auth: super::AdminAuthReq,
    peer: super::AdminPeerAddr,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let state = auth.state;
    let headers = auth.headers;
    if !super::origin_is_present_and_allowed(
        state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
    )
    .get()
    {
        apply_refresh_failure_delay(state.as_ref().policy.failure_delay).await;
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
        apply_refresh_failure_delay(state.as_ref().policy.failure_delay).await;
        return Err(super::AdminError::Authentication);
    };
    let token = super::super::SecrecyAdminString::try_from(raw_token.as_ref().to_owned())
        .map(super::super::AdminOpaqueToken::new)
        .map_err(super::super::AdminSecretTextError::from)
        .map_err(super::AdminError::authentication_secret_text)?;
    let context_hash = super::session_context_hash(
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        peer,
    )
    .map_err(super::AdminError::authentication_secret_text)?;
    let token_hash = super::hash_refresh_token_with_context(&token, &context_hash)
        .map_err(super::AdminError::authentication_secret_text)?;
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    let optional_user_id = super::super::repository::users::lock_refresh_token_user(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &token_hash,
    )
    .await
    .map_err(super::AdminError::from)?;
    let Some(user_id) = optional_user_id else {
        tx.commit().await.map_err(super::AdminError::from)?;
        apply_refresh_failure_delay(state.as_ref().policy.failure_delay).await;
        return Err(super::AdminError::Authentication);
    };
    let admin_user_id = user_id;
    super::super::repository::users::revoke_refresh_token(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &token_hash,
        admin_user_id,
    )
    .await
    .map_err(super::AdminError::from)?;
    let session = super::create_session_in_connection(
        state.as_ref(),
        admin_user_id,
        &context_hash,
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
    )
    .await
    .map_err(super::AdminError::session)?;
    let login = super::super::repository::sessions::read_active_user_login(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        admin_user_id,
    )
    .await
    .map_err(super::shared::map_repository_error)?
    .ok_or(super::AdminError::Authentication)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Refresh,
            login: &login,
            resource: super::super::AdminAuditResource::Session,
            resource_id: super::AdminAuditResourceId::Session(session.session_id()),
            user_id: admin_user_id,
        },
    )
    .await?;
    let authenticated = super::load_authenticated_admin_from_db(
        &mut super::super::repository::AdminRepositoryDbRef::Connection(
            super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        ),
        admin_user_id,
        session.session_id(),
    )
    .await?;
    let authenticated_contract = super::authenticated_admin_contract(&authenticated)?;
    let mut response = super::shared::json_response(server_admin_contract::AdminSignInRes::new(
        authenticated_contract,
    ));
    super::append_session_cookies(&mut response, state.as_ref(), &session)?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(response)
}
async fn apply_refresh_failure_delay(delay: super::StdAdminFailureDelayMillis) {
    tokio::time::sleep(tokio::time::Duration::from_millis(delay.0)).await;
}
pub(super) async fn sign_out(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let peer = auth.peer;
    let state = auth.state;
    let headers = auth.headers;
    let authenticated = super::authenticate(
        state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        peer,
    )
    .await?;
    super::validate_csrf(
        state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        &authenticated,
    )
    .await?;
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::sessions::revoke_access_session(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        authenticated.session_id,
        authenticated.id,
    )
    .await
    .map_err(super::AdminError::from)?;
    if let Some(raw_refresh) = super::super::find_admin_cookie(
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        super::super::AdminCookieKind::Refresh,
    ) {
        let refresh = super::super::SecrecyAdminString::try_from(raw_refresh.as_ref().to_owned())
            .map(super::super::AdminOpaqueToken::new)
            .map_err(super::super::AdminSecretTextError::from)
            .map_err(super::AdminError::authentication_secret_text)?;
        let context_hash = super::session_context_hash(
            super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
            peer,
        )
        .map_err(super::AdminError::authentication_secret_text)?;
        let refresh_hash = super::hash_refresh_token_with_context(&refresh, &context_hash)
            .map_err(super::AdminError::authentication_secret_text)?;
        super::super::repository::users::revoke_refresh_token(
            super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            &refresh_hash,
            authenticated.id,
        )
        .await
        .map_err(super::AdminError::from)?;
    }
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::SignOut,
            login: &authenticated.login,
            resource: super::super::AdminAuditResource::Session,
            resource_id: super::AdminAuditResourceId::Session(authenticated.session_id),
            user_id: authenticated.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    let mut response = super::AxumAdminResponse(axum::response::IntoResponse::into_response(
        http::StatusCode::NO_CONTENT,
    ));
    super::append_cleared_session_cookies(&mut response, state.as_ref())?;
    Ok(response)
}
