#![allow(clippy::single_call_fn)] // route inventory and HTML composition each register focused user operations once

pub(in crate::domain_types::auth) async fn user_mutations_create(
    auth: super::super::AdminAuthReq,
    request: super::super::AxumAdminJson<server_admin_contract::domain_types::AdminCreateUserReq>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom::authorize_custom(
        &auth,
        super::super::super::AdminPermission::UsersCreate,
    )
    .await?;
    let (contract_display_name, contract_login, contract_password) = request.0.into_parts();
    let display_name =
        super::super::super::AdminDisplayName::try_from(contract_display_name.into_inner())
            .map_err(|_error| super::super::AdminError::Validation)?;
    let login = super::super::super::AdminLogin::try_from(contract_login.into_inner())
        .map_err(|_error| super::super::AdminError::Validation)?;
    let password = super::super::admin_new_password_from_contract(contract_password)
        .map_err(super::super::AdminError::password_text)?;
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(password)
        .await
        .map_err(super::super::AdminError::password_hash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::super::AdminError::from)?;
    let user_id = crate::repository::insert_user::insert_user(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &login,
        &display_name,
        &password_hash,
    )
    .await
    .map_err(|error| {
        super::super::shared::map_unique_violation::map_unique_violation(error.into_inner())
    })?;
    super::super::persistence::record_audit_success_in_connection(
        super::super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::super::persistence::AdminAuditSuccessRef {
            action: super::super::super::AdminAuditAction::Create,
            login: &actor.login,
            resource: super::super::super::AdminAuditResource::User,
            resource_id: super::super::persistence::AdminAuditResourceId::User(
                super::super::super::AdminUserId::from(user_id.value()),
            ),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::super::AdminError::from)?;
    Ok(super::super::AxumAdminResponse(
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
