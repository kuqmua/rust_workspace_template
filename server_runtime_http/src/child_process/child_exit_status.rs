#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct ChildExitStatus(std::process::ExitStatus);

impl ChildExitStatus {
    #[must_use]
    pub fn succeeded(self) -> super::ChildProcessSucceeded {
        if self.0.success() {
            super::ChildProcessSucceeded::Yes
        } else {
            super::ChildProcessSucceeded::No
        }
    }
}
