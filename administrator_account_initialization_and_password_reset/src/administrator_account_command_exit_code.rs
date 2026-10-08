#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct AdministratorAccountCommandExitCode(std::process::ExitCode);

impl std::process::Termination for AdministratorAccountCommandExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_administrator_command_status_reports_every_exit_code_without_translation() {
        assert!((u8::MIN..=u8::MAX).all(|value| {
            let administrator_account_command_status =
                crate::administrator_account_command_status::AdministratorAccountCommandStatus::from(
                    value,
                );
            let administrator_account_command_exit_code =
                crate::administrator_account_command_exit_code::AdministratorAccountCommandExitCode::from(
                    std::process::ExitCode::from(u8::from(administrator_account_command_status)),
                );
            std::process::Termination::report(administrator_account_command_exit_code)
                == std::process::ExitCode::from(value)
        }));
    }
}
