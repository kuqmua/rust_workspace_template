#![allow(clippy::single_call_fn)] // route inventory registers this role operation once

pub(in crate::domain_types::auth) async fn role_mutations_delete(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminRoleId>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom::authorize_custom(
        &auth,
        super::super::super::AdminPermission::RolesDelete,
    )
    .await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::super::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_DELETE_ROLE_SQL)
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
            resource: super::super::super::AdminAuditResource::Role,
            resource_id: super::super::persistence::AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::super::AdminError::from)?;
    Ok(super::super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
