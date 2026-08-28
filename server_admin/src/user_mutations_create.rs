pub(crate) async fn user_mutations_create(
    auth: crate::AdminAuthReq,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminCreateUserReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let actor = crate::shared::authorize_custom::authorize_custom(
        &auth,
        crate::AdminPermission::UsersCreate,
    )
    .await?;
    let (contract_display_name, contract_login, contract_password) = request.0.into_parts();
    let display_name = crate::AdminDisplayName::try_from(contract_display_name.into_inner())
        .map_err(|_error| crate::AdminError::Validation)?;
    let login = crate::AdminLogin::try_from(contract_login.into_inner())
        .map_err(|_error| crate::AdminError::Validation)?;
    let password = crate::admin_new_password_from_contract(contract_password)
        .map_err(crate::AdminError::password_text)?;
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(password)
        .await
        .map_err(crate::AdminError::password_hash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::AdminError::from)?;
    let user_id = crate::repository::insert_user::insert_user(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &login,
        &display_name,
        &password_hash,
    )
    .await
    .map_err(|error| {
        crate::shared::map_unique_violation::map_unique_violation(error.into_inner())
    })?;
    crate::persistence::record_audit_success_in_connection(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        crate::persistence::AdminAuditSuccessRef {
            action: crate::AdminAuditAction::Create,
            login: &actor.login,
            resource: crate::AdminAuditResource::User,
            resource_id: crate::persistence::AdminAuditResourceId::User(crate::AdminUserId::from(
                user_id.value(),
            )),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(crate::AdminError::from)?;
    Ok(crate::AxumAdminResponse(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(
                server_admin_contract::domain_types::AdminCreateUserRes::new(
                    server_admin_contract::domain_types::AdminUserId::from(user_id.value()),
                ),
            ),
        )),
    ))
}
