pub(crate) async fn role_mutations_delete_filtered(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_role_filter: &server_admin_contract::admin_role_filter::AdminRoleFilter,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorize_custom::authorize_custom(
        &admin_auth_request,
        server_admin_contract::admin_permission::AdminPermission::RolesDelete,
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
    let selected = Vec::from(crate::select_filtered_role_ids::select_filtered_role_ids(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *transaction,
        ),
        admin_role_filter,
    )
    .await?);
    let completed_transaction = futures::TryStreamExt::try_fold(
        futures::stream::iter(selected.into_iter().map(Ok::<_, crate::admin_error::AdminError>)),
        crate::sqlx_admin_transaction::SqlxAdminTransaction::from(transaction),
        async |mut sqlx_admin_transaction, admin_role_record_id| {
            sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_DELETE_ROLE_SQL)
                .bind(admin_role_record_id.get())
                .fetch_optional(&mut **sqlx_admin_transaction)
                .await
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
                .map(|value| server_admin_core::std_admin_bool::StdAdminBool::from(value.is_some()))
                .map_err(crate::admin_error::AdminError::from)?
                .get()
                .then_some(())
                .ok_or(crate::admin_error::AdminError::Conflict)?;
            crate::record_audit_success_in_connection::record_audit_success_in_connection(
                crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut **sqlx_admin_transaction),
                crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
                    crate::admin_audit_action::AdminAuditAction::Delete,
                    actor.get_login(),
                    crate::admin_audit_resource::AdminAuditResource::Role,
                    crate::admin_audit_resource_id::AdminAuditResourceId::Role(admin_role_record_id),
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
