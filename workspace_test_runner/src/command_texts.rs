#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::execution::CommandText;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct CommandTexts(
    pub(super) bounded_types::BoundedVec<CommandText, 0, { usize::MAX }>,
);
