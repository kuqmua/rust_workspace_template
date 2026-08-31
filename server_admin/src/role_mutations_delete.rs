pub(crate) async fn role_mutations_delete(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_role_record_id::AdminRoleRecordId,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &auth,
        server_admin_contract::admin_permission::AdminPermission::RolesDelete,
    )
    .await?;
    let mut tx = auth
        .get_state()
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_DELETE_ROLE_SQL)
        .bind(path.get_inner().get())
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
        crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
            crate::admin_audit_action::AdminAuditAction::Delete,
            actor.get_login(),
            crate::admin_audit_resource::AdminAuditResource::Role,
            crate::admin_audit_resource_id::AdminAuditResourceId::Role(*path.get_inner()),
            *actor.get_id(),
        ),
    )
    .await?;
    tx.commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    Ok(crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
