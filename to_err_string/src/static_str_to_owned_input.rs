#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct StaticStrToOwnedInput(&'static str);

impl StaticStrToOwnedInput {
    pub(super) const fn get(self) -> &'static str {
        self.0
    }
}
