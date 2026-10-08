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
        [
            (i64::MIN, true),
            (-1i64, true),
            (0i64, true),
            (1i64, true),
            (2i64, false),
            (i64::MAX, false),
        ]
        .into_iter()
        .fold((), |(), (active_count, expected)| {
            assert_eq!(would_remove(active_count, true), expected);
            assert!(!would_remove(active_count, false));
        });
    }
}
