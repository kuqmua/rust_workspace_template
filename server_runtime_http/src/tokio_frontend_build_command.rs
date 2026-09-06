#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct TokioFrontendBuildCommand(tokio::process::Command);

impl TokioFrontendBuildCommand {
    pub(crate) async fn run(
        mut self,
        frontend_build_step: crate::frontend_build_step::FrontendBuildStep,
    ) -> Result<(), crate::frontend_preparation_error::FrontendPreparationError> {
        let status = self.0.kill_on_drop(true).status().await.map_err(|source| {
            crate::frontend_preparation_error::FrontendPreparationError::Command {
                frontend_build_step,
                source: crate::service_runtime_io_error::ServiceRuntimeIoError::from(source),
            }
        })?;
        frontend_build_step
            .check_exit_status(crate::child_exit_status::ChildExitStatus::from(status))
    }

    pub(crate) async fn node_version(
        mut self,
    ) -> Result<
        crate::bounded_text::BoundedText,
        crate::frontend_preparation_error::FrontendPreparationError,
    > {
        let process_error =
            |source| crate::frontend_preparation_error::FrontendPreparationError::Command {
                frontend_build_step: crate::frontend_build_step::FrontendBuildStep::NodeVersion,
                source: crate::service_runtime_io_error::ServiceRuntimeIoError::from(source),
            };
        let mut child = self
            .0
            .kill_on_drop(true)
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(process_error)?;
        let Some(stdout) = child.stdout.take() else {
            child.kill().await.map_err(process_error)?;
            return Err(crate::frontend_preparation_error::FrontendPreparationError::NodeVersion);
        };
        let mut output = Vec::new();
        let read = tokio::io::AsyncReadExt::read_to_end(
            &mut tokio::io::AsyncReadExt::take(stdout, 128u64),
            &mut output,
        )
        .await;
        if read.is_err() || output.len() == 128usize {
            child.kill().await.map_err(process_error)?;
            read.map(|_bytes_read| ()).map_err(process_error)?;
            return Err(crate::frontend_preparation_error::FrontendPreparationError::NodeVersion);
        }
        let status = child.wait().await.map_err(process_error)?;
        crate::frontend_build_step::FrontendBuildStep::NodeVersion
            .check_exit_status(crate::child_exit_status::ChildExitStatus::from(status))?;
        crate::bounded_text::BoundedText::try_from(crate::bounded_bytes::BoundedBytes::from(output))
            .map_err(crate::frontend_preparation_error::FrontendPreparationError::Read)
    }
}
