#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub(super) struct CommandRun {
    command_index: crate::command_idx::CommandIdx,
    duration: crate::command_duration::CommandDuration,
    log_text: crate::command_text::CommandText,
    status_text: crate::command_text::CommandText,
    succeeded: crate::command_succeeded::CommandSucceeded,
}
