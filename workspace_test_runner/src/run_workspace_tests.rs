pub(crate) mod cargo_subcommand_available {
    pub(crate) use crate::cargo_subcommand_available::*;
}
pub(crate) mod generate_pg_table_measure_input_token_stream {
    pub(crate) use crate::generate_pg_table_measure_input_token_stream::*;
}
pub(crate) mod measure_cargo_command {
    pub(crate) use crate::measure_cargo_command::*;
}
pub(crate) fn run_workspace_tests() -> Result<(), ()> {
    if cargo_subcommand_available::cargo_subcommand_available(crate::domain_types::ToolName::from(
        constants_str::NEXTEST,
    ))
    .get()
    {
        println!("test_executor=nextest");
        crate::execution::run_commands(crate::execution::CommandsRef::from(
            &constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS,
        ))
    } else {
        println!("test_executor=cargo fallback=true");
        crate::execution::run_commands(crate::execution::CommandsRef::from(
            &constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS,
        ))
    }
}
