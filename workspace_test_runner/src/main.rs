#![allow(
    clippy::exit,
    reason = "the workspace test runner owns immediate process termination for failed tool modes"
)]
#![allow(
    clippy::needless_for_each,
    reason = "repository policy forbids for loops"
)]

mod adapters;
mod domain_types;
mod run_workspace_tests;

fn main() {
    let mode = adapters::mode::mode();
    let result = match mode.as_ref().map(domain_types::RunnerMode::as_ref) {
        None | Some(constants_str::STATIC) => {
            adapters::execution::run_commands(adapters::execution::CommandsRef::from(
                &constants_str::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS,
            ))
        }
        Some(constants_str::DATABASE) => {
            match std::env::var(constants_str::ENV_NAMES_DATABASE_URL) {
                Ok(database_url) => {
                    match macro_helpers::domain_types::test_database::validate_test_database_url(
                        macro_helpers::domain_types::test_database::UrlRef::from(
                            database_url.as_str(),
                        ),
                    ) {
                        Ok(_target) => adapters::execution::run_commands(
                            adapters::execution::CommandsRef::from(&[(
                                constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                                &constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS[..],
                            )]),
                        ),
                        Err(error) => {
                            eprintln!("database test guard rejected DATABASE_URL: {error}");
                            Err(())
                        }
                    }
                }
                Err(error) => {
                    eprintln!("database test mode requires DATABASE_URL: {error}");
                    Err(())
                }
            }
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_WORKLOAD) => {
            let input = run_workspace_tests::generate_pg_table_measure_input_token_stream::generate_pg_table_measure_input_token_stream(
                &quote::quote! {"False"},
            );
            let output_bytes = (0..domain_types::DIRECT_GENERATION_REPEAT_COUNT).fold(
                constants_usize::ZERO,
                |accumulator, _| {
                    let output = generate_pg_table_src::domain_types::source::generate_pg_table(
                        macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(
                            input.as_ref(),
                        ),
                    );
                    accumulator.saturating_add(output.to_string().len())
                },
            );
            println!(
                "allocation_workload=generate_pg_table_src repeat_count={repeat_count} output_bytes={output_bytes}",
                repeat_count = domain_types::DIRECT_GENERATION_REPEAT_COUNT,
            );
            Ok(())
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TYPES_WORKLOAD) => {
            let input = quote::quote! {
                {
                    "pg_table_cols_write_into_file": "False",
                    "whole_write_into_file": "False",
                    "variant": "All"
                }
            };
            let output_bytes = (0..domain_types::DIRECT_GENERATION_REPEAT_COUNT).fold(
                constants_usize::ZERO,
                |accumulator, _| {
                    let output = generate_pg_types_src::domain_types::source::generate_pg_types(
                        macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(
                            &input,
                        ),
                    );
                    accumulator.saturating_add(output.to_string().len())
                },
            );
            println!(
                "allocation_workload=generate_pg_types_src repeat_count={repeat_count} output_bytes={output_bytes}",
                repeat_count = domain_types::DIRECT_GENERATION_REPEAT_COUNT,
            );
            Ok(())
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_ADMIN_CONTRACT_FIXTURE) => {
            run_workspace_tests::admin_contract_fixture::admin_contract_fixture()
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_PG_CRUD_COMMON_QUERY_PART_WORKLOAD) => {
            run_workspace_tests::run_pg_crud_common::run_pg_crud_common()
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_WHERE_FILTERS_QUERY_PART_WORKLOAD) => {
            run_workspace_tests::run_where_filters::run_where_filters()
        }
        Some(constants_str::MACRO_GENERATION) => domain_types::macro_generation_measurements()
            .iter()
            .try_fold((), |(), (measurement_name, args)| {
                run_workspace_tests::measure_cargo_command::measure_cargo_command(
                    *measurement_name,
                    *args,
                )
            }),
        Some(constants_str::TESTS_ALT) => run_workspace_tests::run_workspace_tests(),
        Some(constants_str::HEAVY_LOAD) => {
            if run_workspace_tests::cargo_subcommand_available::cargo_subcommand_available(
                domain_types::ToolName::from(constants_str::NEXTEST),
            )
            .get()
            {
                adapters::execution::run_commands(adapters::execution::CommandsRef::from(&[(
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                    &constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_HEAVY_ARGS[..],
                )]))
            } else {
                eprintln!("heavy-load mode requires cargo-nextest; optional tool is unavailable");
                Err(())
            }
        }
        Some(constants_str::RELEASE) => {
            [
                constants_str::WORKSPACE_TEST_RUNNER_AUDIT_SUBCOMMAND,
                constants_str::WORKSPACE_TEST_RUNNER_DENY_SUBCOMMAND,
                constants_str::WORKSPACE_TEST_RUNNER_HACK_SUBCOMMAND,
                constants_str::SEMVER_CHECKS,
                constants_str::UDEPS,
                constants_str::MACHETE,
                constants_str::LLVM_COV,
            ]
            .into_iter()
            .for_each(|tool| {
                println!(
                    "release_tool={tool} available={}",
                    run_workspace_tests::cargo_subcommand_available::cargo_subcommand_available(
                        domain_types::ToolName::from(tool)
                    )
                    .get()
                );
            });
            let mut commands =
                Vec::<(&str, &[&str])>::from(constants_str::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS);
            if run_workspace_tests::cargo_subcommand_available::cargo_subcommand_available(
                domain_types::ToolName::from(constants_str::NEXTEST),
            )
            .get()
            {
                commands.extend(constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS);
            } else {
                commands.extend(constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS);
            }
            [
                (
                    constants_str::WORKSPACE_TEST_RUNNER_AUDIT_SUBCOMMAND,
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO_AUDIT_ARGS.as_slice(),
                ),
                (
                    constants_str::WORKSPACE_TEST_RUNNER_DENY_SUBCOMMAND,
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO_DENY_ARGS.as_slice(),
                ),
                (
                    constants_str::WORKSPACE_TEST_RUNNER_HACK_SUBCOMMAND,
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO_HACK_ARGS.as_slice(),
                ),
                (
                    constants_str::MACHETE,
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO_MACHETE_ARGS.as_slice(),
                ),
                (
                    constants_str::SEMVER_CHECKS,
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO_SEMVER_CHECKS_ARGS.as_slice(),
                ),
                (
                    constants_str::UDEPS,
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO_UDEPS_ARGS.as_slice(),
                ),
            ]
            .into_iter()
            .filter(|(subcommand, _args)| {
                run_workspace_tests::cargo_subcommand_available::cargo_subcommand_available(
                    domain_types::ToolName::from(*subcommand),
                )
                .get()
            })
            .for_each(|(_subcommand, args)| {
                commands.push((constants_str::WORKSPACE_TEST_RUNNER_CARGO, args));
            });
            adapters::execution::run_commands(adapters::execution::CommandsRef::from(
                commands.as_slice(),
            ))
        }
        Some(constants_str::MEASURE) => run_workspace_tests::measure_mode::measure_mode(),
        Some(constants_str::ALL_ALT) => {
            adapters::execution::run_commands(adapters::execution::CommandsRef::from(
                &constants_str::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS,
            ))
            .and_then(|()| run_workspace_tests::run_workspace_tests())
            .and_then(|()| {
                domain_types::macro_generation_measurements()
                    .iter()
                    .try_fold((), |(), (measurement_name, args)| {
                        run_workspace_tests::measure_cargo_command::measure_cargo_command(
                            *measurement_name,
                            *args,
                        )
                    })
            })
        }
        Some(other) => {
            eprintln!(
                "unknown mode `{other}`; expected `static`, `database`, `tests`, `heavy-load`, `release`, `macro-generation`, `measure`, `all`, or `alloc-workload-*`"
            );
            Err(())
        }
    };
    if result.is_err() {
        std::process::exit(1);
    }
}
