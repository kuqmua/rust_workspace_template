#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct AdminActiveAdministratorCount(i64);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(crate) struct LastAdminState {
    active_count: AdminActiveAdministratorCount,
    target_is_admin: crate::domain_types::StdAdminBool,
}
impl LastAdminState {
    pub(crate) fn would_remove_last(self) -> crate::domain_types::StdAdminBool {
        crate::domain_types::StdAdminBool::from(
            self.target_is_admin.get() && self.active_count.0 <= constants_i64::ONE,
        )
    }
}

pub(crate) async fn lock_last_admin(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_LOCK_LAST_ADMIN_SQL)
        .execute(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn read_last_admin_state(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
) -> Result<LastAdminState, crate::domain_types::SqlxAdminError> {
    let target_is_admin =
        sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_USER_IS_ADMIN_SQL)
            .bind(user_id.get())
            .fetch_one(&mut *connection.0)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)?;
    let active_count =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_ACTIVE_ADMIN_COUNT_SQL)
            .fetch_one(connection.0)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)?;
    Ok(LastAdminState {
        active_count: AdminActiveAdministratorCount::from(active_count),
        target_is_admin: crate::domain_types::StdAdminBool::from(target_is_admin),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn last_administrator_state_requires_admin_target_and_at_most_one_active_admin() {
        let would_remove = |active_count, target_is_admin| {
            super::LastAdminState {
                active_count: super::AdminActiveAdministratorCount::from(active_count),
                target_is_admin: crate::domain_types::StdAdminBool::from(target_is_admin),
            }
            .would_remove_last()
            .get()
        };
        assert!(would_remove(constants_i64::ONE, true));
        assert!(would_remove(constants_i64::ZERO, true));
        assert!(!would_remove(2i64, true));
        assert!(!would_remove(constants_i64::ONE, false));
    }
}
