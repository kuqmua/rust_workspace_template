#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct CommandIdx(pub(super) usize);
impl CommandIdx {
    pub(super) const fn get(self) -> usize {
        self.0
    }
}
