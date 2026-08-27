#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::CommandDuration;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct CommandStartedAtInstant(pub(super) std::time::Instant);
impl CommandStartedAtInstant {
    pub(super) fn elapsed(self) -> CommandDuration {
        CommandDuration::from(self.0.elapsed())
    }
}
