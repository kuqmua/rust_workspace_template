#[cfg(test)]
mod tests {
    #[test]
    fn test_last_administrator_state_requires_admin_target_and_at_most_one_active_admin() {
        let would_remove = |active_count, target_is_admin| {
            crate::last_admin_state::LastAdminState::new(
                crate::admin_active_administrator_count::AdminActiveAdministratorCount::from(
                    active_count,
                ),
                server_admin_core::std_admin_bool::StdAdminBool::from(target_is_admin),
            )
            .would_remove_last()
            .get()
        };
        assert!(would_remove(constants_i64::ONE, true));
        assert!(would_remove(constants_i64::ZERO, true));
        assert!(!would_remove(2i64, true));
        assert!(!would_remove(constants_i64::ONE, false));
    }
}
