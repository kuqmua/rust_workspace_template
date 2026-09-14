pub(crate) async fn select_filtered_role_ids(
    mut sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    admin_role_filter: &server_admin_contract::admin_role_filter::AdminRoleFilter,
) -> Result<
    pg_crud_common::list_items::ListItems<
        server_admin_core::admin_role_record_id::AdminRoleRecordId,
    >,
    crate::admin_error::AdminError,
> {
    if admin_role_filter.get_role_id().is_none()
        && admin_role_filter.get_name().is_none()
        && admin_role_filter.get_is_system().is_none()
    {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let matches =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_SELECT_FILTERED_ROLES_SQL)
            .bind(admin_role_filter.get_role_id().copied().map(i64::from))
            .bind(
                admin_role_filter
                    .get_name()
                    .map(|admin_role_name| admin_role_name.as_ref().as_str()),
            )
            .bind(admin_role_filter.get_is_system().copied().map(bool::from))
            .fetch_all(&mut **sqlx_admin_repository_connection_mut_ref)
            .await
            .map_err(crate::admin_error::AdminError::from)?;
    if matches.is_empty() {
        return Err(crate::admin_error::AdminError::Conflict);
    }
    if matches.len() > 10_000 {
        return Err(crate::admin_error::AdminError::Validation);
    }
    matches
        .into_iter()
        .map(|identifier| {
            server_admin_core::admin_role_record_id::AdminRoleRecordId::try_from(identifier)
                .map_err(|error| {
                    crate::admin_error::AdminError::from(sqlx::Error::Decode(Box::new(error)))
                })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(pg_crud_common::list_items::ListItems::from)
}
