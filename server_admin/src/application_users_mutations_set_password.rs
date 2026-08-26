#![allow(clippy::single_call_fn)] // route inventory registers this user operation once

pub(in crate::domain_types::auth) async fn set_password(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminUserId>,
    request: super::super::AxumAdminJson<
        server_admin_contract::domain_types::AdminSetUserPasswordReq,
    >,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom::authorize_custom(
        &auth,
        super::super::super::AdminPermission::UsersUpdate,
    )
    .await?;
    let password = super::super::admin_new_password_from_contract(request.0.into_password())
        .map_err(super::super::AdminError::password_text)?;
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(password)
        .await
        .map_err(super::super::AdminError::password_hash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::super::AdminError::from)?;
    crate::adapters::repository::update_user_password::update_user_password(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        &password_hash,
        super::super::super::AdminPasswordChangeRequired::from(true),
    )
    .await
    .map_err(super::super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::super::AdminError::Conflict)?;
    crate::adapters::repository::revoke_user_sessions::revoke_user_sessions(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(super::super::AdminError::from)?;
    super::super::persistence::record_audit_success_in_connection(
        super::super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::super::persistence::AdminAuditSuccessRef {
            action: super::super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::super::AdminAuditResource::User,
            resource_id: super::super::persistence::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::super::AdminError::from)?;
    Ok(super::super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
