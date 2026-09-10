pub(crate) async fn role_mutations_create_many(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_create_roles_request: server_admin_contract::admin_create_roles_request::AdminCreateRolesRequest,
) -> Result<server_admin_contract::admin_role_ids::AdminRoleIds, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &admin_auth_request,
        server_admin_contract::admin_permission::AdminPermission::RolesCreate,
    )
    .await?;
    let requests = AsRef::<
        [server_admin_contract::admin_create_role_request::AdminCreateRoleRequest],
    >::as_ref(&admin_create_roles_request);
    if requests.is_empty() {
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
    let (completed_transaction, identifiers) = futures::TryStreamExt::try_fold(
        futures::stream::iter(requests.iter().map(Ok::<_, crate::admin_error::AdminError>)),
        (crate::sqlx_admin_transaction::SqlxAdminTransaction::from(transaction), Vec::with_capacity(requests.len())),
        async |(mut sqlx_admin_transaction, mut identifiers), request| {
            let identifier = sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_INSERT_ROLE_SQL)
                .bind(request.get_name().as_ref()).fetch_one(&mut **sqlx_admin_transaction).await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
                .and_then(|value| server_admin_core::admin_role_record_id::AdminRoleRecordId::try_from(value)
                    .map_err(crate::sqlx_admin_error::SqlxAdminError::from))
                .map_err(|error| crate::map_unique_violation::map_unique_violation(error.into_inner()))?;
            crate::record_audit_success_in_connection::record_audit_success_in_connection(
                crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut **sqlx_admin_transaction),
                crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
                    crate::admin_audit_action::AdminAuditAction::Create,
                    actor.get_login(), crate::admin_audit_resource::AdminAuditResource::Role,
                    crate::admin_audit_resource_id::AdminAuditResourceId::Role(identifier), actor.id(),
                ),
            ).await?;
            identifiers.push(server_admin_contract::admin_role_id::AdminRoleId::from(identifier.value()));
            Ok((sqlx_admin_transaction, identifiers))
        },
    ).await?;
    let role_ids = server_admin_contract::admin_role_ids::AdminRoleIds::try_from(identifiers)
        .map_err(
            |server_admin_contract::admin_collection_error::AdminCollectionError::TooLong| {
                crate::admin_error::AdminError::Validation
            },
        )?;
    sqlx::Transaction::from(completed_transaction)
        .commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    Ok(role_ids)
}
