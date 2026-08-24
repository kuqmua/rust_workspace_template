#![allow(clippy::single_call_fn)] // route inventory and HTML composition each register focused account operations once

pub(super) async fn me_view(
    auth: super::AdminAuthReq,
) -> Result<server_admin_contract::AuthenticatedAdmin, super::AdminError> {
    me_context_view(auth).await.map(|context| context.0)
}
pub(super) async fn me_context_view(
    auth: super::AdminAuthReq,
) -> Result<
    (
        server_admin_contract::AuthenticatedAdmin,
        super::super::AdminPasswordChangeRequired,
    ),
    super::AdminError,
> {
    me_context_view_ref(&auth).await
}
pub(super) async fn me_context_view_ref(
    auth: &super::AdminAuthReq,
) -> Result<
    (
        server_admin_contract::AuthenticatedAdmin,
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
    me_view(auth).await.map(super::shared::json_response)
}
pub(super) async fn change_own_password(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::AdminChangeOwnPasswordReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::shared::authenticate_mutation(&auth).await?;
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
        super::super::AdminPasswordChangeRequired::from(false),
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
