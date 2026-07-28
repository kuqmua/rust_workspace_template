#![allow(clippy::single_call_fn)] // route facade preserves utoipa inventory while private implementations own handler logic
fn map_unique_violation<Error>(value: Error) -> super::AdminError
where
    Error: Into<sqlx::Error>,
{
    let error = value.into();
    if pg_crud_common::classify_pg_error(pg_crud_common::SqlxPgErrorRef::from(&error))
        == pg_crud_common::PgErrorKind::UniqueViolation
    {
        super::AdminError::Conflict
    } else {
        super::AdminError::from(error)
    }
}
fn map_repository_error(
    repository_error: super::super::repository::AdminRepositoryError,
) -> super::AdminError {
    match repository_error {
        super::super::repository::AdminRepositoryError::InvalidStoredValue => {
            super::AdminError::Validation
        }
        super::super::repository::AdminRepositoryError::Sqlx(sqlx_error) => {
            super::AdminError::from(sqlx_error)
        }
    }
}
fn page_total(
    value: super::super::repository::AdminPageTotalCount,
) -> Result<server_admin_contract::AdminPageTotal, super::AdminError> {
    u64::try_from(value.get())
        .map(server_admin_contract::AdminPageTotal::from)
        .map_err(|_error| super::AdminError::Validation)
}
fn validate_table_sort(
    query: &server_admin_contract::AdminTableQuery,
    options: &[server_admin_contract::AdminTableSortField],
) -> Result<(), super::AdminError> {
    if query.sort().as_ref().is_empty() {
        return Ok(());
    }
    server_admin_contract::AdminTableSortField::try_from_key(
        options,
        server_admin_contract::AdminTableSortKeyRef::from(query.sort().as_ref()),
    )
    .map(drop)
    .map_err(|_error| super::AdminError::Validation)
}
async fn authenticate_mutation(
    auth: &super::AdminAuthReq,
) -> Result<super::AuthenticatedAdmin, super::AdminError> {
    let actor = super::authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await?;
    let subject = super::super::StdAdminString::try_from(actor.id.get().to_string())
        .map_err(|_error| super::AdminError::Validation)?;
    super::rate_limit::enforce_rate_limit(
        auth.state.as_ref(),
        super::rate_limit::AdminRateLimitScope::Mutation,
        &subject,
        auth.state.as_ref().policy.mutation_limit,
        auth.state.as_ref().policy.mutation_window,
    )
    .await?;
    super::validate_csrf(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        &actor,
    )
    .await?;
    Ok(actor)
}
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
    let mut response =
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            server_admin_contract::AdminSignInRes::new(authenticated_contract),
        )));
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
    let session = super::create_refreshed_session_in_connection(
        state.as_ref(),
        admin_user_id,
        &context_hash,
        super::super::AdminRefreshToken::new(token),
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
    )
    .await
    .map_err(super::AdminError::session)?;
    let login = super::super::repository::sessions::read_active_user_login(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        admin_user_id,
    )
    .await
    .map_err(map_repository_error)?
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
    let mut response =
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            server_admin_contract::AdminSignInRes::new(authenticated_contract),
        )));
    super::append_access_session_cookies(&mut response, state.as_ref(), &session)?;
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
pub(super) async fn me_view(
    auth: super::AdminAuthReq,
) -> Result<server_admin_contract::AuthenticatedAdmin, super::AdminError> {
    super::authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await
    .and_then(|authenticated| super::authenticated_admin_contract(&authenticated))
}
pub(super) async fn me(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    me_view(auth).await.map(|authenticated| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            authenticated,
        )))
    })
}
pub(super) async fn change_own_password(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::AdminChangeOwnPasswordReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = authenticate_mutation(&auth).await?;
    let (current_password, new_password) = request.0.into_parts();
    let expected_hash = super::super::repository::users::read_password_hash(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
        actor.id,
    )
    .await
    .map_err(super::AdminError::from)?
    .ok_or(super::AdminError::Authentication)?;
    if !auth
        .state
        .as_ref()
        .password_hasher
        .verify(
            super::admin_password_from_contract(current_password)
                .map_err(super::AdminError::password_text)?,
            expected_hash,
        )
        .await
        .map_err(super::AdminError::password_hash)?
        .get()
    {
        return Err(super::AdminError::Validation);
    }
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(
            super::admin_new_password_from_contract(new_password)
                .map_err(super::AdminError::password_text)?,
        )
        .await
        .map_err(super::AdminError::password_hash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::users::update_user_password(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        actor.id,
        &password_hash,
    )
    .await
    .map_err(super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    super::super::repository::sessions::revoke_other_access_sessions(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        actor.id,
        actor.session_id,
    )
    .await
    .map_err(super::AdminError::from)?;
    super::super::repository::sessions::revoke_user_refresh_tokens(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        actor.id,
    )
    .await
    .map_err(super::AdminError::from)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(actor.id),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn sessions_view(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<server_admin_contract::AdminSessionsPage, super::AdminError> {
    let authenticated = super::authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await?;
    super::super::repository::sessions::list_active_sessions(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
        authenticated.session_id,
        authenticated.id,
        &query.0,
    )
    .await
    .map_err(map_repository_error)
}
pub(super) async fn sessions(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    sessions_view(auth, query).await.map(|sessions| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            sessions,
        )))
    })
}
pub(super) async fn revoke_session(
    auth: super::AdminAuthReq,
    session: super::AdminSessionPath,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let authenticated = super::authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::validate_csrf(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        &authenticated,
    )
    .await?;
    super::super::repository::sessions::revoke_access_session(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        session.0,
        authenticated.id,
    )
    .await
    .map_err(super::AdminError::from)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Delete,
            login: &authenticated.login,
            resource: super::super::AdminAuditResource::Session,
            resource_id: super::AdminAuditResourceId::Session(session.0),
            user_id: authenticated.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn revoke_all_sessions(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let authenticated = super::authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await?;
    super::validate_csrf(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        &authenticated,
    )
    .await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::sessions::revoke_user_sessions(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        authenticated.id,
    )
    .await
    .map_err(super::AdminError::from)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Delete,
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
    super::append_cleared_session_cookies(&mut response, auth.state.as_ref())?;
    Ok(response)
}
pub(super) async fn update_settings(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::AdminUpdateSettingsReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::authorize_custom(&auth, super::super::AdminPermission::SystemSettingsUpdate).await?;
    if !bool::from(request.0.has_fields()) || !bool::from(request.0.is_valid()) {
        return Err(super::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::settings::update_settings(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        request.0,
    )
    .await
    .map_err(super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::SystemSettings,
            resource_id: super::AdminAuditResourceId::SystemSettings,
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn create_user(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::AdminCreateUserReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::UsersCreate).await?;
    let (contract_display_name, contract_login, contract_password) = request.0.into_parts();
    let display_name = super::super::AdminDisplayName::try_from(contract_display_name.into_inner())
        .map_err(|_error| super::AdminError::Validation)?;
    let login = super::super::AdminLogin::try_from(contract_login.into_inner())
        .map_err(|_error| super::AdminError::Validation)?;
    let password = super::admin_new_password_from_contract(contract_password)
        .map_err(super::AdminError::password_text)?;
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(password)
        .await
        .map_err(super::AdminError::password_hash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    let user_id = super::super::repository::users::insert_user(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &login,
        &display_name,
        &password_hash,
    )
    .await
    .map_err(|error| map_unique_violation(error.0))?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Create,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(super::super::AdminUserId::from(
                user_id.value(),
            )),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(server_admin_contract::AdminCreateUserRes::new(
                server_admin_contract::AdminUserId::from(user_id.value()),
            )),
        )),
    ))
}
pub(super) async fn update_user(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::AdminUpdateUserReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::UsersUpdate).await?;
    let (contract_display_name, contract_login) = request.0.into_parts();
    let display_name = contract_display_name
        .map(|value| super::super::AdminDisplayName::try_from(value.into_inner()))
        .transpose()
        .map_err(|_error| super::AdminError::Validation)?;
    let login = contract_login
        .map(|value| super::super::AdminLogin::try_from(value.into_inner()))
        .transpose()
        .map_err(|_error| super::AdminError::Validation)?;
    if login.is_none() && display_name.is_none() {
        return Err(super::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::users::update_user(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        login.as_ref(),
        display_name.as_ref(),
    )
    .await
    .map_err(|error| map_unique_violation(error.0))?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn set_user_password(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::AdminSetUserPasswordReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::UsersUpdate).await?;
    let password = super::admin_new_password_from_contract(request.0.into_password())
        .map_err(super::AdminError::password_text)?;
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(password)
        .await
        .map_err(super::AdminError::password_hash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::users::update_user_password(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        &password_hash,
    )
    .await
    .map_err(super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    super::super::repository::sessions::revoke_user_sessions(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(super::AdminError::from)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn set_user_ban(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::AdminSetUserBanReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::UsersUpdate).await?;
    let is_banned = bool::from(request.0.is_banned());
    if is_banned && actor.id == path.0 {
        return Err(super::AdminError::Conflict);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::roles::lock_last_admin(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
    )
    .await
    .map_err(super::AdminError::from)?;
    if is_banned {
        let last_admin_state = super::super::repository::roles::read_last_admin_state(
            super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            path.0,
        )
        .await
        .map_err(super::AdminError::from)?;
        if last_admin_state.would_remove_last().get() {
            return Err(super::AdminError::Conflict);
        }
    }
    super::super::repository::users::update_user_ban(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        super::super::StdAdminBool::from(is_banned),
    )
    .await
    .map_err(super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    if is_banned {
        super::super::repository::sessions::revoke_user_sessions(
            super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            path.0,
        )
        .await
        .map_err(super::AdminError::from)?;
    }
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn delete_user(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::UsersDelete).await?;
    if actor.id == path.0 {
        return Err(super::AdminError::Conflict);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    let last_admin_state = super::super::repository::roles::lock_and_read_last_admin_state(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(super::AdminError::from)?;
    if last_admin_state.would_remove_last().get() {
        return Err(super::AdminError::Conflict);
    }
    super::super::repository::users::delete_user(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Delete,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn create_role(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::AdminCreateRoleReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::RolesCreate).await?;
    let name = super::super::AdminRoleName::try_from(request.0.into_name().into_inner())
        .map_err(|_error| super::AdminError::Validation)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    let role_id = super::super::repository::roles::insert_role(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &name,
    )
    .await
    .map_err(|error| map_unique_violation(error.0))?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Create,
            login: &actor.login,
            resource: super::super::AdminAuditResource::Role,
            resource_id: super::AdminAuditResourceId::Role(role_id),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(server_admin_contract::AdminCreateRoleRes::new(
                server_admin_contract::AdminRoleId::from(role_id.value()),
            )),
        )),
    ))
}
pub(super) async fn update_role(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
    request: super::AxumAdminJson<server_admin_contract::AdminUpdateRoleReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::RolesUpdate).await?;
    let name = super::super::AdminRoleName::try_from(request.0.into_name().into_inner())
        .map_err(|_error| super::AdminError::Validation)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::roles::update_role(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        &name,
    )
    .await
    .map_err(|error| map_unique_violation(error.0))?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::Role,
            resource_id: super::AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn delete_role(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::RolesDelete).await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::roles::delete_role(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Delete,
            login: &actor.login,
            resource: super::super::AdminAuditResource::Role,
            resource_id: super::AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn set_role_permissions(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
    request: super::AxumAdminJson<server_admin_contract::AdminSetRolePermissionsReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::authorize_custom(&auth, super::super::AdminPermission::RolePermissionsUpdate)
            .await?;
    let (expected_permission_ids, contract_permission_ids) = request.0.into_parts();
    if AsRef::<[server_admin_contract::AdminPermissionId]>::as_ref(&expected_permission_ids)
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len()
        != AsRef::<[server_admin_contract::AdminPermissionId]>::as_ref(&expected_permission_ids)
            .len()
        || AsRef::<[server_admin_contract::AdminPermissionId]>::as_ref(&contract_permission_ids)
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != AsRef::<[server_admin_contract::AdminPermissionId]>::as_ref(&contract_permission_ids)
                .len()
    {
        return Err(super::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    let outcome = super::super::repository::permissions::replace_role_permissions(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        expected_permission_ids.as_ref(),
        contract_permission_ids.as_ref(),
    )
    .await
    .map_err(super::AdminError::from)?;
    match outcome {
        super::super::repository::ReplaceRolePermissionsOutcome::Updated => {}
        super::super::repository::ReplaceRolePermissionsOutcome::UnknownPermission => {
            return Err(super::AdminError::Validation);
        }
        super::super::repository::ReplaceRolePermissionsOutcome::MissingRole
        | super::super::repository::ReplaceRolePermissionsOutcome::StaleAssignment
        | super::super::repository::ReplaceRolePermissionsOutcome::SystemRole => {
            return Err(super::AdminError::Conflict);
        }
    }
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::Role,
            resource_id: super::AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn set_user_roles(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::AdminSetUserRolesReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::authorize_custom(&auth, super::super::AdminPermission::UserRolesUpdate).await?;
    let (expected_role_ids, contract_role_ids) = request.0.into_parts();
    if AsRef::<[server_admin_contract::AdminRoleId]>::as_ref(&expected_role_ids)
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len()
        != AsRef::<[server_admin_contract::AdminRoleId]>::as_ref(&expected_role_ids).len()
        || AsRef::<[server_admin_contract::AdminRoleId]>::as_ref(&contract_role_ids)
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != AsRef::<[server_admin_contract::AdminRoleId]>::as_ref(&contract_role_ids).len()
    {
        return Err(super::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    let outcome = super::super::repository::roles::replace_user_roles(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        expected_role_ids.as_ref(),
        contract_role_ids.as_ref(),
    )
    .await
    .map_err(super::AdminError::from)?;
    match outcome {
        super::super::repository::ReplaceUserRolesOutcome::Updated => {}
        super::super::repository::ReplaceUserRolesOutcome::UnknownRole => {
            return Err(super::AdminError::Validation);
        }
        super::super::repository::ReplaceUserRolesOutcome::LastActiveAdministrator
        | super::super::repository::ReplaceUserRolesOutcome::MissingUser
        | super::super::repository::ReplaceUserRolesOutcome::StaleAssignment => {
            return Err(super::AdminError::Conflict);
        }
    }
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn users_page(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<server_admin_contract::AdminUsersPage, super::AdminError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::UsersRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    validate_table_sort(&query.0, &server_admin_contract::AdminTableSortField::USER)?;
    let pool = super::super::repository::SqlxAdminRepositoryPoolRef::from(
        auth.state.as_ref().pool.as_ref(),
    );
    let (users, total) = super::super::repository::users::list_users(pool, &query.0)
        .await
        .map_err(map_repository_error)?;
    let roles = super::super::repository::roles::list_role_catalog(pool)
        .await
        .map_err(map_repository_error)?;
    Ok(server_admin_contract::AdminUsersPage::new(
        users,
        roles,
        page_total(total)?,
    ))
}
pub(super) async fn list_users(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    users_page(auth, query).await.map(|page| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            page,
        )))
    })
}
pub(super) async fn roles_page(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<server_admin_contract::AdminRolesPage, super::AdminError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::RolesRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    validate_table_sort(&query.0, &server_admin_contract::AdminTableSortField::ROLE)?;
    let pool = super::super::repository::SqlxAdminRepositoryPoolRef::from(
        auth.state.as_ref().pool.as_ref(),
    );
    let (roles, total) = super::super::repository::roles::list_roles(pool, &query.0)
        .await
        .map_err(map_repository_error)?;
    let permissions = super::super::repository::permissions::list_permission_catalog(pool)
        .await
        .map_err(map_repository_error)?;
    Ok(server_admin_contract::AdminRolesPage::new(
        roles,
        permissions,
        page_total(total)?,
    ))
}
pub(super) async fn list_roles(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    roles_page(auth, query).await.map(|page| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            page,
        )))
    })
}
pub(super) async fn permissions_page(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<server_admin_contract::AdminPermissionsPage, super::AdminError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::PermissionsRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    validate_table_sort(
        &query.0,
        &server_admin_contract::AdminTableSortField::PERMISSION,
    )?;
    let (permissions, total) = super::super::repository::permissions::list_permissions(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
        &query.0,
    )
    .await
    .map_err(map_repository_error)?;
    Ok(server_admin_contract::AdminPermissionsPage::new(
        permissions,
        page_total(total)?,
    ))
}
pub(super) async fn list_permissions(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    permissions_page(auth, query).await.map(|page| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            page,
        )))
    })
}
pub(super) async fn settings_view(
    auth: super::AdminAuthReq,
) -> Result<server_admin_contract::AdminSettingsView, super::AdminError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::SystemSettingsRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    super::super::repository::settings::read_settings(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
    )
    .await
    .map_err(map_repository_error)
}
pub(super) async fn settings(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    settings_view(auth).await.map(|view| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            view,
        )))
    })
}
pub(super) async fn branding(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    branding_view(auth).await.map(|view| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            view,
        )))
    })
}
pub(super) async fn data_tables(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    data_table_catalog(auth).await.map(|catalog| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            catalog,
        )))
    })
}
pub(super) async fn data_table_catalog(
    auth: super::AdminAuthReq,
) -> Result<server_admin_contract::AdminDataTableCatalog, super::AdminError> {
    let actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::TablesRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let admin = super::authenticated_admin_contract(&actor)?;
    let items = server_admin_contract::AdminDataTable::ALL
        .into_iter()
        .filter(|table| bool::from(admin.has_permission(table.permission())))
        .collect::<Vec<_>>();
    Ok(server_admin_contract::AdminDataTableCatalog::new(
        server_admin_contract::AdminDataTables::try_from(items)
            .map_err(|_error| super::AdminError::Validation)?,
    ))
}
pub(super) async fn data_table_view(
    auth: super::AdminAuthReq,
    table: server_admin_contract::AdminDataTable,
    query: &server_admin_contract::AdminDataTableQuery,
) -> Result<server_admin_contract::AdminDataTableView, super::AdminError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        table.permission().as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    super::super::repository::data_tables::read(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
        table,
        query,
    )
    .await
    .map_err(map_repository_error)
}
pub(super) async fn data_table(
    auth: super::AdminAuthReq,
    super::AxumAdminPath(table): super::AxumAdminPath<server_admin_contract::AdminDataTable>,
    super::AxumAdminQuery(query): super::AxumAdminQuery<server_admin_contract::AdminDataTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    data_table_view(auth, table, &query).await.map(|view| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            view,
        )))
    })
}
pub(super) async fn branding_view(
    auth: super::AdminAuthReq,
) -> Result<server_admin_contract::AdminBrandingView, super::AdminError> {
    let settings = super::super::repository::settings::read_settings(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
    )
    .await
    .map_err(map_repository_error)?;
    Ok(server_admin_contract::AdminBrandingView::from_settings(
        &settings,
    ))
}
