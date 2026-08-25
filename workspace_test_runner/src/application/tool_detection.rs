pub(super) fn cargo_subcommand_available(
    subcommand: crate::domain_types::ToolName,
) -> crate::domain_types::ToolAvailable {
    let args = [subcommand.get(), constants_str::VERSION];
    macro_helpers::domain_types::tool_command::ToolCommand::new(
        macro_helpers::domain_types::tool_command::ToolProgramRef::from(
            constants_str::WORKSPACE_TEST_RUNNER_CARGO,
        ),
    )
    .args(macro_helpers::domain_types::tool_command::ToolArgsRef::from(args.as_slice()))
    .output()
    .is_ok_and(|output| output.status.success())
    .into()
}
