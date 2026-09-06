#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum FrontendBuildStep {
    BrowserAssets,
    Dependencies,
    NodeVersion,
    WasmTarget,
}

impl FrontendBuildStep {
    pub(crate) fn check_exit_status(
        self,
        child_exit_status: crate::child_exit_status::ChildExitStatus,
    ) -> Result<(), crate::frontend_preparation_error::FrontendPreparationError> {
        match child_exit_status.succeeded() {
            crate::child_process_succeeded::ChildProcessSucceeded::Yes => Ok(()),
            crate::child_process_succeeded::ChildProcessSucceeded::No => Err(
                crate::frontend_preparation_error::FrontendPreparationError::Failed {
                    frontend_build_step: self,
                    child_exit_status,
                },
            ),
        }
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "each deterministic exit-status fixture is asserted separately while repository policy forbids for loops"
    )]
    fn test_frontend_preparation_rejects_each_failed_build_step() {
        [
            crate::frontend_build_step::FrontendBuildStep::BrowserAssets,
            crate::frontend_build_step::FrontendBuildStep::Dependencies,
            crate::frontend_build_step::FrontendBuildStep::NodeVersion,
            crate::frontend_build_step::FrontendBuildStep::WasmTarget,
        ]
        .into_iter()
        .for_each(|step| {
            assert!(matches!(step.check_exit_status(crate::child_exit_status::ChildExitStatus::from(
                <std::process::ExitStatus as std::os::unix::process::ExitStatusExt>::from_raw(0),
            )), Ok(())));
            assert!(matches!(
                step.check_exit_status(crate::child_exit_status::ChildExitStatus::from(
                    <std::process::ExitStatus as std::os::unix::process::ExitStatusExt>::from_raw(256),
                )),
                Err(crate::frontend_preparation_error::FrontendPreparationError::Failed {
                    frontend_build_step, ..
                }) if frontend_build_step == step
            ));
        });
    }
}
