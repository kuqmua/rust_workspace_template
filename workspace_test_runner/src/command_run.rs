#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    generate_accessor::Getters,
    generate_constructor::New,
)]
pub(super) struct CommandRun {
    command_index: crate::command_idx::CommandIdx,
    duration: crate::command_duration::CommandDuration,
    log_text: crate::command_text::CommandText,
    status_text: crate::command_text::CommandText,
    succeeded: crate::command_succeeded::CommandSucceeded,
}
