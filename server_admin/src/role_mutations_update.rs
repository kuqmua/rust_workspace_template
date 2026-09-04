pub(crate) async fn role_mutations_update(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_role_record_id::AdminRoleRecordId,
    >,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_role_request::AdminUpdateRoleRequest,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &admin_auth_request,
        server_admin_contract::admin_permission::AdminPermission::RolesUpdate,
    )
    .await?;
    let name = server_admin_contract::admin_role_name::AdminRoleName::try_from(
        axum_admin_json.into_inner().into_name().into_inner(),
    )
    .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    let mut tx = admin_auth_request
        .get_state()
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_ROLE_SQL)
        .bind(axum_admin_path.get_inner().get())
        .bind(name.as_ref())
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(|value| server_admin_core::std_admin_bool::StdAdminBool::from(value.is_some()))
        .map_err(|error| crate::map_unique_violation::map_unique_violation(error.into_inner()))?
        .get()
        .then_some(())
        .ok_or(crate::admin_error::AdminError::Conflict)?;
    crate::finalize_audited_transaction::finalize_audited_transaction(
        crate::sqlx_admin_transaction::SqlxAdminTransaction::from(tx),
        crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
            crate::admin_audit_action::AdminAuditAction::Update,
            actor.get_login(),
            crate::admin_audit_resource::AdminAuditResource::Role,
            crate::admin_audit_resource_id::AdminAuditResourceId::Role(
                *axum_admin_path.get_inner(),
            ),
            *actor.get_id(),
        ),
    )
    .await?;
    Ok(crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
