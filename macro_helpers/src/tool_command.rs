#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub struct ToolCommand {
    inner: crate::tool_process_command::ToolProcessCommand,
    program: crate::os_string_value::OsStringValue,
}
impl std::fmt::Debug for ToolCommand {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(constants_str::TOOLCOMMAND)
            .field(constants_str::PROGRAM, &*self.program)
            .field(constants_str::ARGUMENTS, &constants_str::REDACTED)
            .finish_non_exhaustive()
    }
}
impl ToolCommand {
    pub fn arg(&mut self, tool_arg_ref: crate::tool_arg_ref::ToolArgRef<'_>) -> &mut Self {
        let _command = self.inner.arg(*tool_arg_ref);
        self
    }
    pub fn args(&mut self, tool_args_ref: crate::tool_args_ref::ToolArgsRef<'_>) -> &mut Self {
        let _command = self.inner.args(*tool_args_ref);
        self
    }
    pub fn current_dir(
        &mut self,
        macro_path_ref: crate::macro_path_ref::MacroPathRef<'_>,
    ) -> &mut Self {
        let _command = self.inner.current_dir(*macro_path_ref);
        self
    }
    pub fn env(
        &mut self,
        tool_env_key_ref: crate::tool_env_key_ref::ToolEnvKeyRef<'_>,
        tool_env_value_ref: crate::tool_env_value_ref::ToolEnvValueRef<'_>,
    ) -> &mut Self {
        let _command = self.inner.env(*tool_env_key_ref, *tool_env_value_ref);
        self
    }
    #[must_use]
    pub fn new(tool_program_ref: crate::tool_program_ref::ToolProgramRef<'_>) -> Self {
        Self {
            inner: crate::tool_process_command::ToolProcessCommand::from(
                std::process::Command::new(*tool_program_ref),
            ),
            program: crate::os_string_value::OsStringValue::from(*tool_program_ref),
        }
    }
    pub fn output(&mut self) -> std::io::Result<crate::process_output::ProcessOutput> {
        self.inner
            .output()
            .map(crate::process_output::ProcessOutput::from)
    }
    pub fn bounded_output(
        &mut self,
        tool_output_limit: crate::tool_output_limit::ToolOutputLimit,
    ) -> std::io::Result<crate::process_output::ProcessOutput> {
        let mut child = self
            .inner
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        let mut stdout_pipe = child
            .stdout
            .take()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::BrokenPipe))?;
        let mut stderr_pipe = child
            .stderr
            .take()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::BrokenPipe))?;
        std::thread::scope(|scope| {
            let stdout_handle = scope.spawn(move || {
                let mut tail = crate::tool_output_tail::ToolOutputTail::new(tool_output_limit);
                let _copied_bytes = std::io::copy(&mut stdout_pipe, &mut tail)?;
                Ok::<Vec<u8>, std::io::Error>(tail.into_bytes())
            });
            let stderr_handle = scope.spawn(move || {
                let mut tail = crate::tool_output_tail::ToolOutputTail::new(tool_output_limit);
                let _copied_bytes = std::io::copy(&mut stderr_pipe, &mut tail)?;
                Ok::<Vec<u8>, std::io::Error>(tail.into_bytes())
            });
            let status = child.wait();
            let stdout_result = stdout_handle.join();
            let stderr_result = stderr_handle.join();
            let stdout_bytes = stdout_result.map_err(|panic| {
                std::io::Error::other(format!(
                    "{}{panic:?}",
                    constants_str::COMMAND_THREAD_PANICKED_SUMMARY
                ))
            })??;
            let stderr_bytes = stderr_result.map_err(|panic| {
                std::io::Error::other(format!(
                    "{}{panic:?}",
                    constants_str::COMMAND_THREAD_PANICKED_SUMMARY
                ))
            })??;
            Ok(crate::process_output::ProcessOutput::from(
                std::process::Output {
                    status: status?,
                    stdout: stdout_bytes,
                    stderr: stderr_bytes,
                },
            ))
        })
    }
    pub fn status(&mut self) -> std::io::Result<crate::process_exit_status::ProcessExitStatus> {
        self.inner
            .status()
            .map(crate::process_exit_status::ProcessExitStatus::from)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_bounded_output_retains_recent_process_bytes() {
        let mut command = super::ToolCommand::new(crate::tool_program_ref::ToolProgramRef::from(
            constants_str::PRINTF,
        ));
        let _command = command.arg(crate::tool_arg_ref::ToolArgRef::from(
            constants_str::SECRET_VALUE,
        ));
        let result = command.bounded_output(crate::tool_output_limit::ToolOutputLimit::from(
            constants_usize::THREE,
        ));
        assert!(result.is_ok_and(|output| {
            output.status.success()
                && constants_str::SECRET_VALUE
                    .as_bytes()
                    .get(constants_str::SECRET_VALUE.len() - constants_usize::THREE..)
                    .is_some_and(|expected| output.stdout.as_slice() == expected)
                && output.stderr.is_empty()
        }));
    }

    #[test]
    fn test_debug_redacts_arguments() {
        let mut command = super::ToolCommand::new(crate::tool_program_ref::ToolProgramRef::from(
            constants_str::PRINTF,
        ));
        let _command = command.arg(crate::tool_arg_ref::ToolArgRef::from(
            constants_str::SECRET_VALUE,
        ));
        let debug = format!("{command:?}");
        assert!(debug.contains(constants_str::PRINTF));
        assert!(debug.contains(constants_str::REDACTED));
        assert!(!debug.contains(constants_str::SECRET_VALUE));
    }
}
