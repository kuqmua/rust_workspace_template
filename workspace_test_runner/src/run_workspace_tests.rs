pub(crate) fn run_workspace_tests() -> Result<(), ()> {
    if crate::cargo_subcommand_available::cargo_subcommand_available(
        crate::tool_name::ToolName::from(constants_str::NEXTEST),
    )
    .get()
    {
        println!("test_executor=nextest");
        crate::run_commands::run_commands(crate::commands_ref::CommandsRef::from(
            &constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS,
        ))
    } else {
        println!("test_executor=cargo fallback=true");
        crate::run_commands::run_commands(crate::commands_ref::CommandsRef::from(
            &constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS,
        ))
    }
}
