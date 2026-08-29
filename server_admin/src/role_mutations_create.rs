pub(crate) async fn role_mutations_create(
    auth: crate::admin_auth_req::AdminAuthReq,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_create_role_req::AdminCreateRoleReq,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &auth,
        server_admin_contract::admin_permission::AdminPermission::RolesCreate,
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
    let role_id = sqlx::query_scalar::<_, i64>(
        constants_str::integration_fixtures::SERVER_ADMIN_INSERT_ROLE_SQL,
    )
    .bind(name.as_ref())
    .fetch_one(&mut *tx)
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .and_then(|value| {
        server_admin_core::admin_role_id::AdminRoleId::try_from(value)
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    })
    .map_err(|error| crate::map_unique_violation::map_unique_violation(error.into_inner()))?;
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef {
            action: crate::admin_audit_action::AdminAuditAction::Create,
            login: &actor.login,
            resource: crate::admin_audit_resource::AdminAuditResource::Role,
            resource_id: crate::admin_audit_resource_id::AdminAuditResourceId::Role(role_id),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    Ok(crate::axum_admin_response::AxumAdminResponse(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(
                server_admin_contract::admin_create_role_res::AdminCreateRoleRes::new(
                    server_admin_contract::admin_role_id::AdminRoleId::from(role_id.value()),
                ),
            ),
        )),
    ))
}
