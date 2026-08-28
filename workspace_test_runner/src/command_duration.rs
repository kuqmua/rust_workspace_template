#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::execution::CommandDurationMillis;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct CommandDuration(pub(super) std::time::Duration);
impl CommandDuration {
    pub(super) fn as_millis(self) -> CommandDurationMillis {
        CommandDurationMillis::from(self.0.as_millis())
    }
}
