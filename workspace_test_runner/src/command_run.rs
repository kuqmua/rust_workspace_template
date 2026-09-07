#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub(super) struct CommandRun {
    #[getters(get_mut)]
    command_failures_vec_deque: crate::command_failures_vec_deque::CommandFailuresVecDeque,
    command_index: crate::command_index::CommandIndex,
    duration: crate::command_duration::CommandDuration,
    log_text: crate::command_text::CommandText,
    status_text: crate::command_text::CommandText,
}
