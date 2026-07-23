#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

pub(crate) async fn list_permission_catalog(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
) -> Result<server_admin_contract::AdminPermissionSummaries, super::AdminRepositoryError> {
    sqlx::query_as::<_, (i64, String)>(str_constants::SERVER_ADMIN_LIST_PERMISSIONS_SQL)
        .fetch_all(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .into_iter()
        .map(|(id, name)| {
            Ok(server_admin_contract::AdminPermissionSummary::new(
                server_admin_contract::AdminPermissionId::try_from(id)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::AdminPermissionValue::try_from(name)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()
        .and_then(|values| {
            server_admin_contract::AdminPermissionSummaries::try_from(values)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
        })
}

pub(crate) async fn list_permissions(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    query: &server_admin_contract::AdminTableQuery,
) -> Result<
    (
        server_admin_contract::AdminPermissionSummaries,
        super::AdminPageTotalCount,
    ),
    super::AdminRepositoryError,
> {
    let search = query.search().as_ref();
    let total =
        sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_COUNT_FILTERED_PERMISSIONS_SQL)
            .bind(search)
            .fetch_one(pool.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    let items =
        sqlx::query_as::<_, (i64, String)>(str_constants::SERVER_ADMIN_PAGE_PERMISSIONS_SQL)
            .bind(search)
            .bind(query.sort().as_ref())
            .bind(query.direction().as_ref())
            .bind(i64::from(u16::from(query.limit())))
            .bind(i64::from(u32::from(query.offset())))
            .fetch_all(pool.0)
            .await
            .map_err(crate::SqlxAdminError::from)?
            .into_iter()
            .map(|(id, name)| {
                Ok(server_admin_contract::AdminPermissionSummary::new(
                    server_admin_contract::AdminPermissionId::try_from(id)
                        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                    server_admin_contract::AdminPermissionValue::try_from(name)
                        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                ))
            })
            .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    Ok((
        server_admin_contract::AdminPermissionSummaries::try_from(items)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        super::AdminPageTotalCount::from(total),
    ))
}

pub(crate) async fn replace_role_permissions(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    role_id: crate::AdminRoleId,
    expected_permission_ids: &[server_admin_contract::AdminPermissionId],
    permission_ids: &[server_admin_contract::AdminPermissionId],
) -> Result<super::ReplaceRolePermissionsOutcome, crate::SqlxAdminError> {
    let optional_is_system =
        sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_LOCK_ROLE_SYSTEM_STATE_SQL)
            .bind(role_id.get())
            .fetch_optional(&mut *connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    let Some(is_system) = optional_is_system else {
        return Ok(super::ReplaceRolePermissionsOutcome::MissingRole);
    };
    if is_system {
        return Ok(super::ReplaceRolePermissionsOutcome::SystemRole);
    }
    let current_permission_ids =
        sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_READ_ROLE_PERMISSION_IDS_SQL)
            .bind(role_id.get())
            .fetch_all(&mut *connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    let mut expected_raw_ids = expected_permission_ids
        .iter()
        .copied()
        .map(i64::from)
        .collect::<Vec<_>>();
    expected_raw_ids.sort_unstable();
    if current_permission_ids != expected_raw_ids {
        return Ok(super::ReplaceRolePermissionsOutcome::StaleAssignment);
    }
    let raw_ids = permission_ids
        .iter()
        .copied()
        .map(i64::from)
        .collect::<Vec<_>>();
    let existing_count =
        sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_COUNT_PERMISSIONS_SQL)
            .bind(&raw_ids)
            .fetch_one(&mut *connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    if usize::try_from(existing_count).ok() != Some(raw_ids.len()) {
        return Ok(super::ReplaceRolePermissionsOutcome::UnknownPermission);
    }
    let _delete_result =
        sqlx::query(str_constants::SERVER_ADMIN_REPLACE_ROLE_PERMISSIONS_DELETE_SQL)
            .bind(role_id.get())
            .execute(&mut *connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    let _insert_result =
        sqlx::query(str_constants::SERVER_ADMIN_REPLACE_ROLE_PERMISSIONS_INSERT_SQL)
            .bind(role_id.get())
            .bind(&raw_ids)
            .execute(connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    Ok(super::ReplaceRolePermissionsOutcome::Updated)
}
