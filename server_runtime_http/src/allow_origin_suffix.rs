#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct AllowOriginSuffix(bool);

impl AllowOriginSuffix {
    pub(crate) const fn get(self) -> bool {
        self.0
    }
}
