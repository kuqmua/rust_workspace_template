pub(crate) async fn role_mutations_create(
    auth: crate::AdminAuthReq,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminCreateRoleReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let actor = crate::shared::authorize_custom::authorize_custom(
        &auth,
        crate::AdminPermission::RolesCreate,
    )
    .await?;
    let name = crate::AdminRoleName::try_from(request.0.into_name().into_inner())
        .map_err(|_error| crate::AdminError::Validation)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::AdminError::from)?;
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
            crate::shared::map_unique_violation::map_unique_violation(error.into_inner())
        })?;
    crate::persistence::record_audit_success_in_connection(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        crate::persistence::AdminAuditSuccessRef {
            action: crate::AdminAuditAction::Create,
            login: &actor.login,
            resource: crate::AdminAuditResource::Role,
            resource_id: crate::persistence::AdminAuditResourceId::Role(role_id),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(crate::AdminError::from)?;
    Ok(crate::AxumAdminResponse(
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
