#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub(super) struct CommandDuration(std::time::Duration);
impl CommandDuration {
    pub(super) fn as_millis(self) -> crate::command_duration_millis::CommandDurationMillis {
        crate::command_duration_millis::CommandDurationMillis::from(self.0.as_millis())
    }
}
