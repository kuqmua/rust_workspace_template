#![allow(clippy::single_call_fn)] // route inventory and HTML composition each register focused account operations once

pub(super) async fn me_context_view_ref(
    auth: &super::AdminAuthReq,
) -> Result<
    (
        server_admin_contract::domain_types::AuthenticatedAdmin,
        super::super::AdminPasswordChangeRequired,
    ),
    super::AdminError,
> {
    super::authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await
    .and_then(|authenticated| {
        let password_change_required = authenticated.password_change_required();
        super::authenticated_admin_contract(&authenticated)
            .map(|contract| (contract, password_change_required))
    })
}
pub(super) async fn me(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    me_context_view_ref(&auth)
        .await
        .map(|context| super::shared::json_response(context.0))
}
pub(super) async fn change_own_password(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminChangeOwnPasswordReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
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
    let (current_password, new_password) = request.0.into_parts();
    let expected_hash =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_PASSWORD_HASH_SQL)
            .bind(actor.id.get())
            .fetch_optional(auth.state.as_ref().pool.as_ref())
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .map(|value| {
                value.map(|hash| {
                    super::super::AdminPasswordHash::new(
                        pg_types_text_misc::StringAsNonNullTextSecret::from(hash),
                    )
                })
            })
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
    crate::adapters::repository::users::update_user_password(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        actor.id,
        &password_hash,
        super::super::AdminPasswordChangeRequired::from(false),
    )
    .await
    .map_err(super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_OTHER_ACCESS_SESSIONS_SQL)
        .bind(actor.id.get())
        .bind(actor.session_id.get().get())
        .execute(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(super::AdminError::from)
        .map(drop)?;
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_USER_REFRESH_TOKENS_SQL)
        .bind(actor.id.get())
        .execute(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(super::AdminError::from)
        .map(drop)?;
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
