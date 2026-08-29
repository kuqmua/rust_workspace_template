pub(crate) async fn role_mutations_update(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<server_admin_core::admin_role_id::AdminRoleId>,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_role_req::AdminUpdateRoleReq,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &auth,
        server_admin_contract::admin_permission::AdminPermission::RolesUpdate,
    )
    .await?;
    let name = server_admin_contract::admin_role_name::AdminRoleName::try_from(
        request.0.into_name().into_inner(),
    )
    .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(
        constants_str::integration_fixtures::SERVER_ADMIN_UPDATE_ROLE_SQL,
    )
    .bind(path.0.get())
    .bind(name.as_ref())
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
            resource: crate::admin_audit_resource::AdminAuditResource::Role,
            resource_id: crate::admin_audit_resource_id::AdminAuditResourceId::Role(path.0),
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
