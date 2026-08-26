#[path = "admin_contract_fixture.rs"]
pub(crate) mod admin_contract_fixture;
#[path = "cargo_subcommand_available.rs"]
pub(crate) mod cargo_subcommand_available;
#[path = "generate_pg_table_measure_input_token_stream.rs"]
pub(crate) mod generate_pg_table_measure_input_token_stream;
#[path = "measure_mode.rs"]
pub(crate) mod measure_mode;
#[path = "measurement.rs"]
pub(crate) mod measurement;
#[path = "query_part_workloads.rs"]
pub(crate) mod query_part_workloads;
pub(crate) fn run_workspace_tests() -> Result<(), ()> {
    if cargo_subcommand_available::cargo_subcommand_available(crate::domain_types::ToolName::from(
        constants_str::NEXTEST,
    ))
    .get()
    {
        println!("test_executor=nextest");
        crate::adapters::execution::run_commands(crate::adapters::execution::CommandsRef::from(
            &constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS,
        ))
    } else {
        println!("test_executor=cargo fallback=true");
        crate::adapters::execution::run_commands(crate::adapters::execution::CommandsRef::from(
            &constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS,
        ))
    }
}
