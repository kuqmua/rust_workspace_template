pub(crate) async fn mutations_set_password(
    auth: crate::AdminAuthReq,
    path: crate::AxumAdminPath<crate::AdminUserId>,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserPasswordReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let actor = crate::shared::authorize_custom::authorize_custom(
        &auth,
        crate::AdminPermission::UsersUpdate,
    )
    .await?;
    let password = crate::admin_new_password_from_contract(request.0.into_password())
        .map_err(crate::AdminError::password_text)?;
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(password)
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
        path.0,
        &password_hash,
        crate::AdminPasswordChangeRequired::from(true),
    )
    .await
    .map_err(crate::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(crate::AdminError::Conflict)?;
    crate::repository::revoke_user_sessions::revoke_user_sessions(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(crate::AdminError::from)?;
    crate::persistence::record_audit_success_in_connection(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        crate::persistence::AdminAuditSuccessRef {
            action: crate::AdminAuditAction::Update,
            login: &actor.login,
            resource: crate::AdminAuditResource::User,
            resource_id: crate::persistence::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(crate::AdminError::from)?;
    Ok(crate::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
