#[derive(
    Debug,
    Default,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct CommandFailuresVecDeque(
    std::collections::VecDeque<crate::command_failure::CommandFailure>,
);
impl CommandFailuresVecDeque {
    pub(super) fn append(&mut self, command_failures_vec_deque: &mut Self) {
        self.0.append(&mut command_failures_vec_deque.0);
    }

    pub(super) fn push(&mut self, command_failure: crate::command_failure::CommandFailure) {
        self.0.push_back(command_failure);
    }
}
