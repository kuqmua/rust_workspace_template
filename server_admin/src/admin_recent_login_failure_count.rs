#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct AdminRecentLoginFailureCount(i64);

impl AdminRecentLoginFailureCount {
    pub(crate) fn reached(
        self,
        threshold: crate::std_admin_failure_threshold::StdAdminFailureThreshold,
    ) -> server_admin_core::std_admin_bool::StdAdminBool {
        server_admin_core::std_admin_bool::StdAdminBool::from(self.0 >= threshold.get())
    }
}
