#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(crate) struct LastAdminState {
    pub(crate) active_count: crate::admin_active_administrator_count::AdminActiveAdministratorCount,
    pub(crate) target_is_admin: server_admin_core::std_admin_bool::StdAdminBool,
}

impl LastAdminState {
    pub(crate) fn would_remove_last(self) -> server_admin_core::std_admin_bool::StdAdminBool {
        server_admin_core::std_admin_bool::StdAdminBool::from(
            self.target_is_admin.get() && self.active_count.0 <= constants_i64::ONE,
        )
    }
}
