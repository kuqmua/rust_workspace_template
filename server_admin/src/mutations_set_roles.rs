#![allow(clippy::single_call_fn)] // route inventory registers this user operation once

pub(in crate::domain_types::auth) async fn mutations_set_roles(
    auth: super::super::AdminAuthReq,
    path: super::super::AxumAdminPath<super::super::super::AdminUserId>,
    request: super::super::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserRolesReq>,
) -> Result<super::super::AxumAdminResponse, super::super::AdminError> {
    let actor = super::super::shared::authorize_custom::authorize_custom(
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
        crate::repository::roles::lock_last_admin(
            crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
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
                crate::repository::ReplaceUserRolesOutcome::MissingUser,
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
            return Ok(crate::repository::ReplaceUserRolesOutcome::StaleAssignment);
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
            return Ok(crate::repository::ReplaceUserRolesOutcome::UnknownRole);
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
                return Ok(crate::repository::ReplaceUserRolesOutcome::LastActiveAdministrator);
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
        crate::repository::revoke_user_sessions::revoke_user_sessions(
            crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            path.0,
        )
        .await?;
        Ok(crate::repository::ReplaceUserRolesOutcome::Updated)
    }
    .await
    .map_err(super::super::AdminError::from)?;
    match outcome {
        crate::repository::ReplaceUserRolesOutcome::Updated => {}
        crate::repository::ReplaceUserRolesOutcome::UnknownRole => {
            return Err(super::super::AdminError::Validation);
        }
        crate::repository::ReplaceUserRolesOutcome::LastActiveAdministrator
        | crate::repository::ReplaceUserRolesOutcome::MissingUser
        | crate::repository::ReplaceUserRolesOutcome::StaleAssignment => {
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
