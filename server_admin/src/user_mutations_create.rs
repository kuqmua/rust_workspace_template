pub(crate) async fn user_mutations_create(
    auth: crate::admin_auth_req::AdminAuthReq,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_create_user_req::AdminCreateUserReq,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &auth,
        server_admin_contract::admin_permission::AdminPermission::UsersCreate,
    )
    .await?;
    let (contract_display_name, contract_login, contract_password) = request.0.into_parts();
    let display_name = server_admin_contract::admin_display_name::AdminDisplayName::try_from(
        contract_display_name.into_inner(),
    )
    .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    let login =
        server_admin_contract::admin_login::AdminLogin::try_from(contract_login.into_inner())
            .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    let password = crate::admin_new_password_from_contract::admin_new_password_from_contract(
        contract_password,
    )
    .map_err(crate::admin_error::AdminError::password_text)?;
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(password)
        .await
        .map_err(crate::admin_error::AdminError::password_hash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    let user_id = crate::insert_user::insert_user(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        &login,
        &display_name,
        &password_hash,
    )
    .await
    .map_err(|error| crate::map_unique_violation::map_unique_violation(error.into_inner()))?;
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef {
            action: crate::admin_audit_action::AdminAuditAction::Create,
            login: &actor.login,
            resource: crate::admin_audit_resource::AdminAuditResource::User,
            resource_id: crate::admin_audit_resource_id::AdminAuditResourceId::User(
                server_admin_core::admin_user_record_id::AdminUserRecordId::from(user_id.value()),
            ),
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
                server_admin_contract::admin_create_user_res::AdminCreateUserRes::new(
                    server_admin_contract::admin_user_id::AdminUserId::from(user_id.value()),
                ),
            ),
        )),
    ))
}
