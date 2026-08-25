#![allow(clippy::single_call_fn)] // route inventory registers focused role operations once

pub(super) async fn create(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminCreateRoleReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::shared::authorize_custom(&auth, super::super::AdminPermission::RolesCreate).await?;
    let name = super::super::AdminRoleName::try_from(request.0.into_name().into_inner())
        .map_err(|_error| super::AdminError::Validation)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    let role_id = sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_INSERT_ROLE_SQL)
        .bind(name.as_ref())
        .fetch_one(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .and_then(|value| {
            crate::domain_types::AdminRoleId::try_from(value)
                .map_err(crate::domain_types::SqlxAdminError::from)
        })
        .map_err(|error| super::shared::map_unique_violation(error.0))?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Create,
            login: &actor.login,
            resource: super::super::AdminAuditResource::Role,
            resource_id: super::AdminAuditResourceId::Role(role_id),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(
                server_admin_contract::domain_types::AdminCreateRoleRes::new(
                    server_admin_contract::domain_types::AdminRoleId::from(role_id.value()),
                ),
            ),
        )),
    ))
}
pub(super) async fn update(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateRoleReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::shared::authorize_custom(&auth, super::super::AdminPermission::RolesUpdate).await?;
    let name = super::super::AdminRoleName::try_from(request.0.into_name().into_inner())
        .map_err(|_error| super::AdminError::Validation)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_ROLE_SQL)
        .bind(path.0.get())
        .bind(name.as_ref())
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
            resource: super::super::AdminAuditResource::Role,
            resource_id: super::AdminAuditResourceId::Role(path.0),
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
    path: super::AxumAdminPath<super::super::AdminRoleId>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::shared::authorize_custom(&auth, super::super::AdminPermission::RolesDelete).await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_DELETE_ROLE_SQL)
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
            resource: super::super::AdminAuditResource::Role,
            resource_id: super::AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn set_permissions(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminSetRolePermissionsReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::shared::authorize_custom(
        &auth,
        super::super::AdminPermission::RolePermissionsUpdate,
    )
    .await?;
    let (expected_permission_ids, contract_permission_ids) = request.0.into_parts();
    if AsRef::<[server_admin_contract::domain_types::AdminPermissionId]>::as_ref(
        &expected_permission_ids,
    )
    .iter()
    .collect::<std::collections::HashSet<_>>()
    .len()
        != AsRef::<[server_admin_contract::domain_types::AdminPermissionId]>::as_ref(
            &expected_permission_ids,
        )
        .len()
        || AsRef::<[server_admin_contract::domain_types::AdminPermissionId]>::as_ref(
            &contract_permission_ids,
        )
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len()
            != AsRef::<[server_admin_contract::domain_types::AdminPermissionId]>::as_ref(
                &contract_permission_ids,
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
        let inlined_role_permission_role_id = path.0;
        let inlined_expected_permission_ids = expected_permission_ids.as_ref();
        let inlined_permission_ids = contract_permission_ids.as_ref();
        let optional_is_system =
            sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_LOCK_ROLE_SYSTEM_STATE_SQL)
                .bind(inlined_role_permission_role_id.get())
                .fetch_optional(&mut *tx)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let Some(is_system) = optional_is_system else {
            return Ok::<_, crate::domain_types::SqlxAdminError>(
                crate::adapters::repository::ReplaceRolePermissionsOutcome::MissingRole,
            );
        };
        if is_system {
            return Ok::<_, crate::domain_types::SqlxAdminError>(
                crate::adapters::repository::ReplaceRolePermissionsOutcome::SystemRole,
            );
        }
        let current_permission_ids =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_READ_ROLE_PERMISSION_IDS_SQL)
                .bind(inlined_role_permission_role_id.get())
                .fetch_all(&mut *tx)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let mut expected_raw_ids = inlined_expected_permission_ids
            .iter()
            .copied()
            .map(i64::from)
            .collect::<Vec<_>>();
        expected_raw_ids.sort_unstable();
        if current_permission_ids != expected_raw_ids {
            return Ok::<_, crate::domain_types::SqlxAdminError>(
                crate::adapters::repository::ReplaceRolePermissionsOutcome::StaleAssignment,
            );
        }
        let raw_ids = inlined_permission_ids
            .iter()
            .copied()
            .map(i64::from)
            .collect::<Vec<_>>();
        let existing_count =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_PERMISSIONS_SQL)
                .bind(&raw_ids)
                .fetch_one(&mut *tx)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        if usize::try_from(existing_count).ok() != Some(raw_ids.len()) {
            return Ok::<_, crate::domain_types::SqlxAdminError>(
                crate::adapters::repository::ReplaceRolePermissionsOutcome::UnknownPermission,
            );
        }
        let _delete_result =
            sqlx::query(constants_str::SERVER_ADMIN_REPLACE_ROLE_PERMISSIONS_DELETE_SQL)
                .bind(inlined_role_permission_role_id.get())
                .execute(&mut *tx)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let _insert_result =
            sqlx::query(constants_str::SERVER_ADMIN_REPLACE_ROLE_PERMISSIONS_INSERT_SQL)
                .bind(inlined_role_permission_role_id.get())
                .bind(&raw_ids)
                .execute(&mut *tx)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        Ok::<_, crate::domain_types::SqlxAdminError>(
            crate::adapters::repository::ReplaceRolePermissionsOutcome::Updated,
        )
    }
    .await
    .map_err(super::AdminError::from)?;
    match outcome {
        crate::adapters::repository::ReplaceRolePermissionsOutcome::Updated => {}
        crate::adapters::repository::ReplaceRolePermissionsOutcome::UnknownPermission => {
            return Err(super::AdminError::Validation);
        }
        crate::adapters::repository::ReplaceRolePermissionsOutcome::MissingRole
        | crate::adapters::repository::ReplaceRolePermissionsOutcome::StaleAssignment
        | crate::adapters::repository::ReplaceRolePermissionsOutcome::SystemRole => {
            return Err(super::AdminError::Conflict);
        }
    }
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::Role,
            resource_id: super::AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn roles_page(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<server_admin_contract::domain_types::AdminRolesPage, super::AdminError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::RolesRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    super::shared::validate_table_sort(
        &query.0,
        &server_admin_contract::domain_types::AdminTableSortField::ROLE,
    )?;
    let role_pool = auth.state.as_ref().pool.as_ref();
    let (roles, total) = async {
        let search = query.0.search().as_ref();
        let total =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_FILTERED_ROLES_SQL)
                .bind(search)
                .fetch_one(role_pool)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let rows =
            sqlx::query_as::<_, (i64, String, bool)>(constants_str::SERVER_ADMIN_PAGE_ROLES_SQL)
                .bind(search)
                .bind(query.0.sort().as_ref())
                .bind(query.0.direction().as_ref())
                .bind(i64::from(u16::from(query.0.limit())))
                .bind(i64::from(u32::from(query.0.offset())))
                .fetch_all(role_pool)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
        let role_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
        let links = sqlx::query_as::<_, (i64, i64)>(
            constants_str::SERVER_ADMIN_LIST_ROLE_PERMISSION_IDS_SQL,
        )
        .bind(role_ids.as_slice())
        .fetch_all(role_pool)
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
        let items = rows
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
        Ok::<_, crate::adapters::repository::AdminRepositoryError>((
            server_admin_contract::domain_types::AdminRoleSummaries::try_from(items).map_err(
                |_error| crate::adapters::repository::AdminRepositoryError::InvalidStoredValue,
            )?,
            crate::adapters::repository::AdminPageTotalCount::from(total),
        ))
    }
    .await
    .map_err(super::shared::map_repository_error)?;
    let permissions = async {
        let permission_pool = auth.state.as_ref().pool.as_ref();
        let values =
            sqlx::query_as::<_, (i64, String)>(constants_str::SERVER_ADMIN_LIST_PERMISSIONS_SQL)
                .fetch_all(permission_pool)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?
                .into_iter()
                .map(|(id, name)| {
                    Ok(
                server_admin_contract::domain_types::AdminPermissionSummary::new(
                    server_admin_contract::domain_types::AdminPermissionId::try_from(id).map_err(
                        |_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        },
                    )?,
                    server_admin_contract::domain_types::AdminPermissionValue::try_from(name)
                        .map_err(|_error| {
                            crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                        })?,
                ),
            )
                })
                .collect::<Result<Vec<_>, crate::adapters::repository::AdminRepositoryError>>()?;
        server_admin_contract::domain_types::AdminPermissionSummaries::try_from(values)
            .map_err(|_error| crate::adapters::repository::AdminRepositoryError::InvalidStoredValue)
    }
    .await
    .map_err(super::shared::map_repository_error)?;
    Ok(server_admin_contract::domain_types::AdminRolesPage::new(
        roles,
        permissions,
        super::shared::page_total(total)?,
    ))
}
pub(super) async fn list(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    roles_page(auth, query)
        .await
        .map(super::shared::json_response)
}
pub(super) async fn list_permissions(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::PermissionsRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    super::shared::validate_table_sort(
        &query.0,
        &server_admin_contract::domain_types::AdminTableSortField::PERMISSION,
    )?;
    let permission_pool = auth.state.as_ref().pool.as_ref();
    let search = query.0.search().as_ref();
    let total =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_FILTERED_PERMISSIONS_SQL)
            .bind(search)
            .fetch_one(permission_pool)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .map_err(super::AdminError::from)?;
    let items =
        sqlx::query_as::<_, (i64, String)>(constants_str::SERVER_ADMIN_PAGE_PERMISSIONS_SQL)
            .bind(search)
            .bind(query.0.sort().as_ref())
            .bind(query.0.direction().as_ref())
            .bind(i64::from(u16::from(query.0.limit())))
            .bind(i64::from(u32::from(query.0.offset())))
            .fetch_all(permission_pool)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)
            .map_err(super::AdminError::from)?
            .into_iter()
            .map(|(id, name)| {
                Ok(
                    server_admin_contract::domain_types::AdminPermissionSummary::new(
                        server_admin_contract::domain_types::AdminPermissionId::try_from(id)
                            .map_err(|_error| super::AdminError::Validation)?,
                        server_admin_contract::domain_types::AdminPermissionValue::try_from(name)
                            .map_err(|_error| super::AdminError::Validation)?,
                    ),
                )
            })
            .collect::<Result<Vec<_>, super::AdminError>>()?;
    let permissions =
        server_admin_contract::domain_types::AdminPermissionSummaries::try_from(items)
            .map_err(|_error| super::AdminError::Validation)?;
    Ok(super::shared::json_response(
        server_admin_contract::domain_types::AdminPermissionsPage::new(
            permissions,
            super::shared::page_total(crate::adapters::repository::AdminPageTotalCount::from(
                total,
            ))?,
        ),
    ))
}
