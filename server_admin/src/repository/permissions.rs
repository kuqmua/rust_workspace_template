#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

const LIST_PERMISSIONS: &str = "SELECT id, name FROM admin_permissions ORDER BY name";
const LOCK_ROLE_SYSTEM_STATE: &str = "SELECT is_system FROM admin_roles WHERE id = $1 FOR UPDATE";
const COUNT_PERMISSIONS: &str = "SELECT count(*) FROM admin_permissions WHERE id = ANY($1)";
const REPLACE_ROLE_PERMISSIONS_DELETE: &str =
    "DELETE FROM admin_role_permissions WHERE role_id = $1";
const REPLACE_ROLE_PERMISSIONS_INSERT: &str = "INSERT INTO admin_role_permissions (role_id, permission_id) SELECT $1, permission_id FROM unnest($2::bigint[]) AS permission_id";

pub(crate) async fn list_permissions(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
) -> Result<Vec<server_admin_contract::AdminPermissionSummary>, super::AdminRepositoryError> {
    sqlx::query_as::<_, (i64, String)>(LIST_PERMISSIONS)
        .fetch_all(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .into_iter()
        .map(|(id, name)| {
            Ok(server_admin_contract::AdminPermissionSummary::new(
                server_admin_contract::AdminPermissionId::from(id),
                server_admin_contract::AdminPermissionValue::try_from(name)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            ))
        })
        .collect()
}

pub(crate) async fn replace_role_permissions(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    role_id: crate::AdminRoleId,
    permission_ids: &[server_admin_contract::AdminPermissionId],
) -> Result<super::ReplaceRolePermissionsOutcome, crate::SqlxAdminError> {
    let optional_is_system = sqlx::query_scalar::<_, bool>(LOCK_ROLE_SYSTEM_STATE)
        .bind(role_id.0)
        .fetch_optional(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let Some(is_system) = optional_is_system else {
        return Ok(super::ReplaceRolePermissionsOutcome::MissingRole);
    };
    if is_system {
        return Ok(super::ReplaceRolePermissionsOutcome::SystemRole);
    }
    let raw_ids = permission_ids
        .iter()
        .copied()
        .map(i64::from)
        .collect::<Vec<_>>();
    let existing_count = sqlx::query_scalar::<_, i64>(COUNT_PERMISSIONS)
        .bind(&raw_ids)
        .fetch_one(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    if usize::try_from(existing_count).ok() != Some(raw_ids.len()) {
        return Ok(super::ReplaceRolePermissionsOutcome::UnknownPermission);
    }
    let _delete_result = sqlx::query(REPLACE_ROLE_PERMISSIONS_DELETE)
        .bind(role_id.0)
        .execute(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let _insert_result = sqlx::query(REPLACE_ROLE_PERMISSIONS_INSERT)
        .bind(role_id.0)
        .bind(&raw_ids)
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    Ok(super::ReplaceRolePermissionsOutcome::Updated)
}
