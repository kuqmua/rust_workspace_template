pub(crate) async fn user_mutations_create(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_create_users_request: server_admin_contract::admin_create_users_request::AdminCreateUsersRequest,
) -> Result<server_admin_contract::admin_user_ids::AdminUserIds, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &admin_auth_request,
        server_admin_contract::admin_rule::AdminRule::UsersCreate,
    )
    .await?;
    let requests = AsRef::<
        [server_admin_contract::admin_create_user_request::AdminCreateUserRequest],
    >::as_ref(&admin_create_users_request);
    if requests.is_empty() {
        return Err(crate::admin_error::AdminError::Validation);
    }
    if requests
        .iter()
        .any(|request| request.get_role_ids().is_some())
    {
        let _role_actor = crate::authorize_custom::authorize_custom(
            &admin_auth_request,
            server_admin_contract::admin_rule::AdminRule::UserRolesUpdate,
        )
        .await?;
    }
    let mut sqlx_transaction = admin_auth_request
        .get_state()
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    crate::lock_last_admin::lock_last_admin(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *sqlx_transaction,
        ),
    )
    .await?;
    let (transaction, identifiers) = futures::TryStreamExt::try_fold(
        futures::stream::iter(requests.iter().map(Ok::<_, crate::admin_error::AdminError>)),
        (
            crate::sqlx_admin_transaction::SqlxAdminTransaction::from(sqlx_transaction),
            Vec::with_capacity(requests.len()),
        ),
        async |(mut transaction, mut identifiers), request| {
            let password = crate::admin_new_password_from_contract::admin_new_password_from_contract(
                request.get_password().clone(),
            )
            .map_err(crate::admin_error::AdminError::password_text)?;
            let password_hash = admin_auth_request
                .get_state()
                .as_ref()
                .get_password_hasher()
                .hash(password)
                .await
                .map_err(crate::admin_error::AdminError::password_hash)?;
            let user_id = crate::insert_user::insert_user(
                crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
                    &mut **transaction,
                ),
                request.get_login(),
                request.get_display_name(),
                &password_hash,
            )
            .await
            .map_err(|error| crate::map_unique_violation::map_unique_violation(error.into_inner()))?;
            if let Some(role_ids) = request.get_role_ids() {
                crate::replace_user_roles_in_connection::replace_user_roles_in_connection(
                    crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
                        &mut **transaction,
                    ),
                    server_admin_core::admin_user_record_id::AdminUserRecordId::from(user_id.value()),
                    role_ids,
                    None,
                )
                .await?;
            }
            crate::record_audit_success_in_connection::record_audit_success_in_connection(
                crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
                    &mut **transaction,
                ),
                crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
                    crate::admin_audit_action::AdminAuditAction::Create,
                    actor.get_login(),
                    crate::admin_audit_resource::AdminAuditResource::User,
                    crate::admin_audit_resource_id::AdminAuditResourceId::User(
                        server_admin_core::admin_user_record_id::AdminUserRecordId::from(user_id.value()),
                    ),
                    *actor.get_id(),
                ),
            )
            .await?;
            identifiers.push(server_admin_contract::admin_user_id::AdminUserId::from(user_id.value()));
            Ok((transaction, identifiers))
        },
    )
    .await?;
    let user_ids = server_admin_contract::admin_user_ids::AdminUserIds::try_from(identifiers)
        .map_err(
            |server_admin_contract::admin_collection_error::AdminCollectionError::TooLong| {
                crate::admin_error::AdminError::Validation
            },
        )?;
    sqlx::Transaction::from(transaction)
        .commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    Ok(user_ids)
}
