#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_getters::Getters,
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
