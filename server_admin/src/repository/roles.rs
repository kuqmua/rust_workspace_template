#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

const INSERT_ROLE: &str =
    "INSERT INTO admin_roles (name, is_system) VALUES ($1, false) RETURNING id";
const UPDATE_ROLE: &str =
    "UPDATE admin_roles SET name = $2 WHERE id = $1 AND is_system = false RETURNING true";
const DELETE_ROLE: &str =
    "DELETE FROM admin_roles WHERE id = $1 AND is_system = false RETURNING true";
const LIST_ROLES: &str = "SELECT id, name, is_system FROM admin_roles ORDER BY name";
const LOCK_USER_ACTIVE_STATE: &str =
    "SELECT NOT is_banned FROM admin_users WHERE id = $1 FOR UPDATE";
const COUNT_ROLES: &str = "SELECT count(*) FROM admin_roles WHERE id = ANY($1)";
const READ_ADMIN_ROLE_ID: &str =
    "SELECT id FROM admin_roles WHERE name = 'admin' AND is_system = true";
const USER_HAS_ROLE: &str =
    "SELECT EXISTS (SELECT 1 FROM admin_user_roles WHERE user_id = $1 AND role_id = $2)";
const ACTIVE_ROLE_USER_COUNT: &str = "SELECT count(DISTINCT users.id) FROM admin_users users JOIN admin_user_roles user_role ON user_role.user_id = users.id WHERE user_role.role_id = $1 AND users.is_banned = false";
const REPLACE_USER_ROLES_DELETE: &str = "DELETE FROM admin_user_roles WHERE user_id = $1";
const REPLACE_USER_ROLES_INSERT: &str = "INSERT INTO admin_user_roles (user_id, role_id) SELECT $1, role_id FROM unnest($2::bigint[]) AS role_id";
const LOCK_LAST_ADMIN: &str =
    "SELECT pg_advisory_xact_lock(hashtext('admin_last_active_administrator'))";
const USER_IS_ADMIN: &str = "SELECT EXISTS (SELECT 1 FROM admin_user_roles user_role JOIN admin_roles role ON role.id = user_role.role_id WHERE user_role.user_id = $1 AND role.name = 'admin')";
const ACTIVE_ADMIN_COUNT: &str = "SELECT count(DISTINCT users.id) FROM admin_users users JOIN admin_user_roles user_role ON user_role.user_id = users.id JOIN admin_roles role ON role.id = user_role.role_id WHERE role.name = 'admin' AND users.is_banned = false";

#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct AdminActiveAdministratorCount(i64);

#[derive(Clone, Copy, Debug)]
pub(crate) struct LastAdminState {
    active_count: AdminActiveAdministratorCount,
    target_is_admin: crate::StdAdminBool,
}
impl LastAdminState {
    pub(crate) fn would_remove_last(self) -> crate::StdAdminBool {
        crate::StdAdminBool::from(self.target_is_admin.0 && self.active_count.0 <= 1i64)
    }
}

pub(crate) async fn insert_role(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    name: &server_admin_contract::AdminRoleName,
) -> Result<crate::AdminRoleId, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, i64>(INSERT_ROLE)
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
    sqlx::query_scalar::<_, bool>(UPDATE_ROLE)
        .bind(role_id.0)
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
    sqlx::query_scalar::<_, bool>(DELETE_ROLE)
        .bind(role_id.0)
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}

pub(crate) async fn list_roles(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
) -> Result<Vec<server_admin_contract::AdminRoleSummary>, super::AdminRepositoryError> {
    sqlx::query_as::<_, (i64, String, bool)>(LIST_ROLES)
        .fetch_all(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .into_iter()
        .map(|(id, name, is_system)| {
            Ok(server_admin_contract::AdminRoleSummary::new(
                server_admin_contract::AdminRoleId::from(id),
                server_admin_contract::AdminBool::from(is_system),
                server_admin_contract::AdminRoleName::try_from(name)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            ))
        })
        .collect()
}

pub(crate) async fn replace_user_roles(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    role_ids: &[server_admin_contract::AdminRoleId],
) -> Result<super::ReplaceUserRolesOutcome, crate::SqlxAdminError> {
    lock_last_admin(super::SqlxAdminRepositoryConnectionMutRef::from(
        &mut *connection.0,
    ))
    .await?;
    let optional_target_is_active = sqlx::query_scalar::<_, bool>(LOCK_USER_ACTIVE_STATE)
        .bind(user_id.0)
        .fetch_optional(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let Some(target_is_active) = optional_target_is_active else {
        return Ok(super::ReplaceUserRolesOutcome::MissingUser);
    };
    let raw_ids = role_ids.iter().copied().map(i64::from).collect::<Vec<_>>();
    let existing_count = sqlx::query_scalar::<_, i64>(COUNT_ROLES)
        .bind(&raw_ids)
        .fetch_one(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    if usize::try_from(existing_count).ok() != Some(raw_ids.len()) {
        return Ok(super::ReplaceUserRolesOutcome::UnknownRole);
    }
    let admin_role_id = sqlx::query_scalar::<_, i64>(READ_ADMIN_ROLE_ID)
        .fetch_one(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let target_was_admin = sqlx::query_scalar::<_, bool>(USER_HAS_ROLE)
        .bind(user_id.0)
        .bind(admin_role_id)
        .fetch_one(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    if target_is_active && target_was_admin && !raw_ids.contains(&admin_role_id) {
        let active_admin_count = sqlx::query_scalar::<_, i64>(ACTIVE_ROLE_USER_COUNT)
            .bind(admin_role_id)
            .fetch_one(&mut *connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
        if active_admin_count <= 1i64 {
            return Ok(super::ReplaceUserRolesOutcome::LastActiveAdministrator);
        }
    }
    let _delete_result = sqlx::query(REPLACE_USER_ROLES_DELETE)
        .bind(user_id.0)
        .execute(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let _insert_result = sqlx::query(REPLACE_USER_ROLES_INSERT)
        .bind(user_id.0)
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
    sqlx::query(LOCK_LAST_ADMIN)
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn read_last_admin_state(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<LastAdminState, crate::SqlxAdminError> {
    let target_is_admin = sqlx::query_scalar::<_, bool>(USER_IS_ADMIN)
        .bind(user_id.0)
        .fetch_one(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let active_count = sqlx::query_scalar::<_, i64>(ACTIVE_ADMIN_COUNT)
        .fetch_one(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    Ok(LastAdminState {
        active_count: AdminActiveAdministratorCount::from(active_count),
        target_is_admin: crate::StdAdminBool::from(target_is_admin),
    })
}
