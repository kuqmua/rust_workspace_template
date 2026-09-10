pub(crate) async fn user_mutations_delete_filtered(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_user_filter: &server_admin_contract::admin_user_filter::AdminUserFilter,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &admin_auth_request,
        server_admin_contract::admin_permission::AdminPermission::UsersDelete,
    )
    .await?;
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
    let selected = Vec::from(crate::select_filtered_user_ids::select_filtered_user_ids(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut *transaction),
        admin_user_filter,
    ).await?);
    if selected.contains(actor.get_id()) {
        return Err(crate::admin_error::AdminError::Conflict);
    }
    let completed_transaction = futures::TryStreamExt::try_fold(
        futures::stream::iter(selected.into_iter().map(Ok::<_, crate::admin_error::AdminError>)),
        crate::sqlx_admin_transaction::SqlxAdminTransaction::from(transaction),
        async |mut sqlx_admin_transaction, admin_user_record_id| {
            let state = crate::read_last_admin_state::read_last_admin_state(
                crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut **sqlx_admin_transaction),
                admin_user_record_id,
            ).await?;
            if state.would_remove_last().get() {
                return Err(crate::admin_error::AdminError::Conflict);
            }
            let _deleted = sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_DELETE_USER_SQL)
                .bind(admin_user_record_id.get()).fetch_optional(&mut **sqlx_admin_transaction)
                .await.map_err(crate::admin_error::AdminError::from)?
                .ok_or(crate::admin_error::AdminError::Conflict)?;
            crate::record_audit_success_in_connection::record_audit_success_in_connection(
                crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut **sqlx_admin_transaction),
                crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
                    crate::admin_audit_action::AdminAuditAction::Delete,
                    actor.get_login(),
                    crate::admin_audit_resource::AdminAuditResource::User,
                    crate::admin_audit_resource_id::AdminAuditResourceId::User(admin_user_record_id),
                    actor.id(),
                ),
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
