#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) async fn authn_refresh(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_peer_addr: crate::admin_peer_addr::AdminPeerAddr,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let state = admin_auth_request.get_state();
    let headers = admin_auth_request.get_headers();
    if !crate::authorization_origin_is_present_and_allowed::authorization_origin_is_present_and_allowed(
        state.as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(headers.as_ref()),
    )
    .get()
    {
        crate::authn_apply_refresh_failure_delay::authn_apply_refresh_failure_delay(
            state.as_ref().get_policy().get_failure_delay(),
        )
        .await;
        return Err(crate::admin_error::AdminError::Authentication);
    }
    let peer_subject = server_admin_core::std_admin_string::StdAdminString::try_from(
        admin_peer_addr.get_inner().as_ref().ip().to_string(),
    )
    .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    crate::enforce_rate_limit::enforce_rate_limit(
        state.as_ref(),
        crate::admin_rate_limit_scope::AdminRateLimitScope::RefreshIp,
        &peer_subject,
        state.as_ref().get_policy().get_refresh_limit(),
        state.as_ref().get_policy().get_refresh_window(),
    )
    .await?;
    let Some(raw_token) = crate::find_admin_cookie::find_admin_cookie(
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(headers.as_ref()),
        crate::admin_cookie_kind::AdminCookieKind::Refresh,
    ) else {
        crate::authn_apply_refresh_failure_delay::authn_apply_refresh_failure_delay(
            state.as_ref().get_policy().get_failure_delay(),
        )
        .await;
        return Err(crate::admin_error::AdminError::Authentication);
    };
    let token = server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(
        raw_token.as_ref().to_owned(),
    )
    .map(crate::admin_opaque_token::AdminOpaqueToken::new)
    .map_err(crate::admin_secret_text_error::AdminSecretTextError::from)
    .map_err(crate::admin_error::AdminError::authentication_secret_text)?;
    let context_hash =
        crate::authorization_session_context_hash::authorization_session_context_hash(
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(headers.as_ref()),
            admin_peer_addr,
        )
        .map_err(crate::admin_error::AdminError::authentication_secret_text)?;
    let token_hash =
        crate::authorization_hash_refresh_token_with_context::authorization_hash_refresh_token_with_context(
            &token,
            &context_hash,
        )
        .map_err(crate::admin_error::AdminError::authentication_secret_text)?;
    let mut tx = state
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    let optional_user_id =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_LOCK_REFRESH_TOKEN_USER_SQL)
            .bind(token_hash.expose().as_ref())
            .fetch_optional(&mut *tx)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
            .and_then(|value| {
                value
                    .map(server_admin_core::admin_user_record_id::AdminUserRecordId::try_from)
                    .transpose()
                    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
            })
            .map_err(crate::admin_error::AdminError::from)?;
    let Some(user_id) = optional_user_id else {
        tx.commit()
            .await
            .map_err(crate::admin_error::AdminError::from)?;
        crate::authn_apply_refresh_failure_delay::authn_apply_refresh_failure_delay(
            state.as_ref().get_policy().get_failure_delay(),
        )
        .await;
        return Err(crate::admin_error::AdminError::Authentication);
    };
    crate::revoke_refresh_token::revoke_refresh_token(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        &token_hash,
        user_id,
    )
    .await
    .map_err(crate::admin_error::AdminError::from)?;
    let session = crate::create_session_in_connection::create_session_in_connection(
        state.as_ref(),
        user_id,
        &context_hash,
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
    )
    .await
    .map_err(crate::admin_error::AdminError::session)?;
    let login =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_ACTIVE_USER_LOGIN_SQL)
            .bind(user_id.get())
            .fetch_optional(&mut *tx)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
            .map_err(crate::admin_error::AdminError::from)?
            .map(server_admin_contract::admin_login::AdminLogin::try_from)
            .transpose()
            .map_err(|_error| crate::admin_error::AdminError::Validation)?
            .ok_or(crate::admin_error::AdminError::Authentication)?;
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
            crate::admin_audit_action::AdminAuditAction::Refresh,
            &login,
            crate::admin_audit_resource::AdminAuditResource::Session,
            crate::admin_audit_resource_id::AdminAuditResourceId::Session(session.session_id()),
            user_id,
        ),
    )
    .await?;
    let authenticated = crate::load_authenticated_admin_from_db::load_authenticated_admin_from_db(
        &mut crate::admin_db_ref::AdminDbRef::Connection(
            crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        ),
        user_id,
        session.session_id(),
    )
    .await?;
    let authenticated_contract =
        crate::authenticated_admin_contract::authenticated_admin_contract(&authenticated)?;
    let mut response = crate::json_response::json_response(
        server_admin_contract::admin_sign_in_response::AdminSignInResponse::new(
            authenticated_contract,
        ),
    );
    crate::append_session_cookies::append_session_cookies(&mut response, state.as_ref(), &session)?;
    tx.commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    Ok(response)
}
