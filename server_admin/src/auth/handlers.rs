#![allow(clippy::single_call_fn)] // route facade preserves utoipa inventory while private implementations own handler logic
fn map_unique_violation<Error>(value: Error) -> super::AdminApiError
where
    Error: Into<sqlx::Error>,
{
    let error = value.into();
    if error
        .as_database_error()
        .is_some_and(sqlx::error::DatabaseError::is_unique_violation)
    {
        super::AdminApiError::Conflict
    } else {
        super::AdminApiError::from(error)
    }
}
pub(super) async fn sign_in(
    auth: super::AdminAuthReq,
    peer: super::AdminPeerAddr,
    request_json: super::AdminSignInJson,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let state = auth.state;
    let headers = auth.headers;
    if !super::origin_is_present_and_allowed(
        state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
    )
    .0
    {
        return Err(super::AdminApiError::Authentication);
    }
    let request = request_json.0;
    let (contract_login, contract_password) = request.into_parts();
    let login = super::super::AdminLogin::try_from(contract_login.into_inner())
        .map_err(|_error| super::AdminApiError::Validation)?;
    let password = super::admin_password_from_contract(contract_password);
    let peer_subject = super::super::StdAdminString::try_from(peer.0.as_ref().ip().to_string())
        .map_err(|_error| super::AdminApiError::Validation)?;
    super::rate_limit::enforce_rate_limit(
        state.as_ref(),
        super::rate_limit::AdminRateLimitScope::SignInIp,
        &peer_subject,
        super::rate_limit::StdAdminRateLimitCount::from(
            state.as_ref().sign_in_rate_limit.0.saturating_mul(5i64),
        ),
        super::rate_limit::StdAdminRateLimitWindowSeconds::from(900i32),
    )
    .await?;
    let pair_subject = super::super::StdAdminString::try_from(format!(
        "{}|{}",
        peer.0.as_ref().ip(),
        login.as_ref()
    ))
    .map_err(|_error| super::AdminApiError::Validation)?;
    super::rate_limit::enforce_rate_limit(
        state.as_ref(),
        super::rate_limit::AdminRateLimitScope::SignInIpLogin,
        &pair_subject,
        state.as_ref().sign_in_rate_limit,
        super::rate_limit::StdAdminRateLimitWindowSeconds::from(900i32),
    )
    .await?;
    let recent_failures = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_COUNT_ASTERISK_FROM_ADMIN_LOGIN_ATTEMPTS_WHERE_LOGIN_DOLLAR_1_AND,
    )
    .bind(login.as_ref())
    .fetch_one(state.as_ref().pool.as_ref())
    .await
    .map_err(super::AdminApiError::from)?;
    if recent_failures >= 10i64 {
        return Err(super::AdminApiError::RateLimited);
    }
    let user = sqlx::query_as::<_, (i64, String, bool)>(
        str_constants::SELECT_ID_PASSWORD_HASH_IS_BANNED_FROM_ADMIN_USERS_WHERE_LOWER_LOGIN,
    )
    .bind(login.as_ref())
    .fetch_optional(state.as_ref().pool.as_ref())
    .await
    .map_err(super::AdminApiError::from)?;
    let Some((user_id, password_hash, is_banned)) = user else {
        drop(
            state
                .as_ref()
                .password_hasher
                .hash(password)
                .await
                .map_err(super::AdminApiError::PasswordHash)?,
        );
        super::record_login_attempt(
            state.as_ref(),
            &login,
            peer,
            super::super::StdAdminBool::from(false),
        )
        .await?;
        return Err(super::AdminApiError::Authentication);
    };
    let verified = state
        .as_ref()
        .password_hasher
        .verify(
            password,
            super::super::AdminPasswordHash::new(
                pg_types_text_misc::StringAsNonNullTextSecret::from(password_hash),
            ),
        )
        .await
        .map_err(|_error| super::AdminApiError::Authentication)?;
    if !verified.0 || is_banned {
        super::record_login_attempt(
            state.as_ref(),
            &login,
            peer,
            super::super::StdAdminBool::from(false),
        )
        .await?;
        return Err(super::AdminApiError::Authentication);
    }
    super::record_login_attempt(
        state.as_ref(),
        &login,
        peer,
        super::super::StdAdminBool::from(true),
    )
    .await?;
    let admin_user_id = super::super::AdminUserId::from(user_id);
    let context_hash = super::session_context_hash(
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        peer,
    );
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminApiError::from)?;
    let session = super::create_session_in_connection(
        state.as_ref(),
        admin_user_id,
        &context_hash,
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
    )
    .await
    .map_err(super::AdminApiError::Session)?;
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
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
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let state = auth.state;
    let headers = auth.headers;
    if !super::origin_is_present_and_allowed(
        state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
    )
    .0
    {
        apply_refresh_failure_delay().await;
        return Err(super::AdminApiError::Authentication);
    }
    let peer_subject = super::super::StdAdminString::try_from(peer.0.as_ref().ip().to_string())
        .map_err(|_error| super::AdminApiError::Validation)?;
    super::rate_limit::enforce_rate_limit(
        state.as_ref(),
        super::rate_limit::AdminRateLimitScope::RefreshIp,
        &peer_subject,
        super::rate_limit::StdAdminRateLimitCount::from(60i64),
        super::rate_limit::StdAdminRateLimitWindowSeconds::from(900i32),
    )
    .await?;
    let Some(raw_token) = super::super::find_admin_cookie(
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        super::super::AdminCookieKind::Refresh,
    ) else {
        apply_refresh_failure_delay().await;
        return Err(super::AdminApiError::Authentication);
    };
    let token = super::super::AdminOpaqueToken::new(super::super::SecrecyAdminString::from(
        secrecy::SecretBox::new(Box::new(raw_token.as_ref().to_owned())),
    ));
    let context_hash = super::session_context_hash(
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        peer,
    );
    let token_hash = super::hash_refresh_token_with_context(&token, &context_hash);
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminApiError::from)?;
    let optional_user_id = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_USER_ID_FROM_ADMIN_REFRESH_TOKENS_WHERE_TOKEN_HASH_DOLLAR_1,
    )
    .bind(secrecy::ExposeSecret::expose_secret(token_hash.0.as_ref()))
    .fetch_optional(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?;
    let Some(user_id) = optional_user_id else {
        tx.commit().await.map_err(super::AdminApiError::from)?;
        apply_refresh_failure_delay().await;
        return Err(super::AdminApiError::Authentication);
    };
    let admin_user_id = super::super::AdminUserId::from(user_id);
    let session = super::create_refreshed_session_in_connection(
        state.as_ref(),
        admin_user_id,
        &context_hash,
        super::super::AdminRefreshToken::new(token),
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
    )
    .await
    .map_err(super::AdminApiError::Session)?;
    let login = sqlx::query_scalar::<_, String>(
        str_constants::SELECT_LOGIN_FROM_ADMIN_USERS_WHERE_ID_DOLLAR_1_AND_IS_BANNED,
    )
    .bind(admin_user_id.0)
    .fetch_optional(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?
    .ok_or(super::AdminApiError::Authentication)
    .and_then(|value| {
        super::super::AdminLogin::try_from(value).map_err(|_error| super::AdminApiError::Validation)
    })?;
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
        &mut super::AdminAuthDbRef::Connection(super::SqlxAdminPgConnectionRef::from(&mut *tx)),
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
    Ok(response)
}
async fn apply_refresh_failure_delay() {
    tokio::time::sleep(tokio::time::Duration::from_millis(200u64)).await;
}
pub(super) async fn sign_out(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
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
        .map_err(super::AdminApiError::from)?;
    let _access_result = sqlx::query(str_constants::SERVER_ADMIN_REVOKE_ACCESS_SESSION_SQL)
        .bind(authenticated.session_id.0.0)
        .bind(authenticated.id.0)
        .execute(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
    if let Some(raw_refresh) = super::super::find_admin_cookie(
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        super::super::AdminCookieKind::Refresh,
    ) {
        let refresh = super::super::AdminOpaqueToken::new(super::super::SecrecyAdminString::from(
            secrecy::SecretBox::new(Box::new(raw_refresh.as_ref().to_owned())),
        ));
        let context_hash = super::session_context_hash(
            super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
            peer,
        );
        let refresh_hash = super::hash_refresh_token_with_context(&refresh, &context_hash);
        let _refresh_result = sqlx::query(
            str_constants::UPDATE_ADMIN_REFRESH_TOKENS_SET_REVOKED_AT_NOW_WHERE_TOKEN_HASH_DOLLAR,
        )
        .bind(secrecy::ExposeSecret::expose_secret(
            refresh_hash.0.as_ref(),
        ))
        .bind(authenticated.id.0)
        .execute(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
    let mut response = super::AxumAdminResponse(axum::response::IntoResponse::into_response(
        http::StatusCode::NO_CONTENT,
    ));
    super::append_cleared_session_cookies(&mut response, state.as_ref())?;
    Ok(response)
}
pub(super) async fn me(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    super::authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await
    .and_then(|authenticated| super::authenticated_admin_contract(&authenticated))
    .map(|authenticated| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            authenticated,
        )))
    })
}
pub(super) async fn sessions(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let authenticated = super::authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await?;
    sqlx::query_as::<_, (uuid::Uuid, String, String)>(
        str_constants::SELECT_ID_CREATED_AT_PATH_TEXT_EXPIRES_AT_PATH_TEXT_FROM_ADMIN,
    )
    .bind(authenticated.id.0)
    .fetch_all(auth.state.as_ref().pool.as_ref())
    .await
    .map_err(super::AdminApiError::from)?
    .into_iter()
    .map(|row| {
        Ok(super::AdminSessionView {
            created_at: super::AdminSessionTimestamp::try_from(row.1)
                .map_err(|_error| super::AdminApiError::Authentication)?,
            expires_at: super::AdminSessionTimestamp::try_from(row.2)
                .map_err(|_error| super::AdminApiError::Authentication)?,
            id: super::super::AdminSessionId::from(super::super::UuidAdminValue::from(row.0)),
        })
    })
    .collect::<Result<Vec<super::AdminSessionView>, super::AdminApiError>>()
    .map(|sessions| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            sessions,
        )))
    })
}
pub(super) async fn revoke_session(
    auth: super::AdminAuthReq,
    session: super::AdminSessionPath,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
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
        .map_err(super::AdminApiError::from)?;
    super::validate_csrf(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        &authenticated,
    )
    .await?;
    let _result = sqlx::query(str_constants::SERVER_ADMIN_REVOKE_ACCESS_SESSION_SQL)
        .bind(session.0.0.0)
        .bind(authenticated.id.0)
        .execute(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn revoke_all_sessions(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
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
        .map_err(super::AdminApiError::from)?;
    let _access_result = sqlx::query(str_constants::SERVER_ADMIN_REVOKE_USER_ACCESS_SESSIONS_SQL)
        .bind(authenticated.id.0)
        .execute(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
    let _refresh_result = sqlx::query(str_constants::SERVER_ADMIN_REVOKE_USER_REFRESH_TOKENS_SQL)
        .bind(authenticated.id.0)
        .execute(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
    let mut response = super::AxumAdminResponse(axum::response::IntoResponse::into_response(
        http::StatusCode::NO_CONTENT,
    ));
    super::append_cleared_session_cookies(&mut response, auth.state.as_ref())?;
    Ok(response)
}
pub(super) async fn update_settings(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::AdminUpdateSettingsReq>,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let actor =
        super::authorize_custom(&auth, super::super::AdminPermission::SystemSettingsUpdate).await?;
    let (
        default_admin_route,
        main_logo,
        organization_contacts,
        organization_name,
        primary_color,
        site_name,
        support_url,
        tab_title,
    ) = request.0.into_parts();
    let has_field = [
        default_admin_route.is_some(),
        main_logo.is_some(),
        organization_contacts.is_some(),
        organization_name.is_some(),
        primary_color.is_some(),
        site_name.is_some(),
        support_url.is_some(),
        tab_title.is_some(),
    ]
    .into_iter()
    .any(std::convert::identity);
    let site_name_is_valid = site_name
        .as_ref()
        .is_none_or(|value| !value.as_ref().trim().is_empty());
    let route_is_valid = default_admin_route.as_ref().is_none_or(|value| {
        value
            .as_ref()
            .starts_with(str_constants::ADMIN_PAGE_PATHS_ROOT)
    });
    if !has_field || !site_name_is_valid || !route_is_valid {
        return Err(super::AdminApiError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminApiError::from)?;
    sqlx::query_scalar::<_, bool>(
        str_constants::UPDATE_ADMIN_SYSTEM_SETTINGS_SET_SITE_NAME_COALESCE_DOLLAR_1_SITE_NAME,
    )
    .bind(site_name.as_ref().map(|value| value.as_ref().as_str()))
    .bind(tab_title.as_ref().map(|value| value.as_ref().as_str()))
    .bind(main_logo.as_ref().map(|value| value.as_ref().as_str()))
    .bind(primary_color.as_ref().map(|value| value.as_ref().as_str()))
    .bind(
        default_admin_route
            .as_ref()
            .map(|value| value.as_ref().as_str()),
    )
    .bind(
        organization_name
            .as_ref()
            .map(|value| value.as_ref().as_str()),
    )
    .bind(
        organization_contacts
            .as_ref()
            .map(|value| value.as_ref().as_str()),
    )
    .bind(support_url.as_ref().map(|value| value.as_ref().as_str()))
    .fetch_optional(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?
    .ok_or(super::AdminApiError::Conflict)
    .map(drop)?;
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn create_user(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::AdminCreateUserReq>,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::UsersCreate).await?;
    let (contract_display_name, contract_login, contract_password) = request.0.into_parts();
    let display_name = super::super::AdminDisplayName::try_from(contract_display_name.into_inner())
        .map_err(|_error| super::AdminApiError::Validation)?;
    let login = super::super::AdminLogin::try_from(contract_login.into_inner())
        .map_err(|_error| super::AdminApiError::Validation)?;
    let password = super::admin_password_from_contract(contract_password);
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(password)
        .await
        .map_err(super::AdminApiError::PasswordHash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminApiError::from)?;
    let user_id = sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_INSERT_USER_SQL)
        .bind(login.as_ref())
        .bind(display_name.as_ref())
        .bind(password_hash.0.as_ref())
        .fetch_one(&mut *tx)
        .await
        .map_err(map_unique_violation)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Create,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(super::super::AdminUserId::from(
                user_id,
            )),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminApiError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(server_admin_contract::AdminCreateUserRes::new(
                server_admin_contract::AdminUserId::from(user_id),
            )),
        )),
    ))
}
pub(super) async fn update_user(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::AdminUpdateUserReq>,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::UsersUpdate).await?;
    let (contract_display_name, contract_login) = request.0.into_parts();
    let display_name = contract_display_name
        .map(|value| super::super::AdminDisplayName::try_from(value.into_inner()))
        .transpose()
        .map_err(|_error| super::AdminApiError::Validation)?;
    let login = contract_login
        .map(|value| super::super::AdminLogin::try_from(value.into_inner()))
        .transpose()
        .map_err(|_error| super::AdminApiError::Validation)?;
    if login.is_none() && display_name.is_none() {
        return Err(super::AdminApiError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminApiError::from)?;
    sqlx::query_scalar::<_, bool>(
        str_constants::UPDATE_ADMIN_USERS_SET_LOGIN_COALESCE_DOLLAR_2_LOGIN_DISPLAY_NAME_COALESCE,
    )
    .bind(path.0.0)
    .bind(login.as_ref().map(|value| value.as_ref().as_str()))
    .bind(display_name.as_ref().map(|value| value.as_ref().as_str()))
    .fetch_optional(&mut *tx)
    .await
    .map_err(map_unique_violation)?
    .ok_or(super::AdminApiError::Conflict)
    .map(drop)?;
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn set_user_password(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::AdminSetUserPasswordReq>,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::UsersUpdate).await?;
    let password = super::admin_password_from_contract(request.0.into_password());
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(password)
        .await
        .map_err(super::AdminApiError::PasswordHash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminApiError::from)?;
    sqlx::query_scalar::<_, bool>(
        str_constants::UPDATE_ADMIN_USERS_SET_PASSWORD_HASH_DOLLAR_2_WHERE_ID_DOLLAR_1,
    )
    .bind(path.0.0)
    .bind(password_hash.0.as_ref())
    .fetch_optional(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?
    .ok_or(super::AdminApiError::Conflict)
    .map(drop)?;
    let _access = sqlx::query(str_constants::SERVER_ADMIN_REVOKE_USER_ACCESS_SESSIONS_SQL)
        .bind(path.0.0)
        .execute(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
    let _refresh = sqlx::query(str_constants::SERVER_ADMIN_REVOKE_USER_REFRESH_TOKENS_SQL)
        .bind(path.0.0)
        .execute(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn set_user_ban(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::AdminSetUserBanReq>,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::UsersUpdate).await?;
    let is_banned = bool::from(request.0.is_banned());
    if is_banned && actor.id == path.0 {
        return Err(super::AdminApiError::Conflict);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminApiError::from)?;
    let _lock = sqlx::query(str_constants::SERVER_ADMIN_LOCK_LAST_ADMIN_SQL)
        .execute(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
    if is_banned {
        let target_is_admin =
            sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_USER_IS_ADMIN_SQL)
                .bind(path.0.0)
                .fetch_one(&mut *tx)
                .await
                .map_err(super::AdminApiError::from)?;
        let active_admin_count =
            sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_ACTIVE_ADMIN_COUNT_SQL)
                .fetch_one(&mut *tx)
                .await
                .map_err(super::AdminApiError::from)?;
        if target_is_admin && active_admin_count <= 1i64 {
            return Err(super::AdminApiError::Conflict);
        }
    }
    sqlx::query_scalar::<_, bool>(
        str_constants::UPDATE_ADMIN_USERS_SET_IS_BANNED_DOLLAR_2_WHERE_ID_DOLLAR_1,
    )
    .bind(path.0.0)
    .bind(is_banned)
    .fetch_optional(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?
    .ok_or(super::AdminApiError::Conflict)
    .map(drop)?;
    if is_banned {
        let _access = sqlx::query(str_constants::SERVER_ADMIN_REVOKE_USER_ACCESS_SESSIONS_SQL)
            .bind(path.0.0)
            .execute(&mut *tx)
            .await
            .map_err(super::AdminApiError::from)?;
        let _refresh = sqlx::query(str_constants::SERVER_ADMIN_REVOKE_USER_REFRESH_TOKENS_SQL)
            .bind(path.0.0)
            .execute(&mut *tx)
            .await
            .map_err(super::AdminApiError::from)?;
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn delete_user(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::UsersDelete).await?;
    if actor.id == path.0 {
        return Err(super::AdminApiError::Conflict);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminApiError::from)?;
    let _lock = sqlx::query(str_constants::SERVER_ADMIN_LOCK_LAST_ADMIN_SQL)
        .execute(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
    let target_is_admin =
        sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_USER_IS_ADMIN_SQL)
            .bind(path.0.0)
            .fetch_one(&mut *tx)
            .await
            .map_err(super::AdminApiError::from)?;
    let active_admin_count =
        sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_ACTIVE_ADMIN_COUNT_SQL)
            .fetch_one(&mut *tx)
            .await
            .map_err(super::AdminApiError::from)?;
    if target_is_admin && active_admin_count <= 1i64 {
        return Err(super::AdminApiError::Conflict);
    }
    sqlx::query_scalar::<_, bool>(
        str_constants::DELETE_FROM_ADMIN_USERS_WHERE_ID_DOLLAR_1_RETURNING_TRUE,
    )
    .bind(path.0.0)
    .fetch_optional(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?
    .ok_or(super::AdminApiError::Conflict)
    .map(drop)?;
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn create_role(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::AdminCreateRoleReq>,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::RolesCreate).await?;
    let name = super::super::AdminRoleName::try_from(request.0.into_name().into_inner())
        .map_err(|_error| super::AdminApiError::Validation)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminApiError::from)?;
    let role_id = sqlx::query_scalar::<_, i64>(
        str_constants::INSERT_INTO_ADMIN_ROLES_NAME_IS_SYSTEM_VALUES_DOLLAR_1_FALSE_RETURNING,
    )
    .bind(name.as_ref())
    .fetch_one(&mut *tx)
    .await
    .map_err(map_unique_violation)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Create,
            login: &actor.login,
            resource: super::super::AdminAuditResource::Role,
            resource_id: super::AdminAuditResourceId::Role(super::super::AdminRoleId::from(
                role_id,
            )),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminApiError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(server_admin_contract::AdminCreateRoleRes::new(
                server_admin_contract::AdminRoleId::from(role_id),
            )),
        )),
    ))
}
pub(super) async fn update_role(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
    request: super::AxumAdminJson<server_admin_contract::AdminUpdateRoleReq>,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::RolesUpdate).await?;
    let name = super::super::AdminRoleName::try_from(request.0.into_name().into_inner())
        .map_err(|_error| super::AdminApiError::Validation)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminApiError::from)?;
    sqlx::query_scalar::<_, bool>(
        str_constants::UPDATE_ADMIN_ROLES_SET_NAME_DOLLAR_2_WHERE_ID_DOLLAR_1_AND,
    )
    .bind(path.0.0)
    .bind(name.as_ref())
    .fetch_optional(&mut *tx)
    .await
    .map_err(map_unique_violation)?
    .ok_or(super::AdminApiError::Conflict)
    .map(drop)?;
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn delete_role(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::RolesDelete).await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminApiError::from)?;
    sqlx::query_scalar::<_, bool>(
        str_constants::DELETE_FROM_ADMIN_ROLES_WHERE_ID_DOLLAR_1_AND_IS_SYSTEM_FALSE,
    )
    .bind(path.0.0)
    .fetch_optional(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?
    .ok_or(super::AdminApiError::Conflict)
    .map(drop)?;
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn set_role_permissions(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
    request: super::AxumAdminJson<server_admin_contract::AdminSetRolePermissionsReq>,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let actor =
        super::authorize_custom(&auth, super::super::AdminPermission::RolePermissionsUpdate)
            .await?;
    let contract_permission_ids = request.0.into_ids();
    if contract_permission_ids
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len()
        != contract_permission_ids.len()
    {
        return Err(super::AdminApiError::Validation);
    }
    let permission_ids = contract_permission_ids
        .into_iter()
        .map(i64::from)
        .collect::<Vec<i64>>();
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminApiError::from)?;
    let role_is_system = sqlx::query_scalar::<_, bool>(
        str_constants::SELECT_IS_SYSTEM_FROM_ADMIN_ROLES_WHERE_ID_DOLLAR_1_FOR_UPDATE,
    )
    .bind(path.0.0)
    .fetch_optional(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?
    .ok_or(super::AdminApiError::Conflict)?;
    if role_is_system {
        return Err(super::AdminApiError::Conflict);
    }
    let existing_permissions = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_COUNT_ASTERISK_FROM_ADMIN_PERMISSIONS_WHERE_ID_ANY_DOLLAR_1,
    )
    .bind(&permission_ids)
    .fetch_one(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?;
    if usize::try_from(existing_permissions).ok() != Some(permission_ids.len()) {
        return Err(super::AdminApiError::Validation);
    }
    let _deleted =
        sqlx::query(str_constants::DELETE_FROM_ADMIN_ROLE_PERMISSIONS_WHERE_ROLE_ID_DOLLAR_1)
            .bind(path.0.0)
            .execute(&mut *tx)
            .await
            .map_err(super::AdminApiError::from)?;
    let _inserted = sqlx::query(
        str_constants::INSERT_INTO_ADMIN_ROLE_PERMISSIONS_ROLE_ID_PERMISSION_ID_SELECT_DOLLAR_1,
    )
    .bind(path.0.0)
    .bind(&permission_ids)
    .execute(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?;
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn set_user_roles(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::AdminSetUserRolesReq>,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let actor =
        super::authorize_custom(&auth, super::super::AdminPermission::UserRolesUpdate).await?;
    let contract_role_ids = request.0.into_ids();
    if contract_role_ids
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len()
        != contract_role_ids.len()
    {
        return Err(super::AdminApiError::Validation);
    }
    let role_ids = contract_role_ids
        .into_iter()
        .map(i64::from)
        .collect::<Vec<i64>>();
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminApiError::from)?;
    let _lock = sqlx::query(str_constants::SERVER_ADMIN_LOCK_LAST_ADMIN_SQL)
        .execute(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
    let target_is_active = sqlx::query_scalar::<_, bool>(
        str_constants::SELECT_NOT_IS_BANNED_FROM_ADMIN_USERS_WHERE_ID_DOLLAR_1_FOR,
    )
    .bind(path.0.0)
    .fetch_optional(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?
    .ok_or(super::AdminApiError::Conflict)?;
    let existing_roles = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_COUNT_ASTERISK_FROM_ADMIN_ROLES_WHERE_ID_ANY_DOLLAR_1,
    )
    .bind(&role_ids)
    .fetch_one(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?;
    if usize::try_from(existing_roles).ok() != Some(role_ids.len()) {
        return Err(super::AdminApiError::Validation);
    }
    let admin_role_id = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_ID_FROM_ADMIN_ROLES_WHERE_NAME_ADMIN_AND_IS_SYSTEM_TRUE,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?;
    let target_was_admin = sqlx::query_scalar::<_, bool>(
        str_constants::SELECT_EXISTS_SELECT_1_FROM_ADMIN_USER_ROLES_WHERE_USER_ID_DOLLAR,
    )
    .bind(path.0.0)
    .bind(admin_role_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?;
    if target_is_active && target_was_admin && !role_ids.contains(&admin_role_id) {
        let active_admin_count = sqlx::query_scalar::<_, i64>(
            str_constants::SELECT_COUNT_DISTINCT_USERS_ID_FROM_ADMIN_USERS_USERS_JOIN_ADMIN_USER,
        )
        .bind(admin_role_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
        if active_admin_count <= 1i64 {
            return Err(super::AdminApiError::Conflict);
        }
    }
    let _deleted = sqlx::query(str_constants::DELETE_FROM_ADMIN_USER_ROLES_WHERE_USER_ID_DOLLAR_1)
        .bind(path.0.0)
        .execute(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
    let _inserted = sqlx::query(
        str_constants::INSERT_INTO_ADMIN_USER_ROLES_USER_ID_ROLE_ID_SELECT_DOLLAR_1_ALT,
    )
    .bind(path.0.0)
    .bind(&role_ids)
    .execute(&mut *tx)
    .await
    .map_err(super::AdminApiError::from)?;
    let _access = sqlx::query(str_constants::SERVER_ADMIN_REVOKE_USER_ACCESS_SESSIONS_SQL)
        .bind(path.0.0)
        .execute(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
    let _refresh = sqlx::query(str_constants::SERVER_ADMIN_REVOKE_USER_REFRESH_TOKENS_SQL)
        .bind(path.0.0)
        .execute(&mut *tx)
        .await
        .map_err(super::AdminApiError::from)?;
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
    tx.commit().await.map_err(super::AdminApiError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn list_users(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::UsersRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let rows = sqlx::query_as::<_, (i64, String, String, bool)>(
        str_constants::SELECT_ID_LOGIN_DISPLAY_NAME_IS_BANNED_FROM_ADMIN_USERS_ORDER_BY,
    )
    .fetch_all(auth.state.as_ref().pool.as_ref())
    .await
    .map_err(super::AdminApiError::from)?;
    let users = rows
        .into_iter()
        .map(|row| {
            Ok(server_admin_contract::AdminUserSummary::new(
                server_admin_contract::AdminDisplayName::try_from(row.2)
                    .map_err(|_error| super::AdminApiError::Validation)?,
                server_admin_contract::AdminUserId::from(row.0),
                server_admin_contract::AdminBool::from(row.3),
                server_admin_contract::AdminLogin::try_from(row.1)
                    .map_err(|_error| super::AdminApiError::Validation)?,
            ))
        })
        .collect::<Result<Vec<server_admin_contract::AdminUserSummary>, super::AdminApiError>>()?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(axum::Json(users)),
    ))
}
pub(super) async fn list_roles(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::RolesRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let rows = sqlx::query_as::<_, (i64, String, bool)>(
        str_constants::SELECT_ID_NAME_IS_SYSTEM_FROM_ADMIN_ROLES_ORDER_BY_NAME,
    )
    .fetch_all(auth.state.as_ref().pool.as_ref())
    .await
    .map_err(super::AdminApiError::from)?;
    let roles = rows
        .into_iter()
        .map(|row| {
            Ok(server_admin_contract::AdminRoleSummary::new(
                server_admin_contract::AdminRoleId::from(row.0),
                server_admin_contract::AdminBool::from(row.2),
                server_admin_contract::AdminRoleName::try_from(row.1)
                    .map_err(|_error| super::AdminApiError::Validation)?,
            ))
        })
        .collect::<Result<Vec<server_admin_contract::AdminRoleSummary>, super::AdminApiError>>()?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(axum::Json(roles)),
    ))
}
pub(super) async fn list_permissions(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::PermissionsRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let rows = sqlx::query_as::<_, (i64, String)>(
        str_constants::SELECT_ID_NAME_FROM_ADMIN_PERMISSIONS_ORDER_BY_NAME,
    )
    .fetch_all(auth.state.as_ref().pool.as_ref())
    .await
    .map_err(super::AdminApiError::from)?;
    let permissions = rows
        .into_iter()
        .map(|row| {
            Ok(server_admin_contract::AdminPermissionSummary::new(
                server_admin_contract::AdminPermissionId::from(row.0),
                server_admin_contract::AdminPermissionValue::try_from(row.1)
                    .map_err(|_error| super::AdminApiError::Validation)?,
            ))
        })
        .collect::<Result<Vec<server_admin_contract::AdminPermissionSummary>, super::AdminApiError>>(
        )?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(axum::Json(permissions)),
    ))
}
pub(super) async fn settings(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::SystemSettingsRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let row = sqlx::query_as::<
        _,
        (
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    >(
        str_constants::SELECT_SITE_NAME_TAB_TITLE_MAIN_LOGO_PRIMARY_COLOR_DEFAULT_ADMIN_ROUTE,
    )
    .fetch_one(auth.state.as_ref().pool.as_ref())
    .await
    .map_err(super::AdminApiError::from)?;
    let view = server_admin_contract::AdminSettingsView::new(
        server_admin_contract::AdminSettingText::try_from(row.4)
            .map_err(|_error| super::AdminApiError::Validation)?,
        row.2
            .map(server_admin_contract::AdminSettingText::try_from)
            .transpose()
            .map_err(|_error| super::AdminApiError::Validation)?,
        row.6
            .map(server_admin_contract::AdminSettingText::try_from)
            .transpose()
            .map_err(|_error| super::AdminApiError::Validation)?,
        row.5
            .map(server_admin_contract::AdminSettingText::try_from)
            .transpose()
            .map_err(|_error| super::AdminApiError::Validation)?,
        row.3
            .map(server_admin_contract::AdminSettingText::try_from)
            .transpose()
            .map_err(|_error| super::AdminApiError::Validation)?,
        server_admin_contract::AdminSettingText::try_from(row.0)
            .map_err(|_error| super::AdminApiError::Validation)?,
        row.7
            .map(server_admin_contract::AdminSettingText::try_from)
            .transpose()
            .map_err(|_error| super::AdminApiError::Validation)?,
        row.1
            .map(server_admin_contract::AdminSettingText::try_from)
            .transpose()
            .map_err(|_error| super::AdminApiError::Validation)?,
    );
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(axum::Json(view)),
    ))
}
