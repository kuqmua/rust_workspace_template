#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct ChildExitStatus(std::process::ExitStatus);

impl ChildExitStatus {
    #[must_use]
    pub fn succeeded(self) -> crate::child_process_succeeded::ChildProcessSucceeded {
        if self.0.success() {
            crate::child_process_succeeded::ChildProcessSucceeded::Yes
        } else {
            crate::child_process_succeeded::ChildProcessSucceeded::No
        }
    }
}
