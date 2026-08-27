#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct SnakeIdentifierifierLen(pub(super) usize);
impl From<usize> for SnakeIdentifierifierLen {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
