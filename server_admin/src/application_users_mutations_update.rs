#![allow(clippy::single_call_fn)] // route inventory registers this user operation once

pub(in crate::domain_types::auth) async fn mutations_update(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminUserId>,
    request: super::super::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateUserReq>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom::authorize_custom(
        &auth,
        super::super::super::AdminPermission::UsersUpdate,
    )
    .await?;
    let (contract_display_name, contract_login) = request.0.into_parts();
    let display_name = contract_display_name
        .map(|value| super::super::super::AdminDisplayName::try_from(value.into_inner()))
        .transpose()
        .map_err(|_error| super::super::AdminError::Validation)?;
    let login = contract_login
        .map(|value| super::super::super::AdminLogin::try_from(value.into_inner()))
        .transpose()
        .map_err(|_error| super::super::AdminError::Validation)?;
    if login.is_none() && display_name.is_none() {
        return Err(super::super::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::super::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_SQL)
        .bind(path.0.get())
        .bind(login.as_ref().map(|value| value.as_ref().as_str()))
        .bind(display_name.as_ref().map(|value| value.as_ref().as_str()))
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
