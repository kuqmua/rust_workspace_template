#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct CommandStartedAtInstant(std::time::Instant);
impl CommandStartedAtInstant {
    pub(super) fn elapsed(self) -> crate::command_duration::CommandDuration {
        crate::command_duration::CommandDuration::from(self.0.elapsed())
    }
}
