#[path = "os_string_value.rs"]
mod os_string_value;
#[path = "path_ref.rs"]
mod path_ref;
#[path = "process_command.rs"]
mod process_command;
#[path = "process_exit_status.rs"]
mod process_exit_status;
#[path = "process_output.rs"]
mod process_output;
#[path = "tool_arg_ref.rs"]
mod tool_arg_ref;
#[path = "tool_args_ref.rs"]
mod tool_args_ref;
#[path = "tool_env_key_ref.rs"]
mod tool_env_key_ref;
#[path = "tool_env_value_ref.rs"]
mod tool_env_value_ref;
#[path = "tool_program_ref.rs"]
mod tool_program_ref;

pub use path_ref::PathRef;
pub use process_exit_status::ProcessExitStatus;
pub use process_output::ProcessOutput;
pub use tool_arg_ref::ToolArgRef;
pub use tool_args_ref::ToolArgsRef;
pub use tool_env_key_ref::ToolEnvKeyRef;
pub use tool_env_value_ref::ToolEnvValueRef;
pub use tool_program_ref::ToolProgramRef;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub struct ToolCommand {
    inner: process_command::ProcessCommand,
    program: os_string_value::OsStringValue,
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
            inner: process_command::ProcessCommand::from(std::process::Command::new(*program)),
            program: os_string_value::OsStringValue::from(*program),
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
        let mut command = super::ToolCommand::new(super::tool_program_ref::ToolProgramRef::from(
            constants_str::PRINTF,
        ));
        let _command = command.arg(super::tool_arg_ref::ToolArgRef::from(
            constants_str::SECRET_VALUE,
        ));
        let debug = format!("{command:?}");
        assert!(debug.contains("printf"));
        assert!(debug.contains("<redacted>"));
        assert!(!debug.contains("secret-value"));
    }
}
