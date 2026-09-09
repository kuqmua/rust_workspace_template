pub(crate) async fn apply_user_update_in_connection(
    mut sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    runtime_authenticated_admin: &crate::runtime_authenticated_admin::RuntimeAuthenticatedAdmin,
    admin_user_record_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
    admin_update_user_request: &server_admin_contract::admin_update_user_request::AdminUpdateUserRequest,
) -> Result<(), crate::admin_error::AdminError> {
    let is_banned = admin_update_user_request
        .is_banned()
        .copied()
        .map(bool::from);
    if admin_update_user_request.login().is_none()
        && admin_update_user_request.display_name().is_none()
        && is_banned.is_none()
    {
        return Err(crate::admin_error::AdminError::Validation);
    }
    if is_banned == Some(true) && runtime_authenticated_admin.id() == admin_user_record_id {
        return Err(crate::admin_error::AdminError::Conflict);
    }
    if is_banned.is_some() {
        crate::lock_last_admin::lock_last_admin(
            crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
                &mut **sqlx_admin_repository_connection_mut_ref,
            ),
        ).await?;
        if is_banned == Some(true) {
            let state = crate::read_last_admin_state::read_last_admin_state(
                crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
                    &mut **sqlx_admin_repository_connection_mut_ref,
                ),
                admin_user_record_id,
            ).await?;
            if state.would_remove_last().get() {
                return Err(crate::admin_error::AdminError::Conflict);
            }
        }
    }
    let _stored_is_banned =
        sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_SQL)
            .bind(admin_user_record_id.get())
            .bind(
                admin_update_user_request
                    .login()
                    .as_ref()
                    .map(|value| value.as_ref().as_str()),
            )
            .bind(
                admin_update_user_request
                    .display_name()
                    .as_ref()
                    .map(|value| value.as_ref().as_str()),
            )
            .bind(is_banned)
            .fetch_optional(&mut **sqlx_admin_repository_connection_mut_ref)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
            .map_err(|error| crate::map_unique_violation::map_unique_violation(error.into_inner()))?
            .ok_or(crate::admin_error::AdminError::Conflict)?;
    if is_banned == Some(true) {
        crate::revoke_user_sessions::revoke_user_sessions(
            crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
                &mut **sqlx_admin_repository_connection_mut_ref,
            ),
            admin_user_record_id,
        ).await?;
    }
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        sqlx_admin_repository_connection_mut_ref,
        crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
            crate::admin_audit_action::AdminAuditAction::Update,
            runtime_authenticated_admin.get_login(),
            crate::admin_audit_resource::AdminAuditResource::User,
            crate::admin_audit_resource_id::AdminAuditResourceId::User(admin_user_record_id),
            runtime_authenticated_admin.id(),
        ),
    )
    .await
}
