#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub(crate) struct AdminRecentLoginFailureCount(i64);

impl AdminRecentLoginFailureCount {
    pub(crate) fn reached(
        self,
        threshold: crate::std_admin_failure_threshold::StdAdminFailureThreshold,
    ) -> server_admin_core::std_admin_bool::StdAdminBool {
        server_admin_core::std_admin_bool::StdAdminBool::from(*self.get_inner() >= threshold.get())
    }
}
