pub(crate) async fn user_mutations_update(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_user_record_id::AdminUserRecordId,
    >,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_user_request::AdminUpdateUserRequest,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &admin_auth_request,
        server_admin_contract::admin_permission::AdminPermission::UsersUpdate,
    )
    .await?;
    let (contract_display_name, contract_login, contract_is_banned) =
        axum_admin_json.into_inner().into_parts();
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
    let is_banned = contract_is_banned.map(bool::from);
    if login.is_none() && display_name.is_none() && is_banned.is_none() {
        return Err(crate::admin_error::AdminError::Validation);
    }
    if is_banned == Some(true) && *actor.get_id() == *axum_admin_path.get_inner() {
        return Err(crate::admin_error::AdminError::Conflict);
    }
    let mut tx = admin_auth_request
        .get_state()
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    if is_banned.is_some() {
        crate::lock_last_admin::lock_last_admin(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
    )
    .await
    .map_err(crate::admin_error::AdminError::from)?;
        if is_banned == Some(true) {
            let last_admin_state = crate::read_last_admin_state::read_last_admin_state(
            crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            *axum_admin_path.get_inner(),
        )
        .await
        .map_err(crate::admin_error::AdminError::from)?;
            if last_admin_state.would_remove_last().get() {
                return Err(crate::admin_error::AdminError::Conflict);
            }
        }
    }
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_SQL)
        .bind(axum_admin_path.get_inner().get())
        .bind(login.as_ref().map(|value| value.as_ref().as_str()))
        .bind(display_name.as_ref().map(|value| value.as_ref().as_str()))
        .bind(is_banned)
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(|value| server_admin_core::std_admin_bool::StdAdminBool::from(value.is_some()))
        .map_err(|error| crate::map_unique_violation::map_unique_violation(error.into_inner()))?
        .get()
        .then_some(())
        .ok_or(crate::admin_error::AdminError::Conflict)?;
    if is_banned == Some(true) {
        crate::revoke_user_sessions::revoke_user_sessions(
            crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            *axum_admin_path.get_inner(),
        )
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    }
    crate::finalize_audited_transaction::finalize_audited_transaction(
        crate::sqlx_admin_transaction::SqlxAdminTransaction::from(tx),
        crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
            crate::admin_audit_action::AdminAuditAction::Update,
            actor.get_login(),
            crate::admin_audit_resource::AdminAuditResource::User,
            crate::admin_audit_resource_id::AdminAuditResourceId::User(
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
