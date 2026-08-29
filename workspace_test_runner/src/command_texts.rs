#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct CommandTexts(
    bounded_types::bounded_vec::BoundedVec<crate::command_text::CommandText, 0, { usize::MAX }>,
);

impl CommandTexts {
    pub(super) fn as_ref(&self) -> &[crate::command_text::CommandText] {
        self.0.as_ref()
    }
}
