#![allow(clippy::single_call_fn)] // route inventory registers this user operation once

pub(in crate::domain_types::auth) async fn user_mutations_delete(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminUserId>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom::authorize_custom(
        &auth,
        super::super::super::AdminPermission::UsersDelete,
    )
    .await?;
    if actor.id == path.0 {
        return Err(super::super::AdminError::Conflict);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::super::AdminError::from)?;
    crate::repository::roles::lock_last_admin(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
    )
    .await
    .map_err(super::super::AdminError::from)?;
    let last_admin_state = crate::repository::roles::read_last_admin_state(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(super::super::AdminError::from)?;
    if last_admin_state.would_remove_last().get() {
        return Err(super::super::AdminError::Conflict);
    }
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_DELETE_USER_SQL)
        .bind(path.0.get())
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| super::super::super::StdAdminBool::from(value.is_some()))
        .map_err(super::super::AdminError::from)?
        .get()
        .then_some(())
        .ok_or(super::super::AdminError::Conflict)?;
    super::super::persistence::record_audit_success_in_connection(
        super::super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::super::persistence::AdminAuditSuccessRef {
            action: super::super::super::AdminAuditAction::Delete,
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
