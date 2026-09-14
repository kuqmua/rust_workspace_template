pub(crate) async fn role_mutations_update_many(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_role_update_slice: crate::admin_role_update_slice::AdminRoleUpdateSlice<'_>,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let updates = admin_role_update_slice.as_ref();
    let updates_names = updates
        .iter()
        .any(|update| update.changes().get_name().is_some());
    let updates_permissions = updates
        .iter()
        .any(|update| update.changes().get_permissions().is_some());
    if updates.is_empty()
        || updates.iter().any(|update| {
            let filter = update.filter();
            filter.get_role_id().is_none()
                && filter.get_name().is_none()
                && filter.get_is_system().is_none()
                || update.changes().get_name().is_none()
                    && update.changes().get_permissions().is_none()
                || update.changes().get_permissions().is_some()
                    && (filter.get_role_id().is_none()
                        || filter.get_name().is_some()
                        || filter.get_is_system().is_some())
        })
    {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let actor = crate::authorize_custom::authorize_custom(
        &admin_auth_request,
        if updates_names {
            server_admin_contract::admin_permission::AdminPermission::RolesUpdate
        } else {
            server_admin_contract::admin_permission::AdminPermission::RolePermissionsUpdate
        },
    )
    .await?;
    if updates_names && updates_permissions {
        let _permission_actor = crate::authorize_custom::authorize_custom(
            &admin_auth_request,
            server_admin_contract::admin_permission::AdminPermission::RolePermissionsUpdate,
        )
        .await?;
    }
    if updates.iter().any(|update| {
        update
            .changes()
            .get_permissions()
            .is_some_and(|permissions| {
                let expected_permission_ids = permissions.expected_permission_ids().as_ref();
                let permission_ids = permissions.permission_ids().as_ref();
                expected_permission_ids
                    .iter()
                    .collect::<std::collections::HashSet<_>>()
                    .len()
                    != expected_permission_ids.len()
                    || permission_ids
                        .iter()
                        .collect::<std::collections::HashSet<_>>()
                        .len()
                        != permission_ids.len()
            })
    }) {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let transaction = admin_auth_request
        .get_state()
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    let (selected_transaction, _, mut selected) = futures::TryStreamExt::try_fold(
        futures::stream::iter(updates.iter().map(Ok::<_, crate::admin_error::AdminError>)),
        (
            crate::sqlx_admin_transaction::SqlxAdminTransaction::from(transaction),
            std::collections::BTreeSet::new(),
            Vec::new(),
        ),
        async |(mut sqlx_admin_transaction, mut identifiers, mut selected_updates), update| {
            let matches = Vec::from(crate::select_filtered_role_ids::select_filtered_role_ids(
                crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut **sqlx_admin_transaction),
                update.filter(),
            ).await?);
            if selected_updates.len().saturating_add(matches.len()) > 10_000 {
                return Err(crate::admin_error::AdminError::Validation);
            }
            matches.into_iter().try_for_each(|identifier| {
                if !identifiers.insert(identifier) {
                    return Err(crate::admin_error::AdminError::Validation);
                }
                selected_updates.push((identifier, update.changes()));
                Ok(())
            })?;
            Ok((sqlx_admin_transaction, identifiers, selected_updates))
        },
    )
    .await?;
    selected.sort_by_key(|(identifier, _changes)| identifier.get());
    let completed_transaction = futures::TryStreamExt::try_fold(
        futures::stream::iter(selected.into_iter().map(Ok::<_, crate::admin_error::AdminError>)),
        selected_transaction,
        async |mut sqlx_admin_transaction, (identifier, changes)| {
            if let Some(name) = changes.get_name() {
                sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_ROLE_SQL)
                    .bind(identifier.get()).bind(name.as_ref())
                    .fetch_optional(&mut **sqlx_admin_transaction).await
                    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
                    .map_err(|error| crate::map_unique_violation::map_unique_violation(error.into_inner()))?
                    .is_some().then_some(()).ok_or(crate::admin_error::AdminError::Conflict)?;
            }
            if let Some(permissions) = changes.get_permissions() {
                let optional_is_system = sqlx::query_scalar::<_, bool>(
                    constants_str::SERVER_ADMIN_LOCK_ROLE_SYSTEM_STATE_SQL,
                )
                .bind(identifier.get())
                .fetch_optional(&mut **sqlx_admin_transaction)
                .await
                .map_err(crate::admin_error::AdminError::from)?;
                if optional_is_system.is_none_or(|is_system| is_system) {
                    return Err(crate::admin_error::AdminError::Conflict);
                }
                let current_permission_ids = sqlx::query_scalar::<_, i64>(
                    constants_str::SERVER_ADMIN_READ_ROLE_PERMISSION_IDS_SQL,
                )
                .bind(identifier.get())
                .fetch_all(&mut **sqlx_admin_transaction)
                .await
                .map_err(crate::admin_error::AdminError::from)?;
                let mut expected_permission_ids = permissions
                    .expected_permission_ids()
                    .as_ref()
                    .iter()
                    .copied()
                    .map(i64::from)
                    .collect::<Vec<_>>();
                #[allow(clippy::stable_sort_primitive, reason = "role permission optimistic concurrency compares the stored canonical order")]
                expected_permission_ids.sort();
                if current_permission_ids != expected_permission_ids {
                    return Err(crate::admin_error::AdminError::Conflict);
                }
                let permission_ids = permissions
                    .permission_ids()
                    .as_ref()
                    .iter()
                    .copied()
                    .map(i64::from)
                    .collect::<Vec<_>>();
                let existing_count = sqlx::query_scalar::<_, i64>(
                    constants_str::SERVER_ADMIN_COUNT_PERMISSIONS_SQL,
                )
                .bind(&permission_ids)
                .fetch_one(&mut **sqlx_admin_transaction)
                .await
                .map_err(crate::admin_error::AdminError::from)?;
                if usize::try_from(existing_count).ok() != Some(permission_ids.len()) {
                    return Err(crate::admin_error::AdminError::Validation);
                }
                let _delete_result = sqlx::query(
                    constants_str::SERVER_ADMIN_REPLACE_ROLE_PERMISSIONS_DELETE_SQL,
                )
                .bind(identifier.get())
                .execute(&mut **sqlx_admin_transaction)
                .await
                .map_err(crate::admin_error::AdminError::from)?;
                let _insert_result = sqlx::query(
                    constants_str::SERVER_ADMIN_REPLACE_ROLE_PERMISSIONS_INSERT_SQL,
                )
                .bind(identifier.get())
                .bind(&permission_ids)
                .execute(&mut **sqlx_admin_transaction)
                .await
                .map_err(crate::admin_error::AdminError::from)?;
            }
            crate::record_audit_success_in_connection::record_audit_success_in_connection(
                crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut **sqlx_admin_transaction),
                crate::admin_audit_success_ref::AdminAuditSuccessRef::new(crate::admin_audit_action::AdminAuditAction::Update, actor.get_login(), crate::admin_audit_resource::AdminAuditResource::Role, crate::admin_audit_resource_id::AdminAuditResourceId::Role(identifier), actor.id()),
            ).await?;
            Ok(sqlx_admin_transaction)
        },
    ).await?;
    sqlx::Transaction::from(completed_transaction)
        .commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    Ok(crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
