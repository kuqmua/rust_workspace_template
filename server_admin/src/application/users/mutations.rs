#![allow(clippy::single_call_fn)] // route inventory and HTML composition each register focused user operations once

pub(in crate::domain_types::auth) async fn create(
    auth: super::super::AdminAuthReq,
    request: super::super::AxumAdminJson<server_admin_contract::domain_types::AdminCreateUserReq>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom(
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
    let user_id = crate::adapters::repository::users::insert_user(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &login,
        &display_name,
        &password_hash,
    )
    .await
    .map_err(|error| super::super::shared::map_unique_violation(error.0))?;
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
pub(in crate::domain_types::auth) async fn update(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminUserId>,
    request: super::super::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateUserReq>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom(
        &auth,
        super::super::super::AdminPermission::UsersUpdate,
    )
    .await?;
    let (contract_display_name, contract_login) = request.0.into_parts();
    let display_name = contract_display_name
        .map(|value| super::super::super::AdminDisplayName::try_from(value.into_inner()))
        .transpose()
        .map_err(|_error| super::super::AdminError::Validation)?;
    let login = contract_login
        .map(|value| super::super::super::AdminLogin::try_from(value.into_inner()))
        .transpose()
        .map_err(|_error| super::super::AdminError::Validation)?;
    if login.is_none() && display_name.is_none() {
        return Err(super::super::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::super::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_SQL)
        .bind(path.0.get())
        .bind(login.as_ref().map(|value| value.as_ref().as_str()))
        .bind(display_name.as_ref().map(|value| value.as_ref().as_str()))
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| super::super::super::StdAdminBool::from(value.is_some()))
        .map_err(|error| super::super::shared::map_unique_violation(error.0))?
        .get()
        .then_some(())
        .ok_or(super::super::AdminError::Conflict)?;
    super::super::persistence::record_audit_success_in_connection(
        super::super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::super::persistence::AdminAuditSuccessRef {
            action: super::super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::super::AdminAuditResource::User,
            resource_id: super::super::persistence::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::super::AdminError::from)?;
    Ok(super::super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(in crate::domain_types::auth) async fn set_password(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminUserId>,
    request: super::super::AxumAdminJson<
        server_admin_contract::domain_types::AdminSetUserPasswordReq,
    >,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom(
        &auth,
        super::super::super::AdminPermission::UsersUpdate,
    )
    .await?;
    let password = super::super::admin_new_password_from_contract(request.0.into_password())
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
    crate::adapters::repository::users::update_user_password(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        &password_hash,
        super::super::super::AdminPasswordChangeRequired::from(true),
    )
    .await
    .map_err(super::super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::super::AdminError::Conflict)?;
    crate::adapters::repository::sessions::revoke_user_sessions(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(super::super::AdminError::from)?;
    super::super::persistence::record_audit_success_in_connection(
        super::super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::super::persistence::AdminAuditSuccessRef {
            action: super::super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::super::AdminAuditResource::User,
            resource_id: super::super::persistence::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::super::AdminError::from)?;
    Ok(super::super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(in crate::domain_types::auth) async fn set_ban(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminUserId>,
    request: super::super::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserBanReq>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom(
        &auth,
        super::super::super::AdminPermission::UsersUpdate,
    )
    .await?;
    let is_banned = bool::from(request.0.is_banned());
    if is_banned && actor.id == path.0 {
        return Err(super::super::AdminError::Conflict);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::super::AdminError::from)?;
    crate::adapters::repository::roles::lock_last_admin(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
    )
    .await
    .map_err(super::super::AdminError::from)?;
    if is_banned {
        let last_admin_state = crate::adapters::repository::roles::read_last_admin_state(
            crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            path.0,
        )
        .await
        .map_err(super::super::AdminError::from)?;
        if last_admin_state.would_remove_last().get() {
            return Err(super::super::AdminError::Conflict);
        }
    }
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_BAN_SQL)
        .bind(path.0.get())
        .bind(is_banned)
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| super::super::super::StdAdminBool::from(value.is_some()))
        .map_err(super::super::AdminError::from)?
        .get()
        .then_some(())
        .ok_or(super::super::AdminError::Conflict)?;
    if is_banned {
        crate::adapters::repository::sessions::revoke_user_sessions(
            crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            path.0,
        )
        .await
        .map_err(super::super::AdminError::from)?;
    }
    super::super::persistence::record_audit_success_in_connection(
        super::super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::super::persistence::AdminAuditSuccessRef {
            action: super::super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::super::AdminAuditResource::User,
            resource_id: super::super::persistence::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::super::AdminError::from)?;
    Ok(super::super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(in crate::domain_types::auth) async fn delete(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminUserId>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom(
        &auth,
        super::super::super::AdminPermission::UsersDelete,
    )
    .await?;
    if actor.id == path.0 {
        return Err(super::super::AdminError::Conflict);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::super::AdminError::from)?;
    crate::adapters::repository::roles::lock_last_admin(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
    )
    .await
    .map_err(super::super::AdminError::from)?;
    let last_admin_state = crate::adapters::repository::roles::read_last_admin_state(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(super::super::AdminError::from)?;
    if last_admin_state.would_remove_last().get() {
        return Err(super::super::AdminError::Conflict);
    }
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_DELETE_USER_SQL)
        .bind(path.0.get())
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| super::super::super::StdAdminBool::from(value.is_some()))
        .map_err(super::super::AdminError::from)?
        .get()
        .then_some(())
        .ok_or(super::super::AdminError::Conflict)?;
    super::super::persistence::record_audit_success_in_connection(
        super::super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::super::persistence::AdminAuditSuccessRef {
            action: super::super::super::AdminAuditAction::Delete,
            login: &actor.login,
            resource: super::super::super::AdminAuditResource::User,
            resource_id: super::super::persistence::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::super::AdminError::from)?;
    Ok(super::super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(in crate::domain_types::auth) async fn set_roles(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminUserId>,
    request: super::super::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserRolesReq>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom(
        &auth,
        super::super::super::AdminPermission::UserRolesUpdate,
    )
    .await?;
    let (expected_role_ids, contract_role_ids) = request.0.into_parts();
    if AsRef::<[server_admin_contract::domain_types::AdminRoleId]>::as_ref(&expected_role_ids)
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len()
        != AsRef::<[server_admin_contract::domain_types::AdminRoleId]>::as_ref(&expected_role_ids)
            .len()
        || AsRef::<[server_admin_contract::domain_types::AdminRoleId]>::as_ref(&contract_role_ids)
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != AsRef::<[server_admin_contract::domain_types::AdminRoleId]>::as_ref(
                &contract_role_ids,
            )
            .len()
    {
        return Err(super::super::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::super::AdminError::from)?;
    let outcome = async {
        crate::adapters::repository::roles::lock_last_admin(
            crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        )
        .await?;
        let optional_target_is_active =
            sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_LOCK_USER_ACTIVE_STATE_SQL)
                .bind(path.0.get())
                .fetch_optional(&mut *tx)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let Some(target_is_active) = optional_target_is_active else {
            return Ok::<_, crate::domain_types::SqlxAdminError>(
                crate::adapters::repository::ReplaceUserRolesOutcome::MissingUser,
            );
        };
        let current_role_ids =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_READ_USER_ROLE_IDS_SQL)
                .bind(path.0.get())
                .fetch_all(&mut *tx)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let mut expected_raw_ids =
            AsRef::<[server_admin_contract::domain_types::AdminRoleId]>::as_ref(&expected_role_ids)
                .iter()
                .copied()
                .map(i64::from)
                .collect::<Vec<_>>();
        expected_raw_ids.sort_unstable();
        if current_role_ids != expected_raw_ids {
            return Ok(crate::adapters::repository::ReplaceUserRolesOutcome::StaleAssignment);
        }
        let raw_ids =
            AsRef::<[server_admin_contract::domain_types::AdminRoleId]>::as_ref(&contract_role_ids)
                .iter()
                .copied()
                .map(i64::from)
                .collect::<Vec<_>>();
        let existing_count =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_ROLES_SQL)
                .bind(&raw_ids)
                .fetch_one(&mut *tx)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        if usize::try_from(existing_count).ok() != Some(raw_ids.len()) {
            return Ok(crate::adapters::repository::ReplaceUserRolesOutcome::UnknownRole);
        }
        let admin_role_id =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
                .fetch_one(&mut *tx)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let target_was_admin =
            sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_USER_HAS_ROLE_SQL)
                .bind(path.0.get())
                .bind(admin_role_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        if target_is_active && target_was_admin && !raw_ids.contains(&admin_role_id) {
            let active_admin_count = sqlx::query_scalar::<_, i64>(
                constants_str::SERVER_ADMIN_ACTIVE_ROLE_USER_COUNT_SQL,
            )
            .bind(admin_role_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)?;
            if active_admin_count <= constants_i64::ONE {
                return Ok(
                    crate::adapters::repository::ReplaceUserRolesOutcome::LastActiveAdministrator,
                );
            }
        }
        let _delete_result = sqlx::query(constants_str::SERVER_ADMIN_REPLACE_USER_ROLES_DELETE_SQL)
            .bind(path.0.get())
            .execute(&mut *tx)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)?;
        let _insert_result = sqlx::query(constants_str::SERVER_ADMIN_REPLACE_USER_ROLES_INSERT_SQL)
            .bind(path.0.get())
            .bind(&raw_ids)
            .execute(&mut *tx)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)?;
        crate::adapters::repository::sessions::revoke_user_sessions(
            crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            path.0,
        )
        .await?;
        Ok(crate::adapters::repository::ReplaceUserRolesOutcome::Updated)
    }
    .await
    .map_err(super::super::AdminError::from)?;
    match outcome {
        crate::adapters::repository::ReplaceUserRolesOutcome::Updated => {}
        crate::adapters::repository::ReplaceUserRolesOutcome::UnknownRole => {
            return Err(super::super::AdminError::Validation);
        }
        crate::adapters::repository::ReplaceUserRolesOutcome::LastActiveAdministrator
        | crate::adapters::repository::ReplaceUserRolesOutcome::MissingUser
        | crate::adapters::repository::ReplaceUserRolesOutcome::StaleAssignment => {
            return Err(super::super::AdminError::Conflict);
        }
    }
    super::super::persistence::record_audit_success_in_connection(
        super::super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::super::persistence::AdminAuditSuccessRef {
            action: super::super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::super::AdminAuditResource::User,
            resource_id: super::super::persistence::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::super::AdminError::from)?;
    Ok(super::super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
