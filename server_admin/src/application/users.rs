#![allow(clippy::single_call_fn)] // route inventory and HTML composition each register focused user operations once

pub(super) async fn create(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminCreateUserReq>,
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
    let user_id = crate::adapters::repository::users::insert_user(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
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
            axum::Json(
                server_admin_contract::domain_types::AdminCreateUserRes::new(
                    server_admin_contract::domain_types::AdminUserId::from(user_id.value()),
                ),
            ),
        )),
    ))
}
pub(super) async fn update(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateUserReq>,
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
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_SQL)
        .bind(path.0.get())
        .bind(login.as_ref().map(|value| value.as_ref().as_str()))
        .bind(display_name.as_ref().map(|value| value.as_ref().as_str()))
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| super::super::StdAdminBool::from(value.is_some()))
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
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserPasswordReq>,
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
    crate::adapters::repository::users::update_user_password(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        &password_hash,
        super::super::AdminPasswordChangeRequired::from(true),
    )
    .await
    .map_err(super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    crate::adapters::repository::sessions::revoke_user_sessions(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
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
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserBanReq>,
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
    crate::adapters::repository::roles::lock_last_admin(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
    )
    .await
    .map_err(super::AdminError::from)?;
    if is_banned {
        let last_admin_state = crate::adapters::repository::roles::read_last_admin_state(
            crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            path.0,
        )
        .await
        .map_err(super::AdminError::from)?;
        if last_admin_state.would_remove_last().get() {
            return Err(super::AdminError::Conflict);
        }
    }
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_BAN_SQL)
        .bind(path.0.get())
        .bind(is_banned)
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| super::super::StdAdminBool::from(value.is_some()))
        .map_err(super::AdminError::from)?
        .get()
        .then_some(())
        .ok_or(super::AdminError::Conflict)?;
    if is_banned {
        crate::adapters::repository::sessions::revoke_user_sessions(
            crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
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
    crate::adapters::repository::roles::lock_last_admin(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
    )
    .await
    .map_err(super::AdminError::from)?;
    let last_admin_state = crate::adapters::repository::roles::read_last_admin_state(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(super::AdminError::from)?;
    if last_admin_state.would_remove_last().get() {
        return Err(super::AdminError::Conflict);
    }
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_DELETE_USER_SQL)
        .bind(path.0.get())
        .fetch_optional(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| super::super::StdAdminBool::from(value.is_some()))
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
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserRolesReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::shared::authorize_custom(&auth, super::super::AdminPermission::UserRolesUpdate)
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
    .map_err(super::AdminError::from)?;
    match outcome {
        crate::adapters::repository::ReplaceUserRolesOutcome::Updated => {}
        crate::adapters::repository::ReplaceUserRolesOutcome::UnknownRole => {
            return Err(super::AdminError::Validation);
        }
        crate::adapters::repository::ReplaceUserRolesOutcome::LastActiveAdministrator
        | crate::adapters::repository::ReplaceUserRolesOutcome::MissingUser
        | crate::adapters::repository::ReplaceUserRolesOutcome::StaleAssignment => {
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
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<server_admin_contract::domain_types::AdminUsersPage, super::AdminError> {
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
        &server_admin_contract::domain_types::AdminTableSortField::USER,
    )?;
    let user_pool = auth.state.as_ref().pool.as_ref();
    let (users, total) = async {
        let search = query.0.search().as_ref();
        let total =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_FILTERED_USERS_SQL)
                .bind(search)
                .fetch_one(user_pool)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let rows = sqlx::query_as::<_, (i64, String, String, bool)>(
            constants_str::SERVER_ADMIN_PAGE_USERS_SQL,
        )
        .bind(search)
        .bind(query.0.sort().as_ref())
        .bind(query.0.direction().as_ref())
        .bind(i64::from(u16::from(query.0.limit())))
        .bind(i64::from(u32::from(query.0.offset())))
        .fetch_all(user_pool)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?;
        let user_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
        let links =
            sqlx::query_as::<_, (i64, i64)>(constants_str::SERVER_ADMIN_LIST_USER_ROLE_IDS_SQL)
                .bind(user_ids.as_slice())
                .fetch_all(user_pool)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let mut role_ids_by_user =
            links.into_iter().try_fold(
                std::collections::HashMap::<
                    i64,
                    Vec<server_admin_contract::domain_types::AdminRoleId>,
                >::with_capacity(user_ids.len()),
                |mut values, (user_id, role_id)| {
                    values.entry(user_id).or_default().push(
                    server_admin_contract::domain_types::AdminRoleId::try_from(role_id).map_err(
                        |_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                );
                    Ok::<_, crate::adapters::repository::AdminRepositoryError>(values)
                },
            )?;
        let items = rows
            .into_iter()
            .map(|(id, login, display_name, is_banned)| {
                Ok(server_admin_contract::domain_types::AdminUserSummary::new(
                    server_admin_contract::domain_types::AdminDisplayName::try_from(display_name)
                        .map_err(|_error| {
                        crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                    })?,
                    server_admin_contract::domain_types::AdminUserId::try_from(id).map_err(
                        |_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::domain_types::AdminBool::from(is_banned),
                    server_admin_contract::domain_types::AdminLogin::try_from(login).map_err(
                        |_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::domain_types::AdminRoleIds::try_from(
                        role_ids_by_user.remove(&id).unwrap_or_default(),
                    )
                    .map_err(|_error| {
                        crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                    })?,
                ))
            })
            .collect::<Result<Vec<_>, crate::adapters::repository::AdminRepositoryError>>()?;
        Ok::<_, crate::adapters::repository::AdminRepositoryError>((
            server_admin_contract::domain_types::AdminUserSummaries::try_from(items).map_err(
                |_error| crate::adapters::repository::AdminRepositoryError::InvalidStoredValue,
            )?,
            crate::adapters::repository::AdminPageTotalCount::from(total),
        ))
    }
    .await
    .map_err(super::shared::map_repository_error)?;
    let roles = async {
        let role_catalog_pool = auth.state.as_ref().pool.as_ref();
        let rows =
            sqlx::query_as::<_, (i64, String, bool)>(constants_str::SERVER_ADMIN_LIST_ROLES_SQL)
                .fetch_all(role_catalog_pool)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let role_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
        let links = sqlx::query_as::<_, (i64, i64)>(
            constants_str::SERVER_ADMIN_LIST_ROLE_PERMISSION_IDS_SQL,
        )
        .bind(role_ids.as_slice())
        .fetch_all(role_catalog_pool)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?;
        let mut permission_ids_by_role = links.into_iter().try_fold(
            std::collections::HashMap::<
                i64,
                Vec<server_admin_contract::domain_types::AdminPermissionId>,
            >::with_capacity(role_ids.len()),
            |mut values, (role_id, permission_id)| {
                values.entry(role_id).or_default().push(
                    server_admin_contract::domain_types::AdminPermissionId::try_from(permission_id)
                        .map_err(|_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        })?,
                );
                Ok::<_, crate::adapters::repository::AdminRepositoryError>(values)
            },
        )?;
        let values = rows
            .into_iter()
            .map(|(id, name, is_system)| {
                Ok(server_admin_contract::domain_types::AdminRoleSummary::new(
                    server_admin_contract::domain_types::AdminRoleId::try_from(id).map_err(
                        |_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::domain_types::AdminBool::from(is_system),
                    server_admin_contract::domain_types::AdminRoleName::try_from(name).map_err(
                        |_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::domain_types::AdminPermissionIds::try_from(
                        permission_ids_by_role.remove(&id).unwrap_or_default(),
                    )
                    .map_err(|_error| {
                        crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                    })?,
                ))
            })
            .collect::<Result<Vec<_>, crate::adapters::repository::AdminRepositoryError>>()?;
        server_admin_contract::domain_types::AdminRoleSummaries::try_from(values)
            .map_err(|_error| crate::adapters::repository::AdminRepositoryError::InvalidStoredValue)
    }
    .await
    .map_err(super::shared::map_repository_error)?;
    Ok(server_admin_contract::domain_types::AdminUsersPage::new(
        users,
        roles,
        super::shared::page_total(total)?,
    ))
}
pub(super) async fn list(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    users_page(auth, query)
        .await
        .map(super::shared::json_response)
}
