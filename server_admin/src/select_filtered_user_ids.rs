pub(crate) async fn select_filtered_user_ids(
    mut sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    admin_user_filter: &server_admin_contract::admin_user_filter::AdminUserFilter,
) -> Result<
    pg_crud_common::list_items::ListItems<
        server_admin_core::admin_user_record_id::AdminUserRecordId,
    >,
    crate::admin_error::AdminError,
> {
    if admin_user_filter.user_id().is_none()
        && admin_user_filter.login().is_none()
        && admin_user_filter.display_name().is_none()
        && admin_user_filter.is_banned().is_none()
    {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let matches =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_SELECT_FILTERED_USERS_SQL)
            .bind(admin_user_filter.user_id().copied().map(i64::from))
            .bind(
                admin_user_filter
                    .login()
                    .map(|login| login.as_ref().as_str()),
            )
            .bind(
                admin_user_filter
                    .display_name()
                    .map(|display_name| display_name.as_ref().as_str()),
            )
            .bind(admin_user_filter.is_banned().copied().map(bool::from))
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
            server_admin_core::admin_user_record_id::AdminUserRecordId::try_from(identifier)
                .map_err(|error| {
                    crate::admin_error::AdminError::from(sqlx::Error::Decode(Box::new(error)))
                })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(pg_crud_common::list_items::ListItems::from)
}
