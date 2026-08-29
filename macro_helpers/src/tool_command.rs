#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub struct ToolCommand {
    inner: crate::process_command::ProcessCommand,
    program: crate::os_string_value::OsStringValue,
}
impl std::fmt::Debug for ToolCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(constants_str::catalog::TOOLCOMMAND)
            .field(constants_str::catalog::PROGRAM, &*self.program)
            .field(
                constants_str::catalog::ARGUMENTS,
                &constants_str::catalog::REDACTED,
            )
            .finish_non_exhaustive()
    }
}
impl ToolCommand {
    pub fn arg(&mut self, value: crate::tool_arg_ref::ToolArgRef<'_>) -> &mut Self {
        let _command = self.inner.arg(*value);
        self
    }
    pub fn args(&mut self, values: crate::tool_args_ref::ToolArgsRef<'_>) -> &mut Self {
        let _command = self.inner.args(*values);
        self
    }
    pub fn current_dir(&mut self, value: crate::path_ref::PathRef<'_>) -> &mut Self {
        let _command = self.inner.current_dir(*value);
        self
    }
    pub fn env(
        &mut self,
        key: crate::tool_env_key_ref::ToolEnvKeyRef<'_>,
        value: crate::tool_env_value_ref::ToolEnvValueRef<'_>,
    ) -> &mut Self {
        let _command = self.inner.env(*key, *value);
        self
    }
    #[must_use]
    pub fn new(program: crate::tool_program_ref::ToolProgramRef<'_>) -> Self {
        Self {
            inner: crate::process_command::ProcessCommand::from(std::process::Command::new(
                *program,
            )),
            program: crate::os_string_value::OsStringValue::from(*program),
        }
    }
    pub fn output(&mut self) -> std::io::Result<crate::process_output::ProcessOutput> {
        self.inner
            .output()
            .map(crate::process_output::ProcessOutput::from)
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
    fn debug_redacts_arguments() {
        let mut command = super::ToolCommand::new(crate::tool_program_ref::ToolProgramRef::from(
            constants_str::catalog::PRINTF,
        ));
        let _command = command.arg(crate::tool_arg_ref::ToolArgRef::from(
            constants_str::catalog::SECRET_VALUE,
        ));
        let debug = format!("{command:?}");
        assert!(debug.contains("printf"));
        assert!(debug.contains("<redacted>"));
        assert!(!debug.contains("secret-value"));
    }
}
