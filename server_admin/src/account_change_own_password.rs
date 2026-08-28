pub(crate) async fn account_change_own_password(
    auth: crate::AdminAuthReq,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminChangeOwnPasswordReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let actor = crate::authorization_authenticate::authorization_authenticate(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await?;
    let subject = crate::StdAdminString::try_from(actor.id.get().to_string())
        .map_err(|_error| crate::AdminError::Validation)?;
    crate::rate_limit::enforce_rate_limit(
        auth.state.as_ref(),
        crate::rate_limit::AdminRateLimitScope::Mutation,
        &subject,
        auth.state.as_ref().policy.mutation_limit,
        auth.state.as_ref().policy.mutation_window,
    )
    .await?;
    crate::authorization_validate_csrf::authorization_validate_csrf(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        &actor,
    )
    .await?;
    let (current_password, new_password) = request.0.into_parts();
    let expected_hash =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_PASSWORD_HASH_SQL)
            .bind(actor.id.get())
            .fetch_optional(auth.state.as_ref().pool.as_ref())
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .map(|value| {
                value.map(|hash| {
                    crate::AdminPasswordHash::new(
                        pg_types_text_misc::StringAsNonNullTextSecret::from(hash),
                    )
                })
            })
            .map_err(crate::AdminError::from)?
            .ok_or(crate::AdminError::Authentication)?;
    if !auth
        .state
        .as_ref()
        .password_hasher
        .verify(
            crate::admin_password_from_contract(current_password)
                .map_err(crate::AdminError::password_text)?,
            expected_hash,
        )
        .await
        .map_err(crate::AdminError::password_hash)?
        .get()
    {
        return Err(crate::AdminError::Validation);
    }
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(
            crate::admin_new_password_from_contract(new_password)
                .map_err(crate::AdminError::password_text)?,
        )
        .await
        .map_err(crate::AdminError::password_hash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::AdminError::from)?;
    crate::repository::update_user_password::update_user_password(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        actor.id,
        &password_hash,
        crate::AdminPasswordChangeRequired::from(false),
    )
    .await
    .map_err(crate::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(crate::AdminError::Conflict)?;
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_OTHER_ACCESS_SESSIONS_SQL)
        .bind(actor.id.get())
        .bind(actor.session_id.get().get())
        .execute(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(crate::AdminError::from)
        .map(drop)?;
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_USER_REFRESH_TOKENS_SQL)
        .bind(actor.id.get())
        .execute(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(crate::AdminError::from)
        .map(drop)?;
    crate::persistence::record_audit_success_in_connection(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        crate::persistence::AdminAuditSuccessRef {
            action: crate::AdminAuditAction::Update,
            login: &actor.login,
            resource: crate::AdminAuditResource::User,
            resource_id: crate::persistence::AdminAuditResourceId::User(actor.id),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(crate::AdminError::from)?;
    Ok(crate::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
