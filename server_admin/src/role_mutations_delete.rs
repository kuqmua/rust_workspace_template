pub(crate) async fn role_mutations_delete(
    auth: crate::AdminAuthReq,
    path: crate::AxumAdminPath<crate::AdminRoleId>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let actor = crate::shared::authorize_custom::authorize_custom(
        &auth,
        crate::AdminPermission::RolesDelete,
    )
    .await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_DELETE_ROLE_SQL)
        .bind(path.0.get())
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
        .map_err(crate::AdminError::from)?
        .get()
        .then_some(())
        .ok_or(crate::AdminError::Conflict)?;
    crate::persistence::record_audit_success_in_connection(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        crate::persistence::AdminAuditSuccessRef {
            action: crate::AdminAuditAction::Delete,
            login: &actor.login,
            resource: crate::AdminAuditResource::Role,
            resource_id: crate::persistence::AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(crate::AdminError::from)?;
    Ok(crate::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
