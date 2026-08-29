pub(crate) async fn mutations_set_roles(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<server_admin_core::admin_user_id::AdminUserId>,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_set_user_roles_req::AdminSetUserRolesReq,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &auth,
        server_admin_contract::admin_permission::AdminPermission::UserRolesUpdate,
    )
    .await?;
    let (expected_role_ids, contract_role_ids) = request.0.into_parts();
    if AsRef::<[server_admin_contract::admin_role_id::AdminRoleId]>::as_ref(&expected_role_ids)
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len()
        != AsRef::<[server_admin_contract::admin_role_id::AdminRoleId]>::as_ref(&expected_role_ids)
            .len()
        || AsRef::<[server_admin_contract::admin_role_id::AdminRoleId]>::as_ref(&contract_role_ids)
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != AsRef::<[server_admin_contract::admin_role_id::AdminRoleId]>::as_ref(
                &contract_role_ids,
            )
            .len()
    {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    let outcome = async {
        crate::lock_last_admin::lock_last_admin(
            crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        )
        .await?;
        let optional_target_is_active =
            sqlx::query_scalar::<_, bool>(constants_str::integration_fixtures::SERVER_ADMIN_LOCK_USER_ACTIVE_STATE_SQL)
                .bind(path.0.get())
                .fetch_optional(&mut *tx)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let Some(target_is_active) = optional_target_is_active else {
            return Ok::<_, crate::sqlx_admin_error::SqlxAdminError>(
                crate::replace_user_roles_outcome::ReplaceUserRolesOutcome::MissingUser,
            );
        };
        let current_role_ids =
            sqlx::query_scalar::<_, i64>(constants_str::integration_fixtures::SERVER_ADMIN_READ_USER_ROLE_IDS_SQL)
                .bind(path.0.get())
                .fetch_all(&mut *tx)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let mut expected_raw_ids =
            AsRef::<[server_admin_contract::admin_role_id::AdminRoleId]>::as_ref(&expected_role_ids)
                .iter()
                .copied()
                .map(i64::from)
                .collect::<Vec<_>>();
        expected_raw_ids.sort_unstable();
        if current_role_ids != expected_raw_ids {
            return Ok(crate::replace_user_roles_outcome::ReplaceUserRolesOutcome::StaleAssignment);
        }
        let raw_ids =
            AsRef::<[server_admin_contract::admin_role_id::AdminRoleId]>::as_ref(&contract_role_ids)
                .iter()
                .copied()
                .map(i64::from)
                .collect::<Vec<_>>();
        let existing_count =
            sqlx::query_scalar::<_, i64>(constants_str::integration_fixtures::SERVER_ADMIN_COUNT_ROLES_SQL)
                .bind(&raw_ids)
                .fetch_one(&mut *tx)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        if usize::try_from(existing_count).ok() != Some(raw_ids.len()) {
            return Ok(crate::replace_user_roles_outcome::ReplaceUserRolesOutcome::UnknownRole);
        }
        let admin_role_id =
            sqlx::query_scalar::<_, i64>(constants_str::integration_fixtures::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
                .fetch_one(&mut *tx)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let target_was_admin =
            sqlx::query_scalar::<_, bool>(constants_str::integration_fixtures::SERVER_ADMIN_USER_HAS_ROLE_SQL)
                .bind(path.0.get())
                .bind(admin_role_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        if target_is_active && target_was_admin && !raw_ids.contains(&admin_role_id) {
            let active_admin_count = sqlx::query_scalar::<_, i64>(
                constants_str::integration_fixtures::SERVER_ADMIN_ACTIVE_ROLE_USER_COUNT_SQL,
            )
            .bind(admin_role_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
            if active_admin_count <= constants_i64::ONE {
                return Ok(crate::replace_user_roles_outcome::ReplaceUserRolesOutcome::LastActiveAdministrator);
            }
        }
        let _delete_result = sqlx::query(constants_str::integration_fixtures::SERVER_ADMIN_REPLACE_USER_ROLES_DELETE_SQL)
            .bind(path.0.get())
            .execute(&mut *tx)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let _insert_result = sqlx::query(constants_str::integration_fixtures::SERVER_ADMIN_REPLACE_USER_ROLES_INSERT_SQL)
            .bind(path.0.get())
            .bind(&raw_ids)
            .execute(&mut *tx)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        crate::revoke_user_sessions::revoke_user_sessions(
            crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            path.0,
        )
        .await?;
        Ok(crate::replace_user_roles_outcome::ReplaceUserRolesOutcome::Updated)
    }
    .await
    .map_err(crate::admin_error::AdminError::from)?;
    match outcome {
        crate::replace_user_roles_outcome::ReplaceUserRolesOutcome::Updated => {}
        crate::replace_user_roles_outcome::ReplaceUserRolesOutcome::UnknownRole => {
            return Err(crate::admin_error::AdminError::Validation);
        }
        crate::replace_user_roles_outcome::ReplaceUserRolesOutcome::LastActiveAdministrator
        | crate::replace_user_roles_outcome::ReplaceUserRolesOutcome::MissingUser
        | crate::replace_user_roles_outcome::ReplaceUserRolesOutcome::StaleAssignment => {
            return Err(crate::admin_error::AdminError::Conflict);
        }
    }
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef {
            action: crate::admin_audit_action::AdminAuditAction::Update,
            login: &actor.login,
            resource: crate::admin_audit_resource::AdminAuditResource::User,
            resource_id: crate::admin_audit_resource_id::AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    Ok(crate::axum_admin_response::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
