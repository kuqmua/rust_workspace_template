#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct AdminRecentLoginFailureCount(i64);

impl AdminRecentLoginFailureCount {
    pub(crate) fn reached(
        self,
        threshold: crate::domain_types::auth::StdAdminFailureThreshold,
    ) -> crate::domain_types::StdAdminBool {
        crate::domain_types::StdAdminBool::from(self.0 >= threshold.get())
    }
}
