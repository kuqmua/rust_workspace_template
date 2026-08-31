pub(crate) async fn mutations_set_permissions(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_role_record_id::AdminRoleRecordId,
    >,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_set_role_permissions_req::AdminSetRolePermissionsReq,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &auth,
        server_admin_contract::admin_permission::AdminPermission::RolePermissionsUpdate,
    )
    .await?;
    let (expected_permission_ids, contract_permission_ids) = request.into_inner().into_parts();
    if AsRef::<[server_admin_contract::admin_permission_id::AdminPermissionId]>::as_ref(
        &expected_permission_ids,
    )
    .iter()
    .collect::<std::collections::HashSet<_>>()
    .len()
        != AsRef::<[server_admin_contract::admin_permission_id::AdminPermissionId]>::as_ref(
            &expected_permission_ids,
        )
        .len()
        || AsRef::<[server_admin_contract::admin_permission_id::AdminPermissionId]>::as_ref(
            &contract_permission_ids,
        )
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len()
            != AsRef::<[server_admin_contract::admin_permission_id::AdminPermissionId]>::as_ref(
                &contract_permission_ids,
            )
            .len()
    {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let mut tx = auth
        .get_state()
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    let outcome = async {
        let inlined_role_permission_role_id = path.get_inner();
        let inlined_expected_permission_ids = expected_permission_ids.as_ref();
        let inlined_permission_ids = contract_permission_ids.as_ref();
        let optional_is_system =
            sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_LOCK_ROLE_SYSTEM_STATE_SQL)
                .bind(inlined_role_permission_role_id.get())
                .fetch_optional(&mut *tx)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let Some(is_system) = optional_is_system else {
            return Ok::<_, crate::sqlx_admin_error::SqlxAdminError>(
                crate::replace_role_permissions_outcome::ReplaceRolePermissionsOutcome::MissingRole,
            );
        };
        if is_system {
            return Ok::<_, crate::sqlx_admin_error::SqlxAdminError>(
                crate::replace_role_permissions_outcome::ReplaceRolePermissionsOutcome::SystemRole,
            );
        }
        let current_permission_ids =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_READ_ROLE_PERMISSION_IDS_SQL)
                .bind(inlined_role_permission_role_id.get())
                .fetch_all(&mut *tx)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let mut expected_raw_ids = inlined_expected_permission_ids
            .iter()
            .copied()
            .map(i64::from)
            .collect::<Vec<_>>();
        expected_raw_ids.sort_unstable();
        if current_permission_ids != expected_raw_ids {
            return Ok::<_, crate::sqlx_admin_error::SqlxAdminError>(
                crate::replace_role_permissions_outcome::ReplaceRolePermissionsOutcome::StaleAssignment,
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
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        if usize::try_from(existing_count).ok() != Some(raw_ids.len()) {
            return Ok::<_, crate::sqlx_admin_error::SqlxAdminError>(
                crate::replace_role_permissions_outcome::ReplaceRolePermissionsOutcome::UnknownPermission,
            );
        }
        let _delete_result =
            sqlx::query(constants_str::SERVER_ADMIN_REPLACE_ROLE_PERMISSIONS_DELETE_SQL)
                .bind(inlined_role_permission_role_id.get())
                .execute(&mut *tx)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let _insert_result =
            sqlx::query(constants_str::SERVER_ADMIN_REPLACE_ROLE_PERMISSIONS_INSERT_SQL)
                .bind(inlined_role_permission_role_id.get())
                .bind(&raw_ids)
                .execute(&mut *tx)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        Ok::<_, crate::sqlx_admin_error::SqlxAdminError>(
            crate::replace_role_permissions_outcome::ReplaceRolePermissionsOutcome::Updated,
        )
    }
    .await
    .map_err(crate::admin_error::AdminError::from)?;
    match outcome {
        crate::replace_role_permissions_outcome::ReplaceRolePermissionsOutcome::Updated => {}
        crate::replace_role_permissions_outcome::ReplaceRolePermissionsOutcome::UnknownPermission => {
            return Err(crate::admin_error::AdminError::Validation);
        }
        crate::replace_role_permissions_outcome::ReplaceRolePermissionsOutcome::MissingRole
        | crate::replace_role_permissions_outcome::ReplaceRolePermissionsOutcome::StaleAssignment
        | crate::replace_role_permissions_outcome::ReplaceRolePermissionsOutcome::SystemRole => {
            return Err(crate::admin_error::AdminError::Conflict);
        }
    }
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
            crate::admin_audit_action::AdminAuditAction::Update,
            actor.get_login(),
            crate::admin_audit_resource::AdminAuditResource::Role,
            crate::admin_audit_resource_id::AdminAuditResourceId::Role(*path.get_inner()),
            *actor.get_id(),
        ),
    )
    .await?;
    tx.commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    Ok(crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
