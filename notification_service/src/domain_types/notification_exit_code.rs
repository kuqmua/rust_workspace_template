#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct NotificationExitCode(std::process::ExitCode);
impl std::process::Termination for NotificationExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}
