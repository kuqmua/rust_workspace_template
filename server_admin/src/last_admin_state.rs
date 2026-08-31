#[derive(
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    generate_accessor::Getters,
)]
pub(crate) struct LastAdminState {
    active_count: crate::admin_active_administrator_count::AdminActiveAdministratorCount,
    target_is_admin: server_admin_core::std_admin_bool::StdAdminBool,
}

impl LastAdminState {
    pub(crate) fn would_remove_last(self) -> server_admin_core::std_admin_bool::StdAdminBool {
        server_admin_core::std_admin_bool::StdAdminBool::from(
            self.target_is_admin.get() && *self.active_count.get_inner() <= constants_i64::ONE,
        )
    }
}
