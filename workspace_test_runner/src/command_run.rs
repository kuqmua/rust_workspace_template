#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(super) struct CommandRun {
    pub(super) duration: crate::command_duration::CommandDuration,
    pub(super) idx: crate::command_idx::CommandIdx,
    pub(super) log_text: crate::command_text::CommandText,
    pub(super) status_text: crate::command_text::CommandText,
    pub(super) succeeded: crate::command_succeeded::CommandSucceeded,
}
