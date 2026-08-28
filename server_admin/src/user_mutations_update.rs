pub(crate) async fn user_mutations_update(
    auth: crate::AdminAuthReq,
    path: crate::AxumAdminPath<crate::AdminUserId>,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateUserReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let actor = crate::shared::authorize_custom::authorize_custom(
        &auth,
        crate::AdminPermission::UsersUpdate,
    )
    .await?;
    let (contract_display_name, contract_login) = request.0.into_parts();
    let display_name = contract_display_name
        .map(|value| crate::AdminDisplayName::try_from(value.into_inner()))
        .transpose()
        .map_err(|_error| crate::AdminError::Validation)?;
    let login = contract_login
        .map(|value| crate::AdminLogin::try_from(value.into_inner()))
        .transpose()
        .map_err(|_error| crate::AdminError::Validation)?;
    if login.is_none() && display_name.is_none() {
        return Err(crate::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_SQL)
        .bind(path.0.get())
        .bind(login.as_ref().map(|value| value.as_ref().as_str()))
        .bind(display_name.as_ref().map(|value| value.as_ref().as_str()))
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
        .map_err(|error| {
            crate::shared::map_unique_violation::map_unique_violation(error.into_inner())
        })?
        .get()
        .then_some(())
        .ok_or(crate::AdminError::Conflict)?;
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
