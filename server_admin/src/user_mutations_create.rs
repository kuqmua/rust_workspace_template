pub(crate) async fn user_mutations_create(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_create_user_request::AdminCreateUserRequest,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &admin_auth_request,
        server_admin_contract::admin_permission::AdminPermission::UsersCreate,
    )
    .await?;
    let (contract_display_name, contract_login, contract_password) =
        axum_admin_json.into_inner().into_parts();
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
    let password_hash = admin_auth_request
        .get_state()
        .as_ref()
        .get_password_hasher()
        .hash(password)
        .await
        .map_err(crate::admin_error::AdminError::password_hash)?;
    let mut tx = admin_auth_request
        .get_state()
        .as_ref()
        .get_pool()
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
    crate::finalize_audited_transaction::finalize_audited_transaction(
        crate::sqlx_admin_transaction::SqlxAdminTransaction::from(tx),
        crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
            crate::admin_audit_action::AdminAuditAction::Create,
            actor.get_login(),
            crate::admin_audit_resource::AdminAuditResource::User,
            crate::admin_audit_resource_id::AdminAuditResourceId::User(
                server_admin_core::admin_user_record_id::AdminUserRecordId::from(user_id.value()),
            ),
            *actor.get_id(),
        ),
    )
    .await?;
    Ok(crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(
                server_admin_contract::admin_create_user_response::AdminCreateUserResponse::new(
                    server_admin_contract::admin_user_id::AdminUserId::from(user_id.value()),
                ),
            ),
        )),
    ))
}
