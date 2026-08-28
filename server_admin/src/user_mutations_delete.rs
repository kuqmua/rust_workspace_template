pub(crate) async fn user_mutations_delete(
    auth: crate::AdminAuthReq,
    path: crate::AxumAdminPath<crate::AdminUserId>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let actor = crate::shared::authorize_custom::authorize_custom(
        &auth,
        crate::AdminPermission::UsersDelete,
    )
    .await?;
    if actor.id == path.0 {
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
    let last_admin_state = crate::repository::roles::read_last_admin_state(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(crate::AdminError::from)?;
    if last_admin_state.would_remove_last().get() {
        return Err(crate::AdminError::Conflict);
    }
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_DELETE_USER_SQL)
        .bind(path.0.get())
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
        .map_err(crate::AdminError::from)?
        .get()
        .then_some(())
        .ok_or(crate::AdminError::Conflict)?;
    crate::persistence::record_audit_success_in_connection(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        crate::persistence::AdminAuditSuccessRef {
            action: crate::AdminAuditAction::Delete,
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
