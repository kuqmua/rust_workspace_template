use super::{CommandDuration, CommandIdx, CommandSucceeded, CommandText};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(super) struct CommandRun {
    pub(super) duration: CommandDuration,
    pub(super) idx: CommandIdx,
    pub(super) log_text: CommandText,
    pub(super) status_text: CommandText,
    pub(super) succeeded: CommandSucceeded,
}
