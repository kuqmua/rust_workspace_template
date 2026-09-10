pub(crate) async fn user_mutations_update_filtered(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_user_update_slice: crate::admin_user_update_slice::AdminUserUpdateSlice<'_>,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let apply_user_update_in_connection = async |
    mut sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    runtime_authenticated_admin: &crate::runtime_authenticated_admin::RuntimeAuthenticatedAdmin,
    admin_user_record_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
    admin_update_user_request: &server_admin_contract::admin_update_user_request::AdminUpdateUserRequest,
| -> Result<(), crate::admin_error::AdminError> {
    let is_banned = admin_update_user_request
        .is_banned()
        .copied()
        .map(bool::from);
    if admin_update_user_request.login().is_none()
        && admin_update_user_request.display_name().is_none()
        && is_banned.is_none()
    {
        return Err(crate::admin_error::AdminError::Validation);
    }
    if is_banned == Some(true) && runtime_authenticated_admin.id() == admin_user_record_id {
        return Err(crate::admin_error::AdminError::Conflict);
    }
    if is_banned.is_some() {
        crate::lock_last_admin::lock_last_admin(
            crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
                &mut **sqlx_admin_repository_connection_mut_ref,
            ),
        ).await?;
        if is_banned == Some(true) {
            let state = crate::read_last_admin_state::read_last_admin_state(
                crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
                    &mut **sqlx_admin_repository_connection_mut_ref,
                ),
                admin_user_record_id,
            ).await?;
            if state.would_remove_last().get() {
                return Err(crate::admin_error::AdminError::Conflict);
            }
        }
    }
    let _stored_is_banned =
        sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_SQL)
            .bind(admin_user_record_id.get())
            .bind(
                admin_update_user_request
                    .login()
                    .as_ref()
                    .map(|value| value.as_ref().as_str()),
            )
            .bind(
                admin_update_user_request
                    .display_name()
                    .as_ref()
                    .map(|value| value.as_ref().as_str()),
            )
            .bind(is_banned)
            .fetch_optional(&mut **sqlx_admin_repository_connection_mut_ref)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
            .map_err(|error| crate::map_unique_violation::map_unique_violation(error.into_inner()))?
            .ok_or(crate::admin_error::AdminError::Conflict)?;
    if is_banned == Some(true) {
        crate::revoke_user_sessions::revoke_user_sessions(
            crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
                &mut **sqlx_admin_repository_connection_mut_ref,
            ),
            admin_user_record_id,
        ).await?;
    }
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        sqlx_admin_repository_connection_mut_ref,
        crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
            crate::admin_audit_action::AdminAuditAction::Update,
            runtime_authenticated_admin.get_login(),
            crate::admin_audit_resource::AdminAuditResource::User,
            crate::admin_audit_resource_id::AdminAuditResourceId::User(admin_user_record_id),
            runtime_authenticated_admin.id(),
        ),
    )
    .await
};
    let actor = crate::authorize_custom::authorize_custom(
        &admin_auth_request,
        server_admin_contract::admin_permission::AdminPermission::UsersUpdate,
    )
    .await?;
    let updates = admin_user_update_slice.as_ref();
    if updates.is_empty()
        || updates.iter().any(|update| {
            let filter = update.filter();
            let changes = update.changes();
            (filter.user_id().is_none()
                && filter.login().is_none()
                && filter.display_name().is_none()
                && filter.is_banned().is_none())
                || (changes.login().is_none()
                    && changes.display_name().is_none()
                    && changes.is_banned().is_none())
        })
    {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let mut transaction = admin_auth_request
        .get_state()
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    crate::lock_last_admin::lock_last_admin(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *transaction,
        ),
    )
    .await?;
    let (selected_transaction, _, mut selected) = futures::TryStreamExt::try_fold(
        futures::stream::iter(updates.iter().map(Ok::<_, crate::admin_error::AdminError>)),
        (
            crate::sqlx_admin_transaction::SqlxAdminTransaction::from(transaction),
            std::collections::BTreeSet::new(),
            Vec::new(),
        ),
        async |(mut sqlx_admin_transaction, mut identifiers, mut selected_updates), update| {
            let filter = update.filter();
            let matches =
                sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_SELECT_FILTERED_USERS_SQL)
                    .bind(filter.user_id().copied().map(i64::from))
                    .bind(filter.login().map(|login| login.as_ref().as_str()))
                    .bind(
                        filter
                            .display_name()
                            .map(|display_name| display_name.as_ref().as_str()),
                    )
                    .bind(filter.is_banned().copied().map(bool::from))
                    .fetch_all(&mut **sqlx_admin_transaction)
                    .await
                    .map_err(crate::admin_error::AdminError::from)?;
            if matches.is_empty() {
                return Err(crate::admin_error::AdminError::Conflict);
            }
            if selected_updates.len().saturating_add(matches.len()) > 10_000 {
                return Err(crate::admin_error::AdminError::Validation);
            }
            matches.into_iter().try_for_each(|identifier| {
                if !identifiers.insert(identifier) {
                    return Err(crate::admin_error::AdminError::Validation);
                }
                let admin_user_record_id =
                    server_admin_core::admin_user_record_id::AdminUserRecordId::try_from(
                        identifier,
                    )
                    .map_err(|error| {
                        crate::admin_error::AdminError::from(sqlx::Error::Decode(Box::new(error)))
                    })?;
                selected_updates.push((admin_user_record_id, update.changes()));
                Ok(())
            })?;
            Ok((sqlx_admin_transaction, identifiers, selected_updates))
        },
    )
    .await?;
    selected.sort_by_key(|(identifier, changes)| {
        (
            changes.is_banned().copied().is_some_and(bool::from),
            identifier.get(),
        )
    });
    let completed_transaction = futures::TryStreamExt::try_fold(
        futures::stream::iter(selected.into_iter().map(Ok::<_, crate::admin_error::AdminError>)),
        selected_transaction,
        async |mut sqlx_admin_transaction, (admin_user_record_id, changes)| {
            apply_user_update_in_connection(
                crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut **sqlx_admin_transaction),
                &actor,
                admin_user_record_id,
                changes,
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
