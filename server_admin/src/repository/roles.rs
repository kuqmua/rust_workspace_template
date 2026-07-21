#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct AdminActiveAdministratorCount(i64);

#[derive(Clone, Copy, Debug)]
pub(crate) struct LastAdminState {
    active_count: AdminActiveAdministratorCount,
    target_is_admin: crate::StdAdminBool,
}
impl LastAdminState {
    pub(crate) fn would_remove_last(self) -> crate::StdAdminBool {
        crate::StdAdminBool::from(self.target_is_admin.get() && self.active_count.0 <= 1i64)
    }
}

pub(crate) async fn insert_role(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    name: &server_admin_contract::AdminRoleName,
) -> Result<crate::AdminRoleId, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_INSERT_ROLE_SQL)
        .bind(name.as_ref())
        .fetch_one(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(crate::AdminRoleId::from)
}

pub(crate) async fn update_role(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    role_id: crate::AdminRoleId,
    name: &server_admin_contract::AdminRoleName,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_UPDATE_ROLE_SQL)
        .bind(role_id.get())
        .bind(name.as_ref())
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}

pub(crate) async fn delete_role(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    role_id: crate::AdminRoleId,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_DELETE_ROLE_SQL)
        .bind(role_id.get())
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}

pub(crate) async fn list_role_catalog(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
) -> Result<server_admin_contract::AdminRoleSummaries, super::AdminRepositoryError> {
    let rows = sqlx::query_as::<_, (i64, String, bool)>(str_constants::SERVER_ADMIN_LIST_ROLES_SQL)
        .fetch_all(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let role_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
    let links =
        sqlx::query_as::<_, (i64, i64)>(str_constants::SERVER_ADMIN_LIST_ROLE_PERMISSION_IDS_SQL)
            .bind(role_ids.as_slice())
            .fetch_all(pool.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    let mut permission_ids_by_role = links.into_iter().fold(
        std::collections::HashMap::<i64, Vec<server_admin_contract::AdminPermissionId>>::new(),
        |mut values, (role_id, permission_id)| {
            values.entry(role_id).or_default().push(
                server_admin_contract::AdminPermissionId::from(permission_id),
            );
            values
        },
    );
    rows.into_iter()
        .map(|(id, name, is_system)| {
            Ok(server_admin_contract::AdminRoleSummary::new(
                server_admin_contract::AdminRoleId::from(id),
                server_admin_contract::AdminBool::from(is_system),
                server_admin_contract::AdminRoleName::try_from(name)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::AdminPermissionIds::try_from(
                    permission_ids_by_role.remove(&id).unwrap_or_default(),
                )
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()
        .and_then(|values| {
            server_admin_contract::AdminRoleSummaries::try_from(values)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
        })
}

pub(crate) async fn list_roles(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    query: &server_admin_contract::AdminTableQuery,
) -> Result<
    (
        server_admin_contract::AdminRoleSummaries,
        super::AdminPageTotalCount,
    ),
    super::AdminRepositoryError,
> {
    let search = query.search().as_ref();
    let total = sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_COUNT_FILTERED_ROLES_SQL)
        .bind(search)
        .fetch_one(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let rows = sqlx::query_as::<_, (i64, String, bool)>(str_constants::SERVER_ADMIN_PAGE_ROLES_SQL)
        .bind(search)
        .bind(query.sort().as_ref())
        .bind(query.direction().as_ref())
        .bind(i64::from(u16::from(query.limit())))
        .bind(i64::from(u32::from(query.offset())))
        .fetch_all(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let role_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
    let links =
        sqlx::query_as::<_, (i64, i64)>(str_constants::SERVER_ADMIN_LIST_ROLE_PERMISSION_IDS_SQL)
            .bind(role_ids.as_slice())
            .fetch_all(pool.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    let mut permission_ids_by_role = links.into_iter().fold(
        std::collections::HashMap::<i64, Vec<server_admin_contract::AdminPermissionId>>::new(),
        |mut values, (role_id, permission_id)| {
            values.entry(role_id).or_default().push(
                server_admin_contract::AdminPermissionId::from(permission_id),
            );
            values
        },
    );
    let items = rows
        .into_iter()
        .map(|(id, name, is_system)| {
            Ok(server_admin_contract::AdminRoleSummary::new(
                server_admin_contract::AdminRoleId::from(id),
                server_admin_contract::AdminBool::from(is_system),
                server_admin_contract::AdminRoleName::try_from(name)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::AdminPermissionIds::try_from(
                    permission_ids_by_role.remove(&id).unwrap_or_default(),
                )
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            ))
        })
        .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    Ok((
        server_admin_contract::AdminRoleSummaries::try_from(items)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        super::AdminPageTotalCount::from(total),
    ))
}

pub(crate) async fn replace_user_roles(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    expected_role_ids: &[server_admin_contract::AdminRoleId],
    role_ids: &[server_admin_contract::AdminRoleId],
) -> Result<super::ReplaceUserRolesOutcome, crate::SqlxAdminError> {
    lock_last_admin(super::SqlxAdminRepositoryConnectionMutRef::from(
        &mut *connection.0,
    ))
    .await?;
    let optional_target_is_active =
        sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_LOCK_USER_ACTIVE_STATE_SQL)
            .bind(user_id.get())
            .fetch_optional(&mut *connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    let Some(target_is_active) = optional_target_is_active else {
        return Ok(super::ReplaceUserRolesOutcome::MissingUser);
    };
    let current_role_ids =
        sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_READ_USER_ROLE_IDS_SQL)
            .bind(user_id.get())
            .fetch_all(&mut *connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    let mut expected_raw_ids = expected_role_ids
        .iter()
        .copied()
        .map(i64::from)
        .collect::<Vec<_>>();
    expected_raw_ids.sort_unstable();
    if current_role_ids != expected_raw_ids {
        return Ok(super::ReplaceUserRolesOutcome::StaleAssignment);
    }
    let raw_ids = role_ids.iter().copied().map(i64::from).collect::<Vec<_>>();
    let existing_count = sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_COUNT_ROLES_SQL)
        .bind(&raw_ids)
        .fetch_one(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    if usize::try_from(existing_count).ok() != Some(raw_ids.len()) {
        return Ok(super::ReplaceUserRolesOutcome::UnknownRole);
    }
    let admin_role_id =
        sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
            .fetch_one(&mut *connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    let target_was_admin =
        sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_USER_HAS_ROLE_SQL)
            .bind(user_id.get())
            .bind(admin_role_id)
            .fetch_one(&mut *connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    if target_is_active && target_was_admin && !raw_ids.contains(&admin_role_id) {
        let active_admin_count =
            sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_ACTIVE_ROLE_USER_COUNT_SQL)
                .bind(admin_role_id)
                .fetch_one(&mut *connection.0)
                .await
                .map_err(crate::SqlxAdminError::from)?;
        if active_admin_count <= 1i64 {
            return Ok(super::ReplaceUserRolesOutcome::LastActiveAdministrator);
        }
    }
    let _delete_result = sqlx::query(str_constants::SERVER_ADMIN_REPLACE_USER_ROLES_DELETE_SQL)
        .bind(user_id.get())
        .execute(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let _insert_result = sqlx::query(str_constants::SERVER_ADMIN_REPLACE_USER_ROLES_INSERT_SQL)
        .bind(user_id.get())
        .bind(&raw_ids)
        .execute(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    super::sessions::revoke_user_sessions(connection, user_id).await?;
    Ok(super::ReplaceUserRolesOutcome::Updated)
}

#[allow(clippy::single_call_fn)] // one transaction-level operation owns the lock/read ordering for the last-administrator invariant
pub(crate) async fn lock_and_read_last_admin_state(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<LastAdminState, crate::SqlxAdminError> {
    lock_last_admin(super::SqlxAdminRepositoryConnectionMutRef::from(
        &mut *connection.0,
    ))
    .await?;
    read_last_admin_state(connection, user_id).await
}

pub(crate) async fn lock_last_admin(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
) -> Result<(), crate::SqlxAdminError> {
    sqlx::query(str_constants::SERVER_ADMIN_LOCK_LAST_ADMIN_SQL)
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn read_last_admin_state(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<LastAdminState, crate::SqlxAdminError> {
    let target_is_admin =
        sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_USER_IS_ADMIN_SQL)
            .bind(user_id.get())
            .fetch_one(&mut *connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    let active_count =
        sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_ACTIVE_ADMIN_COUNT_SQL)
            .fetch_one(connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    Ok(LastAdminState {
        active_count: AdminActiveAdministratorCount::from(active_count),
        target_is_admin: crate::StdAdminBool::from(target_is_admin),
    })
}
