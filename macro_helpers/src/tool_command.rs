pub use super::path_ref::PathRef;
pub use super::process_exit_status::ProcessExitStatus;
pub use super::process_output::ProcessOutput;
pub use super::tool_arg_ref::ToolArgRef;
pub use super::tool_args_ref::ToolArgsRef;
pub use super::tool_env_key_ref::ToolEnvKeyRef;
pub use super::tool_env_value_ref::ToolEnvValueRef;
pub use super::tool_program_ref::ToolProgramRef;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub struct ToolCommand {
    inner: crate::process_command::ProcessCommand,
    program: crate::os_string_value::OsStringValue,
}
impl std::fmt::Debug for ToolCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(constants_str::TOOLCOMMAND)
            .field(constants_str::PROGRAM, &*self.program)
            .field(constants_str::ARGUMENTS, &constants_str::REDACTED)
            .finish_non_exhaustive()
    }
}
impl ToolCommand {
    pub fn arg(&mut self, value: ToolArgRef<'_>) -> &mut Self {
        let _command = self.inner.arg(*value);
        self
    }
    pub fn args(&mut self, values: ToolArgsRef<'_>) -> &mut Self {
        let _command = self.inner.args(*values);
        self
    }
    pub fn current_dir(&mut self, value: PathRef<'_>) -> &mut Self {
        let _command = self.inner.current_dir(*value);
        self
    }
    pub fn env(&mut self, key: ToolEnvKeyRef<'_>, value: ToolEnvValueRef<'_>) -> &mut Self {
        let _command = self.inner.env(*key, *value);
        self
    }
    #[must_use]
    pub fn new(program: ToolProgramRef<'_>) -> Self {
        Self {
            inner: crate::process_command::ProcessCommand::from(std::process::Command::new(
                *program,
            )),
            program: crate::os_string_value::OsStringValue::from(*program),
        }
    }
    pub fn output(&mut self) -> std::io::Result<ProcessOutput> {
        self.inner.output().map(ProcessOutput::from)
    }
    pub fn status(&mut self) -> std::io::Result<ProcessExitStatus> {
        self.inner.status().map(ProcessExitStatus::from)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn debug_redacts_arguments() {
        let mut command = super::ToolCommand::new(crate::tool_program_ref::ToolProgramRef::from(
            constants_str::PRINTF,
        ));
        let _command = command.arg(crate::tool_arg_ref::ToolArgRef::from(
            constants_str::SECRET_VALUE,
        ));
        let debug = format!("{command:?}");
        assert!(debug.contains("printf"));
        assert!(debug.contains("<redacted>"));
        assert!(!debug.contains("secret-value"));
    }
}
