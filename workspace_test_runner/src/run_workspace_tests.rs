pub(crate) fn run_workspace_tests() -> Result<(), crate::run_commands_error::RunCommandsError> {
    let (commands_ref, announcement) =
        if crate::cargo_subcommand_available::cargo_subcommand_available(
            crate::tool_name::ToolName::from(constants_str::NEXTEST),
        )
        .get()
        {
            (
                crate::commands_ref::CommandsRef::from(
                    &constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS,
                ),
                constants_str::RUNNER_NEXTEST_ANNOUNCEMENT,
            )
        } else {
            (
                crate::commands_ref::CommandsRef::from(
                    &constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS,
                ),
                constants_str::RUNNER_CARGO_ANNOUNCEMENT,
            )
        };
    macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput
        .write(macro_helpers::std_fmt_arguments::StdFmtArguments::from(
            format_args!("{announcement}{}", constants_str::NEWLINE),
        ))
        .map_err(|tool_console_write_error| {
            crate::run_commands_error::RunCommandsError::WriteAnnouncement {
                tool_console_write_error,
            }
        })?;
    crate::run_commands::run_commands(commands_ref)
}
