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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_frontend_command_spawn_errors_preserve_build_step_and_source() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(constants_str::CARGO_TOML)
            .join(constants_str::X);
        let make_command = || {
            crate::tokio_frontend_build_command::TokioFrontendBuildCommand::from(
                tokio::process::Command::new(&path),
            )
        };
        let result = make_command()
            .run(crate::frontend_build_step::FrontendBuildStep::WasmTarget)
            .await;
        assert!(result.is_err_and(|error| {
            std::error::Error::source(&error).is_some()
                && matches!(
                    error,
                    crate::frontend_preparation_error::FrontendPreparationError::Command {
                        frontend_build_step:
                            crate::frontend_build_step::FrontendBuildStep::WasmTarget,
                        ..
                    }
                )
        }));
        let node_version_result = make_command().node_version().await;
        assert!(node_version_result.is_err_and(|error| {
            std::error::Error::source(&error).is_some()
                && matches!(
                    error,
                    crate::frontend_preparation_error::FrontendPreparationError::Command {
                        frontend_build_step:
                            crate::frontend_build_step::FrontendBuildStep::NodeVersion,
                        ..
                    }
                )
        }));
    }
    #[tokio::test]
    #[ignore = "requires printf, true, and false executables on PATH; run in a provisioned process environment"]
    async fn test_frontend_commands_preserve_status_and_bound_node_output() {
        let make_output_command = |bounded_text: &crate::bounded_text::BoundedText| {
            let mut command = tokio::process::Command::new(constants_str::PRINTF);
            let _configured_command = command.arg(bounded_text.as_ref());
            crate::tokio_frontend_build_command::TokioFrontendBuildCommand::from(command)
        };
        let valid_text_result =
            crate::bounded_text::BoundedText::try_from(constants_str::X.repeat(127usize));
        assert!(valid_text_result.is_ok());
        let Ok(valid_text) = valid_text_result else {
            return;
        };
        let output = make_output_command(&valid_text).node_version().await;
        assert!(output.is_ok_and(|bounded_text| bounded_text == valid_text));
        let long_text_result =
            crate::bounded_text::BoundedText::try_from(constants_str::X.repeat(128usize));
        assert!(long_text_result.is_ok());
        let Ok(long_text) = long_text_result else {
            return;
        };
        assert!(matches!(
            make_output_command(&long_text).node_version().await,
            Err(crate::frontend_preparation_error::FrontendPreparationError::NodeVersion)
        ));
        let unicode_text_result = crate::bounded_text::BoundedText::try_from(
            [
                char::from(u8::MAX).to_string().repeat(63usize),
                constants_str::X.to_owned(),
            ]
            .concat(),
        );
        assert!(
            unicode_text_result
                .as_ref()
                .is_ok_and(|bounded_text| bounded_text.as_ref().len() == 127usize)
        );
        let Ok(unicode_text) = unicode_text_result else {
            return;
        };
        assert!(
            make_output_command(&unicode_text)
                .node_version()
                .await
                .is_ok_and(|bounded_text| bounded_text == unicode_text)
        );
        let unicode_limit_result = crate::bounded_text::BoundedText::try_from(
            char::from(u8::MAX).to_string().repeat(64usize),
        );
        assert!(
            unicode_limit_result
                .as_ref()
                .is_ok_and(|bounded_text| bounded_text.as_ref().len() == 128usize)
        );
        let Ok(unicode_limit) = unicode_limit_result else {
            return;
        };
        assert!(matches!(
            make_output_command(&unicode_limit).node_version().await,
            Err(crate::frontend_preparation_error::FrontendPreparationError::NodeVersion)
        ));
        let success = crate::tokio_frontend_build_command::TokioFrontendBuildCommand::from(
            tokio::process::Command::new(constants_str::TRUE),
        )
        .run(crate::frontend_build_step::FrontendBuildStep::Dependencies)
        .await;
        assert!(matches!(success, Ok(())));
        let failure = crate::tokio_frontend_build_command::TokioFrontendBuildCommand::from(
            tokio::process::Command::new(constants_str::FALSE),
        )
        .run(crate::frontend_build_step::FrontendBuildStep::BrowserAssets)
        .await;
        assert!(
            matches!(failure, Err(crate::frontend_preparation_error::FrontendPreparationError::Failed { frontend_build_step: crate::frontend_build_step::FrontendBuildStep::BrowserAssets, child_exit_status }) if child_exit_status.succeeded() == crate::child_process_succeeded::ChildProcessSucceeded::No)
        );
        let node_failure = crate::tokio_frontend_build_command::TokioFrontendBuildCommand::from(
            tokio::process::Command::new(constants_str::FALSE),
        )
        .node_version()
        .await;
        assert!(
            matches!(node_failure, Err(crate::frontend_preparation_error::FrontendPreparationError::Failed { frontend_build_step: crate::frontend_build_step::FrontendBuildStep::NodeVersion, child_exit_status }) if child_exit_status.succeeded() == crate::child_process_succeeded::ChildProcessSucceeded::No)
        );
    }
    #[cfg(unix)]
    #[tokio::test]
    #[ignore = "requires printf and true executables on PATH; run in a provisioned process environment"]
    async fn test_frontend_node_output_preserves_utf8_error_and_accepts_empty_output() {
        let argument =
            <std::ffi::OsString as std::os::unix::ffi::OsStringExt>::from_vec(vec![u8::MAX]);
        let mut command = tokio::process::Command::new(constants_str::PRINTF);
        let _configured_command = command.arg(argument);
        let result = crate::tokio_frontend_build_command::TokioFrontendBuildCommand::from(command)
            .node_version()
            .await;
        assert!(result.is_err_and(|error| matches!(
            error,
            crate::frontend_preparation_error::FrontendPreparationError::Read(
                crate::bounded_read_error::BoundedReadError::Utf8 { .. }
            )
        ) && std::error::Error::source(&error).is_some()));
        let empty_output = crate::tokio_frontend_build_command::TokioFrontendBuildCommand::from(
            tokio::process::Command::new(constants_str::TRUE),
        )
        .node_version()
        .await;
        assert!(
            empty_output.is_ok_and(|bounded_text| bounded_text.as_ref() == constants_str::EMPTY)
        );
    }
}
