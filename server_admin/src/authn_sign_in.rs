pub(crate) async fn authn_sign_in(
    auth: crate::admin_auth_req::AdminAuthReq,
    peer: crate::admin_peer_addr::AdminPeerAddr,
    request_json: crate::admin_sign_in_json::AdminSignInJson,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let state = auth.state;
    let headers = auth.headers;
    if !crate::authorization_origin_is_present_and_allowed::authorization_origin_is_present_and_allowed(
        state.as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(headers.as_ref()),
    )
    .get()
    {
        return Err(crate::admin_error::AdminError::Authentication);
    }
    let request = request_json.0;
    let (contract_login, contract_password) = request.into_parts();
    let login =
        server_admin_contract::admin_login::AdminLogin::try_from(contract_login.into_inner())
            .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    let password =
        crate::admin_password_from_contract::admin_password_from_contract(contract_password)
            .map_err(crate::admin_error::AdminError::password_text)?;
    let peer_subject = server_admin_core::std_admin_string::StdAdminString::try_from(
        peer.0.as_ref().ip().to_string(),
    )
    .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    crate::enforce_rate_limit::enforce_rate_limit(
        state.as_ref(),
        crate::admin_rate_limit_scope::AdminRateLimitScope::SignInIp,
        &peer_subject,
        state.as_ref().policy.sign_in_ip_limit,
        state.as_ref().policy.sign_in_window,
    )
    .await?;
    let pair_subject = server_admin_core::std_admin_string::StdAdminString::try_from(format!(
        "{}|{}",
        peer.0.as_ref().ip(),
        login.as_ref()
    ))
    .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    crate::enforce_rate_limit::enforce_rate_limit(
        state.as_ref(),
        crate::admin_rate_limit_scope::AdminRateLimitScope::SignInIpLogin,
        &pair_subject,
        state.as_ref().policy.sign_in_limit,
        state.as_ref().policy.sign_in_window,
    )
    .await?;
    let recent_failures = sqlx::query_scalar::<_, i64>(
        constants_str::integration_fixtures::SERVER_ADMIN_RECENT_LOGIN_FAILURE_COUNT_SQL,
    )
    .bind(login.as_ref())
    .fetch_one(state.as_ref().pool.as_ref())
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map(crate::admin_recent_login_failure_count::AdminRecentLoginFailureCount::from)
    .map_err(crate::admin_error::AdminError::from)?;
    if recent_failures
        .reached(state.as_ref().policy.failure_threshold)
        .get()
    {
        return Err(crate::admin_error::AdminError::RateLimited);
    }
    let optional_user = sqlx::query_as::<_, (i64, String, bool)>(
        constants_str::integration_fixtures::SERVER_ADMIN_SIGN_IN_USER_SQL,
    )
    .bind(login.as_ref())
    .fetch_optional(state.as_ref().pool.as_ref())
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .and_then(|value| {
        value
            .map(crate::admin_sign_in_user::AdminSignInUser::try_from)
            .transpose()
    })
    .map_err(crate::admin_error::AdminError::from)?;
    let Some(sign_in_user) = optional_user else {
        drop(
            state
                .as_ref()
                .password_hasher
                .hash(password)
                .await
                .map_err(crate::admin_error::AdminError::password_hash)?,
        );
        crate::record_login_attempt::record_login_attempt(
            state.as_ref(),
            &login,
            peer,
            server_admin_core::std_admin_bool::StdAdminBool::from(false),
        )
        .await?;
        return Err(crate::admin_error::AdminError::Authentication);
    };
    let (admin_user_id, password_hash, is_banned) = <(
        server_admin_core::admin_user_record_id::AdminUserRecordId,
        crate::admin_password_hash::AdminPasswordHash,
        server_admin_core::std_admin_bool::StdAdminBool,
    )>::from(sign_in_user);
    let verified = state
        .as_ref()
        .password_hasher
        .verify(password, password_hash)
        .await
        .map_err(|_error| crate::admin_error::AdminError::Authentication)?;
    if !verified.get() || is_banned.get() {
        crate::record_login_attempt::record_login_attempt(
            state.as_ref(),
            &login,
            peer,
            server_admin_core::std_admin_bool::StdAdminBool::from(false),
        )
        .await?;
        return Err(crate::admin_error::AdminError::Authentication);
    }
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    crate::record_login_attempt::record_login_attempt(
        state.as_ref(),
        &login,
        peer,
        server_admin_core::std_admin_bool::StdAdminBool::from(true),
    )
    .await?;
    let context_hash =
        crate::authorization_session_context_hash::authorization_session_context_hash(
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(headers.as_ref()),
            peer,
        )
        .map_err(crate::admin_error::AdminError::secret_text)?;
    let session = crate::create_session_in_connection::create_session_in_connection(
        state.as_ref(),
        admin_user_id,
        &context_hash,
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
    )
    .await
    .map_err(crate::admin_error::AdminError::session)?;
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef {
            action: crate::admin_audit_action::AdminAuditAction::SignIn,
            login: &login,
            resource: crate::admin_audit_resource::AdminAuditResource::Session,
            resource_id: crate::admin_audit_resource_id::AdminAuditResourceId::Session(
                session.session_id(),
            ),
            user_id: admin_user_id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    let authenticated = crate::load_authenticated_admin::load_authenticated_admin(
        state.as_ref(),
        admin_user_id,
        session.session_id(),
    )
    .await?;
    let authenticated_contract =
        crate::authenticated_admin_contract::authenticated_admin_contract(&authenticated)?;
    let mut response = crate::json_response::json_response(
        server_admin_contract::admin_sign_in_res::AdminSignInRes::new(authenticated_contract),
    );
    crate::append_session_cookies::append_session_cookies(&mut response, state.as_ref(), &session)?;
    Ok(response)
}
