#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

pub(crate) async fn insert_user(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    login: &server_admin_contract::AdminLogin,
    display_name: &server_admin_contract::AdminDisplayName,
    password_hash: &crate::AdminPasswordHash,
) -> Result<crate::AdminUserId, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_INSERT_USER_SQL)
        .bind(login.as_ref())
        .bind(display_name.as_ref())
        .bind(password_hash.0.as_ref())
        .fetch_one(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(crate::AdminUserId::from)
}

pub(crate) async fn recent_login_failure_count(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    login: &server_admin_contract::AdminLogin,
) -> Result<super::AdminRecentLoginFailureCount, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_RECENT_LOGIN_FAILURE_COUNT_SQL)
        .bind(login.as_ref())
        .fetch_one(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(super::AdminRecentLoginFailureCount::from)
}

pub(crate) async fn find_sign_in_user(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    login: &server_admin_contract::AdminLogin,
) -> Result<Option<super::AdminSignInUser>, crate::SqlxAdminError> {
    sqlx::query_as::<_, (i64, String, bool)>(str_constants::SERVER_ADMIN_SIGN_IN_USER_SQL)
        .bind(login.as_ref())
        .fetch_optional(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| {
            value.map(|(id, password_hash, is_banned)| super::AdminSignInUser {
                id: crate::AdminUserId::from(id),
                password_hash: crate::AdminPasswordHash::new(
                    pg_types_text_misc::StringAsNonNullTextSecret::from(password_hash),
                ),
                is_banned: crate::StdAdminBool::from(is_banned),
            })
        })
}

pub(crate) async fn lock_refresh_token_user(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    token_hash: &crate::AdminTokenHash,
) -> Result<Option<crate::AdminUserId>, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_LOCK_REFRESH_TOKEN_USER_SQL)
        .bind(secrecy::ExposeSecret::expose_secret(token_hash.0.as_ref()))
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| value.map(crate::AdminUserId::from))
}

pub(crate) async fn revoke_refresh_token(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    token_hash: &crate::AdminTokenHash,
    user_id: crate::AdminUserId,
) -> Result<(), crate::SqlxAdminError> {
    sqlx::query(str_constants::SERVER_ADMIN_REVOKE_REFRESH_TOKEN_SQL)
        .bind(secrecy::ExposeSecret::expose_secret(token_hash.0.as_ref()))
        .bind(user_id.get())
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn update_user(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    login: Option<&server_admin_contract::AdminLogin>,
    display_name: Option<&server_admin_contract::AdminDisplayName>,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_UPDATE_USER_SQL)
        .bind(user_id.get())
        .bind(login.map(|value| value.as_ref().as_str()))
        .bind(display_name.map(|value| value.as_ref().as_str()))
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}

pub(crate) async fn update_user_password(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    password_hash: &crate::AdminPasswordHash,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_UPDATE_USER_PASSWORD_SQL)
        .bind(user_id.get())
        .bind(password_hash.0.as_ref())
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}
pub(crate) async fn read_password_hash(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<Option<crate::AdminPasswordHash>, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, String>(str_constants::SERVER_ADMIN_READ_PASSWORD_HASH_SQL)
        .bind(user_id.get())
        .fetch_optional(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| {
            value.map(|hash| {
                crate::AdminPasswordHash::new(pg_types_text_misc::StringAsNonNullTextSecret::from(
                    hash,
                ))
            })
        })
}

pub(crate) async fn update_user_ban(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    is_banned: crate::StdAdminBool,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_UPDATE_USER_BAN_SQL)
        .bind(user_id.get())
        .bind(is_banned.get())
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}

pub(crate) async fn delete_user(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_DELETE_USER_SQL)
        .bind(user_id.get())
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}

pub(crate) async fn list_users(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    query: &server_admin_contract::AdminTableQuery,
) -> Result<
    (
        server_admin_contract::AdminUserSummaries,
        super::AdminPageTotalCount,
    ),
    super::AdminRepositoryError,
> {
    let search = query.search().as_ref();
    let total = sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_COUNT_FILTERED_USERS_SQL)
        .bind(search)
        .fetch_one(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let rows = sqlx::query_as::<_, (i64, String, String, bool)>(
        str_constants::SERVER_ADMIN_PAGE_USERS_SQL,
    )
    .bind(search)
    .bind(query.sort().as_ref())
    .bind(query.direction().as_ref())
    .bind(i64::from(u16::from(query.limit())))
    .bind(i64::from(u32::from(query.offset())))
    .fetch_all(pool.0)
    .await
    .map_err(crate::SqlxAdminError::from)?;
    let user_ids = rows.iter().map(|row| row.0).collect::<Vec<_>>();
    let links = sqlx::query_as::<_, (i64, i64)>(str_constants::SERVER_ADMIN_LIST_USER_ROLE_IDS_SQL)
        .bind(user_ids.as_slice())
        .fetch_all(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let mut role_ids_by_user = links.into_iter().fold(
        std::collections::HashMap::<i64, Vec<server_admin_contract::AdminRoleId>>::new(),
        |mut values, (user_id, role_id)| {
            values
                .entry(user_id)
                .or_default()
                .push(server_admin_contract::AdminRoleId::from(role_id));
            values
        },
    );
    let items = rows
        .into_iter()
        .map(|(id, login, display_name, is_banned)| {
            Ok(server_admin_contract::AdminUserSummary::new(
                server_admin_contract::AdminDisplayName::try_from(display_name)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::AdminUserId::from(id),
                server_admin_contract::AdminBool::from(is_banned),
                server_admin_contract::AdminLogin::try_from(login)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::AdminRoleIds::try_from(
                    role_ids_by_user.remove(&id).unwrap_or_default(),
                )
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            ))
        })
        .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    Ok((
        server_admin_contract::AdminUserSummaries::try_from(items)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        super::AdminPageTotalCount::from(total),
    ))
}

pub(crate) async fn read_authenticated_record(
    db: &mut super::AdminRepositoryDbRef<'_, '_>,
    user_id: crate::AdminUserId,
) -> Result<Option<super::AdminAuthenticatedRecord>, super::AdminRepositoryError> {
    let user_query =
        sqlx::query_as::<_, (String, String)>(str_constants::SERVER_ADMIN_READ_AUTH_USER_SQL)
            .bind(user_id.get());
    let optional_user = match db {
        super::AdminRepositoryDbRef::Connection(connection) => {
            user_query.fetch_optional(&mut *connection.0).await
        }
        super::AdminRepositoryDbRef::Pool(pool) => user_query.fetch_optional(pool.0).await,
    }
    .map_err(crate::SqlxAdminError::from)?;
    let Some((login, display_name)) = optional_user else {
        return Ok(None);
    };
    let roles_query =
        sqlx::query_scalar::<_, String>(str_constants::SERVER_ADMIN_READ_AUTH_ROLES_SQL)
            .bind(user_id.get());
    let raw_roles = match db {
        super::AdminRepositoryDbRef::Connection(connection) => {
            roles_query.fetch_all(&mut *connection.0).await
        }
        super::AdminRepositoryDbRef::Pool(pool) => roles_query.fetch_all(pool.0).await,
    }
    .map_err(crate::SqlxAdminError::from)?;
    let permissions_query =
        sqlx::query_scalar::<_, String>(str_constants::SERVER_ADMIN_READ_AUTH_PERMISSIONS_SQL)
            .bind(user_id.get());
    let raw_permissions = match db {
        super::AdminRepositoryDbRef::Connection(connection) => {
            permissions_query.fetch_all(&mut *connection.0).await
        }
        super::AdminRepositoryDbRef::Pool(pool) => permissions_query.fetch_all(pool.0).await,
    }
    .map_err(crate::SqlxAdminError::from)?;
    Ok(Some(super::AdminAuthenticatedRecord {
        display_name: server_admin_contract::AdminDisplayName::try_from(display_name)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        login: server_admin_contract::AdminLogin::try_from(login)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        permissions: raw_permissions
            .into_iter()
            .map(|permission| server_admin_contract::AdminPermission::try_from(permission.as_str()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?
            .try_into()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        roles: raw_roles
            .into_iter()
            .map(server_admin_contract::AdminRoleName::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?
            .try_into()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
    }))
}
