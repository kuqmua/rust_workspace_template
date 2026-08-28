pub(crate) async fn mutations_set_ban(
    auth: crate::AdminAuthReq,
    path: crate::AxumAdminPath<crate::AdminUserId>,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserBanReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let actor = crate::shared::authorize_custom::authorize_custom(
        &auth,
        crate::AdminPermission::UsersUpdate,
    )
    .await?;
    let is_banned = bool::from(request.0.is_banned());
    if is_banned && actor.id == path.0 {
        return Err(crate::AdminError::Conflict);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::AdminError::from)?;
    crate::repository::roles::lock_last_admin(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
    )
    .await
    .map_err(crate::AdminError::from)?;
    if is_banned {
        let last_admin_state = crate::repository::roles::read_last_admin_state(
            crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            path.0,
        )
        .await
        .map_err(crate::AdminError::from)?;
        if last_admin_state.would_remove_last().get() {
            return Err(crate::AdminError::Conflict);
        }
    }
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_BAN_SQL)
        .bind(path.0.get())
        .bind(is_banned)
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
        .map_err(crate::AdminError::from)?
        .get()
        .then_some(())
        .ok_or(crate::AdminError::Conflict)?;
    if is_banned {
        crate::repository::revoke_user_sessions::revoke_user_sessions(
            crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            path.0,
        )
        .await
        .map_err(crate::AdminError::from)?;
    }
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
