#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::execution::{CommandDuration, CommandIdx, CommandSucceeded, CommandText};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(super) struct CommandRun {
    pub(super) duration: CommandDuration,
    pub(super) idx: CommandIdx,
    pub(super) log_text: CommandText,
    pub(super) status_text: CommandText,
    pub(super) succeeded: CommandSucceeded,
}
