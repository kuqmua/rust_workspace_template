pub(crate) async fn mutations_set_password(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_user_record_id::AdminUserRecordId,
    >,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_set_user_password_req::AdminSetUserPasswordReq,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &auth,
        server_admin_contract::admin_permission::AdminPermission::UsersUpdate,
    )
    .await?;
    let password = crate::admin_new_password_from_contract::admin_new_password_from_contract(
        request.into_inner().into_password(),
    )
    .map_err(crate::admin_error::AdminError::password_text)?;
    let password_hash = auth
        .get_state()
        .as_ref()
        .get_password_hasher()
        .hash(password)
        .await
        .map_err(crate::admin_error::AdminError::password_hash)?;
    let mut tx = auth
        .get_state()
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    crate::update_user_password::update_user_password(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        *path.get_inner(),
        &password_hash,
        crate::admin_password_change_required::AdminPasswordChangeRequired::from(true),
    )
    .await
    .map_err(crate::admin_error::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(crate::admin_error::AdminError::Conflict)?;
    crate::revoke_user_sessions::revoke_user_sessions(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        *path.get_inner(),
    )
    .await
    .map_err(crate::admin_error::AdminError::from)?;
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
            crate::admin_audit_action::AdminAuditAction::Update,
            actor.get_login(),
            crate::admin_audit_resource::AdminAuditResource::User,
            crate::admin_audit_resource_id::AdminAuditResourceId::User(*path.get_inner()),
            *actor.get_id(),
        ),
    )
    .await?;
    tx.commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    Ok(crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
