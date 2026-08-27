#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(crate) struct LastAdminState {
    pub(super) active_count: super::AdminActiveAdministratorCount,
    pub(super) target_is_admin: crate::domain_types::StdAdminBool,
}

impl LastAdminState {
    pub(crate) fn would_remove_last(self) -> crate::domain_types::StdAdminBool {
        crate::domain_types::StdAdminBool::from(
            self.target_is_admin.get() && self.active_count.0 <= constants_i64::ONE,
        )
    }
}
