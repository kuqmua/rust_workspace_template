#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::FromInner)]
pub(crate) struct AdministratorAccountCommandExitCode(std::process::ExitCode);

impl std::process::Termination for AdministratorAccountCommandExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}
