#![allow(clippy::single_call_fn)] // route inventory registers this role operation once

pub(in crate::domain_types::auth) async fn mutations_update(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminRoleId>,
    request: super::super::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateRoleReq>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom::authorize_custom(
        &auth,
        super::super::super::AdminPermission::RolesUpdate,
    )
    .await?;
    let name = super::super::super::AdminRoleName::try_from(request.0.into_name().into_inner())
        .map_err(|_error| super::super::AdminError::Validation)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::super::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_ROLE_SQL)
        .bind(path.0.get())
        .bind(name.as_ref())
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| super::super::super::StdAdminBool::from(value.is_some()))
        .map_err(|error| {
            super::super::shared::map_unique_violation::map_unique_violation(error.into_inner())
        })?
        .get()
        .then_some(())
        .ok_or(super::super::AdminError::Conflict)?;
    super::super::persistence::record_audit_success_in_connection(
        super::super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::super::persistence::AdminAuditSuccessRef {
            action: super::super::super::AdminAuditAction::Update,
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
