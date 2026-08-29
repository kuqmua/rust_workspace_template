pub(crate) async fn role_mutations_delete(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<server_admin_core::admin_role_id::AdminRoleId>,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &auth,
        server_admin_contract::admin_permission::AdminPermission::RolesDelete,
    )
    .await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(
        constants_str::integration_fixtures::SERVER_ADMIN_DELETE_ROLE_SQL,
    )
    .bind(path.0.get())
    .fetch_optional(&mut *tx)
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map(|value| server_admin_core::std_admin_bool::StdAdminBool::from(value.is_some()))
    .map_err(crate::admin_error::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(crate::admin_error::AdminError::Conflict)?;
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef {
            action: crate::admin_audit_action::AdminAuditAction::Delete,
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
