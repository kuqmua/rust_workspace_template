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
    pub fn status(&mut self) -> std::io::Result<crate::process_exit_status::ProcessExitStatus> {
        self.inner
            .status()
            .map(crate::process_exit_status::ProcessExitStatus::from)
    }
}
#[cfg(test)]
mod tests {
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
