#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct AdminDefaultPageLimit;
impl From<AdminDefaultPageLimit> for crate::admin_page_limit::AdminPageLimit {
    fn from(_value: AdminDefaultPageLimit) -> Self {
        Self(Self::DEFAULT)
    }
}
