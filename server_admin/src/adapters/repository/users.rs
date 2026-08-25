#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

pub(crate) async fn insert_user(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    login: &server_admin_contract::domain_types::AdminLogin,
    display_name: &server_admin_contract::domain_types::AdminDisplayName,
    password_hash: &crate::domain_types::AdminPasswordHash,
) -> Result<crate::domain_types::AdminUserId, crate::domain_types::SqlxAdminError> {
    sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_INSERT_USER_SQL)
        .bind(login.as_ref())
        .bind(display_name.as_ref())
        .bind(password_hash.expose().as_ref())
        .fetch_one(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .and_then(|value| {
            crate::domain_types::AdminUserId::try_from(value)
                .map_err(crate::domain_types::SqlxAdminError::from)
        })
}

pub(crate) async fn recent_login_failure_count(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    login: &server_admin_contract::domain_types::AdminLogin,
) -> Result<super::AdminRecentLoginFailureCount, crate::domain_types::SqlxAdminError> {
    sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_RECENT_LOGIN_FAILURE_COUNT_SQL)
        .bind(login.as_ref())
        .fetch_one(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(super::AdminRecentLoginFailureCount::from)
}

pub(crate) async fn find_sign_in_user(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    login: &server_admin_contract::domain_types::AdminLogin,
) -> Result<Option<super::AdminSignInUser>, crate::domain_types::SqlxAdminError> {
    sqlx::query_as::<_, (i64, String, bool)>(constants_str::SERVER_ADMIN_SIGN_IN_USER_SQL)
        .bind(login.as_ref())
        .fetch_optional(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .and_then(|value| {
            value
                .map(|(id, password_hash, is_banned)| {
                    Ok(super::AdminSignInUser {
                        id: crate::domain_types::AdminUserId::try_from(id)?,
                        password_hash: crate::domain_types::AdminPasswordHash::new(
                            pg_types_text_misc::StringAsNonNullTextSecret::from(password_hash),
                        ),
                        is_banned: crate::domain_types::StdAdminBool::from(is_banned),
                    })
                })
                .transpose()
        })
}

pub(crate) async fn lock_refresh_token_user(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    token_hash: &crate::domain_types::AdminTokenHash,
) -> Result<Option<crate::domain_types::AdminUserId>, crate::domain_types::SqlxAdminError> {
    sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_LOCK_REFRESH_TOKEN_USER_SQL)
        .bind(token_hash.expose().as_ref())
        .fetch_optional(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .and_then(|value| {
            value
                .map(crate::domain_types::AdminUserId::try_from)
                .transpose()
                .map_err(crate::domain_types::SqlxAdminError::from)
        })
}

pub(crate) async fn revoke_refresh_token(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    token_hash: &crate::domain_types::AdminTokenHash,
    user_id: crate::domain_types::AdminUserId,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_REFRESH_TOKEN_SQL)
        .bind(token_hash.expose().as_ref())
        .bind(user_id.get())
        .execute(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn update_user(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
    login: Option<&server_admin_contract::domain_types::AdminLogin>,
    display_name: Option<&server_admin_contract::domain_types::AdminDisplayName>,
) -> Result<crate::domain_types::StdAdminBool, crate::domain_types::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_SQL)
        .bind(user_id.get())
        .bind(login.map(|value| value.as_ref().as_str()))
        .bind(display_name.map(|value| value.as_ref().as_str()))
        .fetch_optional(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| crate::domain_types::StdAdminBool::from(value.is_some()))
}

pub(crate) async fn update_user_password(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
    password_hash: &crate::domain_types::AdminPasswordHash,
    password_change_required: crate::domain_types::AdminPasswordChangeRequired,
) -> Result<crate::domain_types::StdAdminBool, crate::domain_types::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_PASSWORD_SQL)
        .bind(user_id.get())
        .bind(password_hash.expose().as_ref())
        .bind(*password_change_required)
        .fetch_optional(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| crate::domain_types::StdAdminBool::from(value.is_some()))
}
pub(crate) async fn read_password_hash(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    user_id: crate::domain_types::AdminUserId,
) -> Result<Option<crate::domain_types::AdminPasswordHash>, crate::domain_types::SqlxAdminError> {
    sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_PASSWORD_HASH_SQL)
        .bind(user_id.get())
        .fetch_optional(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| {
            value.map(|hash| {
                crate::domain_types::AdminPasswordHash::new(
                    pg_types_text_misc::StringAsNonNullTextSecret::from(hash),
                )
            })
        })
}

pub(crate) async fn update_user_ban(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
    is_banned: crate::domain_types::StdAdminBool,
) -> Result<crate::domain_types::StdAdminBool, crate::domain_types::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_BAN_SQL)
        .bind(user_id.get())
        .bind(is_banned.get())
        .fetch_optional(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| crate::domain_types::StdAdminBool::from(value.is_some()))
}

pub(crate) async fn delete_user(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
) -> Result<crate::domain_types::StdAdminBool, crate::domain_types::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_DELETE_USER_SQL)
        .bind(user_id.get())
        .fetch_optional(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(|value| crate::domain_types::StdAdminBool::from(value.is_some()))
}

pub(crate) async fn list_users(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    query: &server_admin_contract::domain_types::AdminTableQuery,
) -> Result<
    (
        server_admin_contract::domain_types::AdminUserSummaries,
        super::AdminPageTotalCount,
    ),
    super::AdminRepositoryError,
> {
    let search = query.search().as_ref();
    let total = sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_FILTERED_USERS_SQL)
        .bind(search)
        .fetch_one(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?;
    let rows = sqlx::query_as::<_, (i64, String, String, bool)>(
        constants_str::SERVER_ADMIN_PAGE_USERS_SQL,
    )
    .bind(search)
    .bind(query.sort().as_ref())
    .bind(query.direction().as_ref())
    .bind(i64::from(u16::from(query.limit())))
    .bind(i64::from(u32::from(query.offset())))
    .fetch_all(pool.0)
    .await
    .map_err(crate::domain_types::SqlxAdminError::from)?;
    let user_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
    let links = sqlx::query_as::<_, (i64, i64)>(constants_str::SERVER_ADMIN_LIST_USER_ROLE_IDS_SQL)
        .bind(user_ids.as_slice())
        .fetch_all(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?;
    let mut role_ids_by_user = links.into_iter().try_fold(
        std::collections::HashMap::<i64, Vec<server_admin_contract::domain_types::AdminRoleId>>::with_capacity(
            user_ids.len(),
        ),
        |mut values, (user_id, role_id)| {
            values.entry(user_id).or_default().push(
                server_admin_contract::domain_types::AdminRoleId::try_from(role_id)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            );
            Ok::<_, super::AdminRepositoryError>(values)
        },
    )?;
    let items = rows
        .into_iter()
        .map(|(id, login, display_name, is_banned)| {
            Ok(server_admin_contract::domain_types::AdminUserSummary::new(
                server_admin_contract::domain_types::AdminDisplayName::try_from(display_name)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::domain_types::AdminUserId::try_from(id)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::domain_types::AdminBool::from(is_banned),
                server_admin_contract::domain_types::AdminLogin::try_from(login)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::domain_types::AdminRoleIds::try_from(
                    role_ids_by_user.remove(&id).unwrap_or_default(),
                )
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            ))
        })
        .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    Ok((
        server_admin_contract::domain_types::AdminUserSummaries::try_from(items)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        super::AdminPageTotalCount::from(total),
    ))
}

pub(crate) async fn read_authenticated_record(
    db: &mut super::AdminRepositoryDbRef<'_, '_>,
    user_id: crate::domain_types::AdminUserId,
) -> Result<Option<super::AdminAuthenticatedRecord>, super::AdminRepositoryError> {
    let user_query =
        sqlx::query_as::<_, (String, String, bool)>(constants_str::SERVER_ADMIN_READ_AUTH_USER_SQL)
            .bind(user_id.get());
    let optional_user = match db {
        super::AdminRepositoryDbRef::Connection(connection) => {
            user_query.fetch_optional(&mut *connection.0).await
        }
        super::AdminRepositoryDbRef::Pool(pool) => user_query.fetch_optional(pool.0).await,
    }
    .map_err(crate::domain_types::SqlxAdminError::from)?;
    let Some((login, display_name, must_change_password)) = optional_user else {
        return Ok(None);
    };
    let roles_query =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_AUTH_ROLES_SQL)
            .bind(user_id.get());
    let raw_roles = match db {
        super::AdminRepositoryDbRef::Connection(connection) => {
            roles_query.fetch_all(&mut *connection.0).await
        }
        super::AdminRepositoryDbRef::Pool(pool) => roles_query.fetch_all(pool.0).await,
    }
    .map_err(crate::domain_types::SqlxAdminError::from)?;
    let permissions_query =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_AUTH_PERMISSIONS_SQL)
            .bind(user_id.get());
    let raw_permissions = match db {
        super::AdminRepositoryDbRef::Connection(connection) => {
            permissions_query.fetch_all(&mut *connection.0).await
        }
        super::AdminRepositoryDbRef::Pool(pool) => permissions_query.fetch_all(pool.0).await,
    }
    .map_err(crate::domain_types::SqlxAdminError::from)?;
    Ok(Some(super::AdminAuthenticatedRecord {
        display_name: server_admin_contract::domain_types::AdminDisplayName::try_from(display_name)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        login: server_admin_contract::domain_types::AdminLogin::try_from(login)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        password_change_required: crate::domain_types::AdminPasswordChangeRequired::from(
            must_change_password,
        ),
        permissions: raw_permissions
            .into_iter()
            .map(|permission| {
                server_admin_contract::domain_types::AdminPermission::try_from(permission.as_str())
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?
            .try_into()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        roles: raw_roles
            .into_iter()
            .map(server_admin_contract::domain_types::AdminRoleName::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?
            .try_into()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
    }))
}
