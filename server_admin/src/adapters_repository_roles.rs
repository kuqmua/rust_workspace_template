use admin_active_administrator_count::AdminActiveAdministratorCount;
pub(crate) use last_admin_state::LastAdminState;
pub(crate) use lock_last_admin::lock_last_admin;
pub(crate) use read_last_admin_state::read_last_admin_state;

#[cfg(test)]
mod tests {
    #[test]
    fn last_administrator_state_requires_admin_target_and_at_most_one_active_admin() {
        let would_remove = |active_count, target_is_admin| {
            crate::LastAdminState {
                active_count: crate::AdminActiveAdministratorCount::from(active_count),
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

// Root-owned module compatibility wrappers.
mod admin_active_administrator_count {
    pub use super::super::admin_active_administrator_count::*;
}
mod last_admin_state {
    pub use super::super::last_admin_state::*;
}
mod lock_last_admin {
    pub use super::super::lock_last_admin::*;
}
mod read_last_admin_state {
    pub use super::super::read_last_admin_state::*;
}
