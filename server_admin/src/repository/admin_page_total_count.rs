#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct AdminPageTotalCount(i64);

impl AdminPageTotalCount {
    pub(crate) const fn get(self) -> i64 {
        self.0
    }
}
