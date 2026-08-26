#![allow(clippy::single_call_fn)] // route inventory registers this role operation once

pub(in crate::domain_types::auth) async fn set_permissions(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminRoleId>,
    request: super::super::AxumAdminJson<
        server_admin_contract::domain_types::AdminSetRolePermissionsReq,
    >,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom::authorize_custom(
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
