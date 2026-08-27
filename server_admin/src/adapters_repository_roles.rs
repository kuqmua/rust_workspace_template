#[path = "adapters_repository_roles/admin_active_administrator_count.rs"]
mod admin_active_administrator_count;
#[path = "adapters_repository_roles/last_admin_state.rs"]
mod last_admin_state;
#[path = "adapters_repository_roles/lock_last_admin.rs"]
mod lock_last_admin;
#[path = "adapters_repository_roles/read_last_admin_state.rs"]
mod read_last_admin_state;

use admin_active_administrator_count::AdminActiveAdministratorCount;
pub(crate) use last_admin_state::LastAdminState;
pub(crate) use lock_last_admin::lock_last_admin;
pub(crate) use read_last_admin_state::read_last_admin_state;

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
