#![allow(clippy::single_call_fn)] // route inventory registers focused role operations once

pub(in crate::domain_types::auth) async fn create(
    auth: super::super::AdminAuthReq,
    request: super::super::AxumAdminJson<server_admin_contract::domain_types::AdminCreateRoleReq>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom::authorize_custom(
        &auth,
        super::super::super::AdminPermission::RolesCreate,
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
    let role_id = sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_INSERT_ROLE_SQL)
        .bind(name.as_ref())
        .fetch_one(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .and_then(|value| {
            crate::domain_types::AdminRoleId::try_from(value)
                .map_err(crate::domain_types::SqlxAdminError::from)
        })
        .map_err(|error| {
            super::super::shared::map_unique_violation::map_unique_violation(error.into_inner())
        })?;
    super::super::persistence::record_audit_success_in_connection(
        super::super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::super::persistence::AdminAuditSuccessRef {
            action: super::super::super::AdminAuditAction::Create,
            login: &actor.login,
            resource: super::super::super::AdminAuditResource::Role,
            resource_id: super::super::persistence::AdminAuditResourceId::Role(role_id),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::super::AdminError::from)?;
    Ok(super::super::AxumAdminResponse(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(
                server_admin_contract::domain_types::AdminCreateRoleRes::new(
                    server_admin_contract::domain_types::AdminRoleId::from(role_id.value()),
                ),
            ),
        )),
    ))
}
