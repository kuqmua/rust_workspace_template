#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(super) struct FilterPlaceholderCount(usize);

impl FilterPlaceholderCount {
    pub(super) const fn get(self) -> usize {
        self.0
    }

    pub(super) fn one() -> Self {
        Self::from(constants_usize::ONE)
    }
}
