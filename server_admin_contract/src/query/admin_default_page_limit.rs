use super::AdminPageLimit;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct AdminDefaultPageLimit;
impl From<AdminDefaultPageLimit> for AdminPageLimit {
    fn from(_value: AdminDefaultPageLimit) -> Self {
        Self(Self::DEFAULT)
    }
}
