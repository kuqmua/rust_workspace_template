pub(crate) async fn authn_sign_in(
    auth: crate::AdminAuthReq,
    peer: crate::AdminPeerAddr,
    request_json: crate::AdminSignInJson,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let state = auth.state;
    let headers = auth.headers;
    if !crate::authorization_origin_is_present_and_allowed::authorization_origin_is_present_and_allowed(
        state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(headers.as_ref()),
    )
    .get()
    {
        return Err(crate::AdminError::Authentication);
    }
    let request = request_json.0;
    let (contract_login, contract_password) = request.into_parts();
    let login = crate::AdminLogin::try_from(contract_login.into_inner())
        .map_err(|_error| crate::AdminError::Validation)?;
    let password = crate::admin_password_from_contract(contract_password)
        .map_err(crate::AdminError::password_text)?;
    let peer_subject = crate::StdAdminString::try_from(peer.0.as_ref().ip().to_string())
        .map_err(|_error| crate::AdminError::Validation)?;
    crate::rate_limit::enforce_rate_limit(
        state.as_ref(),
        crate::rate_limit::AdminRateLimitScope::SignInIp,
        &peer_subject,
        state.as_ref().policy.sign_in_ip_limit,
        state.as_ref().policy.sign_in_window,
    )
    .await?;
    let pair_subject =
        crate::StdAdminString::try_from(format!("{}|{}", peer.0.as_ref().ip(), login.as_ref()))
            .map_err(|_error| crate::AdminError::Validation)?;
    crate::rate_limit::enforce_rate_limit(
        state.as_ref(),
        crate::rate_limit::AdminRateLimitScope::SignInIpLogin,
        &pair_subject,
        state.as_ref().policy.sign_in_limit,
        state.as_ref().policy.sign_in_window,
    )
    .await?;
    let recent_failures =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_RECENT_LOGIN_FAILURE_COUNT_SQL)
            .bind(login.as_ref())
            .fetch_one(state.as_ref().pool.as_ref())
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .map(crate::repository::AdminRecentLoginFailureCount::from)
            .map_err(crate::AdminError::from)?;
    if recent_failures
        .reached(state.as_ref().policy.failure_threshold)
        .get()
    {
        return Err(crate::AdminError::RateLimited);
    }
    let optional_user =
        sqlx::query_as::<_, (i64, String, bool)>(constants_str::SERVER_ADMIN_SIGN_IN_USER_SQL)
            .bind(login.as_ref())
            .fetch_optional(state.as_ref().pool.as_ref())
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .and_then(|value| {
                value
                    .map(crate::repository::AdminSignInUser::try_from)
                    .transpose()
            })
            .map_err(crate::AdminError::from)?;
    let Some(sign_in_user) = optional_user else {
        drop(
            state
                .as_ref()
                .password_hasher
                .hash(password)
                .await
                .map_err(crate::AdminError::password_hash)?,
        );
        crate::persistence::record_login_attempt(
            state.as_ref(),
            &login,
            peer,
            crate::StdAdminBool::from(false),
        )
        .await?;
        return Err(crate::AdminError::Authentication);
    };
    let (admin_user_id, password_hash, is_banned) = <(
        crate::AdminUserId,
        crate::AdminPasswordHash,
        crate::StdAdminBool,
    )>::from(sign_in_user);
    let verified = state
        .as_ref()
        .password_hasher
        .verify(password, password_hash)
        .await
        .map_err(|_error| crate::AdminError::Authentication)?;
    if !verified.get() || is_banned.get() {
        crate::persistence::record_login_attempt(
            state.as_ref(),
            &login,
            peer,
            crate::StdAdminBool::from(false),
        )
        .await?;
        return Err(crate::AdminError::Authentication);
    }
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::AdminError::from)?;
    crate::persistence::record_login_attempt(
        state.as_ref(),
        &login,
        peer,
        crate::StdAdminBool::from(true),
    )
    .await?;
    let context_hash =
        crate::authorization_session_context_hash::authorization_session_context_hash(
            crate::HttpAdminHeaderMapRef::from(headers.as_ref()),
            peer,
        )
        .map_err(crate::AdminError::secret_text)?;
    let session = crate::create_session_in_connection::create_session_in_connection(
        state.as_ref(),
        admin_user_id,
        &context_hash,
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
    )
    .await
    .map_err(crate::AdminError::session)?;
    crate::persistence::record_audit_success_in_connection(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        crate::persistence::AdminAuditSuccessRef {
            action: crate::AdminAuditAction::SignIn,
            login: &login,
            resource: crate::AdminAuditResource::Session,
            resource_id: crate::persistence::AdminAuditResourceId::Session(session.session_id()),
            user_id: admin_user_id,
        },
    )
    .await?;
    tx.commit().await.map_err(crate::AdminError::from)?;
    let authenticated = crate::persistence::load_authenticated_admin(
        state.as_ref(),
        admin_user_id,
        session.session_id(),
    )
    .await?;
    let authenticated_contract = crate::authenticated_admin_contract(&authenticated)?;
    let mut response = crate::shared::json_response::json_response(
        server_admin_contract::domain_types::AdminSignInRes::new(authenticated_contract),
    );
    crate::append_session_cookies::append_session_cookies(&mut response, state.as_ref(), &session)?;
    Ok(response)
}
