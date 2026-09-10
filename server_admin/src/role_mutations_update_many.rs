pub(crate) async fn role_mutations_update_many(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_role_update_slice: crate::admin_role_update_slice::AdminRoleUpdateSlice<'_>,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &admin_auth_request,
        server_admin_contract::admin_permission::AdminPermission::RolesUpdate,
    )
    .await?;
    let updates = admin_role_update_slice.as_ref();
    if updates.is_empty()
        || updates.iter().any(|update| {
            let filter = update.filter();
            filter.get_role_id().is_none()
                && filter.get_name().is_none()
                && filter.get_is_system().is_none()
        })
    {
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
            let filter = update.filter();
            let matches =
                sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_SELECT_FILTERED_ROLES_SQL)
                    .bind(filter.get_role_id().copied().map(i64::from))
                    .bind(filter.get_name().map(|name| name.as_ref().as_str()))
                    .bind(filter.get_is_system().copied().map(bool::from))
                    .fetch_all(&mut **sqlx_admin_transaction)
                    .await
                    .map_err(crate::admin_error::AdminError::from)?;
            if matches.is_empty() {
                return Err(crate::admin_error::AdminError::Conflict);
            }
            if selected_updates.len().saturating_add(matches.len()) > 10_000 {
                return Err(crate::admin_error::AdminError::Validation);
            }
            matches.into_iter().try_for_each(|value| {
                let identifier =
                    server_admin_core::admin_role_record_id::AdminRoleRecordId::try_from(value)
                        .map_err(|error| {
                            crate::admin_error::AdminError::from(sqlx::Error::Decode(Box::new(
                                error,
                            )))
                        })?;
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
            sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_ROLE_SQL)
                .bind(identifier.get()).bind(changes.get_name().as_ref())
                .fetch_optional(&mut **sqlx_admin_transaction).await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
                .map_err(|error| crate::map_unique_violation::map_unique_violation(error.into_inner()))?
                .is_some().then_some(()).ok_or(crate::admin_error::AdminError::Conflict)?;
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
