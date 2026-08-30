pub(crate) async fn user_mutations_update(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_user_record_id::AdminUserRecordId,
    >,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_user_req::AdminUpdateUserReq,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &auth,
        server_admin_contract::admin_permission::AdminPermission::UsersUpdate,
    )
    .await?;
    let (contract_display_name, contract_login) = request.0.into_parts();
    let display_name = contract_display_name
        .map(|value| {
            server_admin_contract::admin_display_name::AdminDisplayName::try_from(
                value.into_inner(),
            )
        })
        .transpose()
        .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    let login = contract_login
        .map(|value| server_admin_contract::admin_login::AdminLogin::try_from(value.into_inner()))
        .transpose()
        .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    if login.is_none() && display_name.is_none() {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(
        constants_str::integration_fixtures::SERVER_ADMIN_UPDATE_USER_SQL,
    )
    .bind(path.0.get())
    .bind(login.as_ref().map(|value| value.as_ref().as_str()))
    .bind(display_name.as_ref().map(|value| value.as_ref().as_str()))
    .fetch_optional(&mut *tx)
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map(|value| server_admin_core::std_admin_bool::StdAdminBool::from(value.is_some()))
    .map_err(|error| crate::map_unique_violation::map_unique_violation(error.into_inner()))?
    .get()
    .then_some(())
    .ok_or(crate::admin_error::AdminError::Conflict)?;
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef {
            action: crate::admin_audit_action::AdminAuditAction::Update,
            login: &actor.login,
            resource: crate::admin_audit_resource::AdminAuditResource::User,
            resource_id: crate::admin_audit_resource_id::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    Ok(crate::axum_admin_response::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
