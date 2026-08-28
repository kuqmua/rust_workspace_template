#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct CommandSucceeded(pub(super) bool);
impl CommandSucceeded {
    pub(super) const fn get(self) -> bool {
        self.0
    }
}
