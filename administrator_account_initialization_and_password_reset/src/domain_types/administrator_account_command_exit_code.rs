#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(crate) struct AdministratorAccountCommandExitCode(std::process::ExitCode);

impl std::process::Termination for AdministratorAccountCommandExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}
