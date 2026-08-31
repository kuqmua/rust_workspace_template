pub(crate) async fn mutations_set_ban(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_user_record_id::AdminUserRecordId,
    >,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_set_user_ban_req::AdminSetUserBanReq,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &auth,
        server_admin_contract::admin_permission::AdminPermission::UsersUpdate,
    )
    .await?;
    let is_banned = bool::from(request.get_inner().is_banned());
    if is_banned && *actor.get_id() == *path.get_inner() {
        return Err(crate::admin_error::AdminError::Conflict);
    }
    let mut tx = auth
        .get_state()
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    crate::lock_last_admin::lock_last_admin(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
    )
    .await
    .map_err(crate::admin_error::AdminError::from)?;
    if is_banned {
        let last_admin_state = crate::read_last_admin_state::read_last_admin_state(
            crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            *path.get_inner(),
        )
        .await
        .map_err(crate::admin_error::AdminError::from)?;
        if last_admin_state.would_remove_last().get() {
            return Err(crate::admin_error::AdminError::Conflict);
        }
    }
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_BAN_SQL)
        .bind(path.get_inner().get())
        .bind(is_banned)
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(|value| server_admin_core::std_admin_bool::StdAdminBool::from(value.is_some()))
        .map_err(crate::admin_error::AdminError::from)?
        .get()
        .then_some(())
        .ok_or(crate::admin_error::AdminError::Conflict)?;
    if is_banned {
        crate::revoke_user_sessions::revoke_user_sessions(
            crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            *path.get_inner(),
        )
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    }
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
