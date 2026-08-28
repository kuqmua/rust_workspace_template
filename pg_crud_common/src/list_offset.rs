#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListOffset(pub(super) i64);

impl From<crate::domain_types::PaginationOffset> for ListOffset {
    fn from(value: crate::domain_types::PaginationOffset) -> Self {
        Self(value.get())
    }
}
