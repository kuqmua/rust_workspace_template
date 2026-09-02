pub(crate) fn cargo_subcommand_available(
    tool_name: crate::tool_name::ToolName,
) -> crate::tool_available::ToolAvailable {
    let args = [tool_name.get(), constants_str::VERSION];
    macro_helpers::tool_command::ToolCommand::new(
        macro_helpers::tool_program_ref::ToolProgramRef::from(
            constants_str::WORKSPACE_TEST_RUNNER_CARGO,
        ),
    )
    .args(macro_helpers::tool_args_ref::ToolArgsRef::from(
        args.as_slice(),
    ))
    .output()
    .is_ok_and(|output| output.status.success())
    .into()
}
