#![allow(clippy::single_call_fn)] // route inventory and HTML composition each register focused user operations once

pub(super) async fn create(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::AdminCreateUserReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::shared::authorize_custom(&auth, super::super::AdminPermission::UsersCreate).await?;
    let (contract_display_name, contract_login, contract_password) = request.0.into_parts();
    let display_name = super::super::AdminDisplayName::try_from(contract_display_name.into_inner())
        .map_err(|_error| super::AdminError::Validation)?;
    let login = super::super::AdminLogin::try_from(contract_login.into_inner())
        .map_err(|_error| super::AdminError::Validation)?;
    let password = super::admin_new_password_from_contract(contract_password)
        .map_err(super::AdminError::password_text)?;
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(password)
        .await
        .map_err(super::AdminError::password_hash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    let user_id = super::super::repository::users::insert_user(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &login,
        &display_name,
        &password_hash,
    )
    .await
    .map_err(|error| super::shared::map_unique_violation(error.0))?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Create,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(super::super::AdminUserId::from(
                user_id.value(),
            )),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(server_admin_contract::AdminCreateUserRes::new(
                server_admin_contract::AdminUserId::from(user_id.value()),
            )),
        )),
    ))
}
pub(super) async fn update(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::AdminUpdateUserReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::shared::authorize_custom(&auth, super::super::AdminPermission::UsersUpdate).await?;
    let (contract_display_name, contract_login) = request.0.into_parts();
    let display_name = contract_display_name
        .map(|value| super::super::AdminDisplayName::try_from(value.into_inner()))
        .transpose()
        .map_err(|_error| super::AdminError::Validation)?;
    let login = contract_login
        .map(|value| super::super::AdminLogin::try_from(value.into_inner()))
        .transpose()
        .map_err(|_error| super::AdminError::Validation)?;
    if login.is_none() && display_name.is_none() {
        return Err(super::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::users::update_user(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        login.as_ref(),
        display_name.as_ref(),
    )
    .await
    .map_err(|error| super::shared::map_unique_violation(error.0))?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn set_password(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::AdminSetUserPasswordReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::shared::authorize_custom(&auth, super::super::AdminPermission::UsersUpdate).await?;
    let password = super::admin_new_password_from_contract(request.0.into_password())
        .map_err(super::AdminError::password_text)?;
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(password)
        .await
        .map_err(super::AdminError::password_hash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::users::update_user_password(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        &password_hash,
        super::super::AdminPasswordChangeRequired::from(true),
    )
    .await
    .map_err(super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    super::super::repository::sessions::revoke_user_sessions(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(super::AdminError::from)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn set_ban(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::AdminSetUserBanReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::shared::authorize_custom(&auth, super::super::AdminPermission::UsersUpdate).await?;
    let is_banned = bool::from(request.0.is_banned());
    if is_banned && actor.id == path.0 {
        return Err(super::AdminError::Conflict);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::roles::lock_last_admin(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
    )
    .await
    .map_err(super::AdminError::from)?;
    if is_banned {
        let last_admin_state = super::super::repository::roles::read_last_admin_state(
            super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            path.0,
        )
        .await
        .map_err(super::AdminError::from)?;
        if last_admin_state.would_remove_last().get() {
            return Err(super::AdminError::Conflict);
        }
    }
    super::super::repository::users::update_user_ban(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        super::super::StdAdminBool::from(is_banned),
    )
    .await
    .map_err(super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    if is_banned {
        super::super::repository::sessions::revoke_user_sessions(
            super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            path.0,
        )
        .await
        .map_err(super::AdminError::from)?;
    }
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn delete(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::shared::authorize_custom(&auth, super::super::AdminPermission::UsersDelete).await?;
    if actor.id == path.0 {
        return Err(super::AdminError::Conflict);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    let last_admin_state = super::super::repository::roles::lock_and_read_last_admin_state(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(super::AdminError::from)?;
    if last_admin_state.would_remove_last().get() {
        return Err(super::AdminError::Conflict);
    }
    super::super::repository::users::delete_user(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Delete,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn set_roles(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::AdminSetUserRolesReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::shared::authorize_custom(&auth, super::super::AdminPermission::UserRolesUpdate)
            .await?;
    let (expected_role_ids, contract_role_ids) = request.0.into_parts();
    if AsRef::<[server_admin_contract::AdminRoleId]>::as_ref(&expected_role_ids)
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len()
        != AsRef::<[server_admin_contract::AdminRoleId]>::as_ref(&expected_role_ids).len()
        || AsRef::<[server_admin_contract::AdminRoleId]>::as_ref(&contract_role_ids)
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != AsRef::<[server_admin_contract::AdminRoleId]>::as_ref(&contract_role_ids).len()
    {
        return Err(super::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    let outcome = super::super::repository::roles::replace_user_roles(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        expected_role_ids.as_ref(),
        contract_role_ids.as_ref(),
    )
    .await
    .map_err(super::AdminError::from)?;
    match outcome {
        super::super::repository::ReplaceUserRolesOutcome::Updated => {}
        super::super::repository::ReplaceUserRolesOutcome::UnknownRole => {
            return Err(super::AdminError::Validation);
        }
        super::super::repository::ReplaceUserRolesOutcome::LastActiveAdministrator
        | super::super::repository::ReplaceUserRolesOutcome::MissingUser
        | super::super::repository::ReplaceUserRolesOutcome::StaleAssignment => {
            return Err(super::AdminError::Conflict);
        }
    }
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::User,
            resource_id: super::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn users_page(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<server_admin_contract::AdminUsersPage, super::AdminError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::UsersRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    super::shared::validate_table_sort(
        &query.0,
        &server_admin_contract::AdminTableSortField::USER,
    )?;
    let pool = super::super::repository::SqlxAdminRepositoryPoolRef::from(
        auth.state.as_ref().pool.as_ref(),
    );
    let (users, total) = super::super::repository::users::list_users(pool, &query.0)
        .await
        .map_err(super::shared::map_repository_error)?;
    let roles = super::super::repository::roles::list_role_catalog(pool)
        .await
        .map_err(super::shared::map_repository_error)?;
    Ok(server_admin_contract::AdminUsersPage::new(
        users,
        roles,
        super::shared::page_total(total)?,
    ))
}
pub(super) async fn list(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    users_page(auth, query)
        .await
        .map(super::shared::json_response)
}
