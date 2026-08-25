#![allow(clippy::single_call_fn)] // route inventory registers focused role operations once

pub(in crate::domain_types::auth) async fn create(
    auth: super::super::AdminAuthReq,
    request: super::super::AxumAdminJson<server_admin_contract::domain_types::AdminCreateRoleReq>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom(
        &auth,
        super::super::super::AdminPermission::RolesCreate,
    )
    .await?;
    let name = super::super::super::AdminRoleName::try_from(request.0.into_name().into_inner())
        .map_err(|_error| super::super::AdminError::Validation)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::super::AdminError::from)?;
    let role_id = sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_INSERT_ROLE_SQL)
        .bind(name.as_ref())
        .fetch_one(&mut *tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .and_then(|value| {
            crate::domain_types::AdminRoleId::try_from(value)
                .map_err(crate::domain_types::SqlxAdminError::from)
        })
        .map_err(|error| super::super::shared::map_unique_violation(error.0))?;
    super::super::persistence::record_audit_success_in_connection(
        super::super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::super::persistence::AdminAuditSuccessRef {
            action: super::super::super::AdminAuditAction::Create,
            login: &actor.login,
            resource: super::super::super::AdminAuditResource::Role,
            resource_id: super::super::persistence::AdminAuditResourceId::Role(role_id),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::super::AdminError::from)?;
    Ok(super::super::AxumAdminResponse(
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
pub(in crate::domain_types::auth) async fn update(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminRoleId>,
    request: super::super::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateRoleReq>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom(
        &auth,
        super::super::super::AdminPermission::RolesUpdate,
    )
    .await?;
    let name = super::super::super::AdminRoleName::try_from(request.0.into_name().into_inner())
        .map_err(|_error| super::super::AdminError::Validation)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::super::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_ROLE_SQL)
        .bind(path.0.get())
        .bind(name.as_ref())
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
            resource: super::super::super::AdminAuditResource::Role,
            resource_id: super::super::persistence::AdminAuditResourceId::Role(path.0),
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
    path: super::super::AxumAdminPath<super::super::super::AdminRoleId>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom(
        &auth,
        super::super::super::AdminPermission::RolesDelete,
    )
    .await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::super::AdminError::from)?;
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_DELETE_ROLE_SQL)
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
            resource: super::super::super::AdminAuditResource::Role,
            resource_id: super::super::persistence::AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::super::AdminError::from)?;
    Ok(super::super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(in crate::domain_types::auth) async fn set_permissions(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminRoleId>,
    request: super::super::AxumAdminJson<
        server_admin_contract::domain_types::AdminSetRolePermissionsReq,
    >,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom(
        &auth,
        super::super::super::AdminPermission::RolePermissionsUpdate,
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
    .map_err(super::super::AdminError::from)?;
    match outcome {
        crate::adapters::repository::ReplaceRolePermissionsOutcome::Updated => {}
        crate::adapters::repository::ReplaceRolePermissionsOutcome::UnknownPermission => {
            return Err(super::super::AdminError::Validation);
        }
        crate::adapters::repository::ReplaceRolePermissionsOutcome::MissingRole
        | crate::adapters::repository::ReplaceRolePermissionsOutcome::StaleAssignment
        | crate::adapters::repository::ReplaceRolePermissionsOutcome::SystemRole => {
            return Err(super::super::AdminError::Conflict);
        }
    }
    super::super::persistence::record_audit_success_in_connection(
        super::super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::super::persistence::AdminAuditSuccessRef {
            action: super::super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::super::AdminAuditResource::Role,
            resource_id: super::super::persistence::AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::super::AdminError::from)?;
    Ok(super::super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
