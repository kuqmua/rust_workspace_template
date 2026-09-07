#![allow(
    clippy::exit,
    reason = "the workspace test runner owns immediate process termination for failed tool modes"
)]
#![allow(
    clippy::needless_for_each,
    reason = "repository policy forbids for loops"
)]
#![allow(
    clippy::wildcard_imports,
    reason = "root-owned runner modes retain the former domain facade vocabulary"
)]

pub mod admin_fixture;
pub mod admin_fixture_conversion_error;
pub mod admin_fixture_string;
pub mod allocation_tool;
pub mod allocation_tools;
pub mod cargo_args;
pub mod cargo_measurement_error;
pub mod cargo_subcommand_available;
pub mod check_tool_available;
pub mod clean_ansi_text;
pub mod command_duration;
pub mod command_duration_millis;
pub mod command_failure;
pub mod command_failures_vec_deque;
pub mod command_index;
pub mod command_run;
pub mod command_started_at_instant;
pub mod command_text;
pub mod command_texts;
pub mod commands_ref;
pub mod create_admin_fixture_string;
pub mod direct_generation_measurement;
pub mod direct_generation_output_measurement;
pub mod domain_types;
#[cfg(test)]
pub mod execution_tests;
pub mod failed_test_names;
pub mod generate_pg_table_measure_input_token_stream;
pub mod generation_stage_measurement;
pub mod macro_generation_measurements;
pub mod measure_cargo_command;
pub mod measure_direct_generation;
pub mod measure_generation_stages;
pub mod measure_memusage_command;
pub mod measurement_name;
pub mod memory_usage_column_index;
pub mod memusage_heap_value;
pub mod memusage_key;
pub mod memusage_measurement_error;
pub mod memusage_prog_name_ref;
pub mod memusage_row_name;
pub mod memusage_table_value;
pub mod memusage_value_ref;
pub mod print_without_measurement_footer;
pub mod print_without_memusage_footer;
pub mod program_args_ref;
pub mod program_path_ref;
pub mod quote_token_stream_generate_pg_table_measure_input_token_stream;
pub mod run_admin_fixture_cli;
pub mod run_commands;
pub mod run_commands_error;
pub mod run_counter;
pub mod run_measurements_cli;
pub mod run_report_error;
pub mod run_workspace_tests;
pub mod runner_cli_outcome;
pub mod runner_mode;
pub mod stderr_text_ref;
pub mod strip_ansi;
pub mod strip_ansi_codes;
pub mod summary_text;
pub mod summary_text_append_error;
pub mod text_ref;
pub mod tool_available;
pub mod tool_name;
pub mod tool_path;

#[cfg(test)]
mod test_runner_errors;
#[cfg(test)]
pub mod test_workspace_test_runner;

fn main() {
    let cli_result =
        |runner_cli_outcome: runner_cli_outcome::RunnerCliOutcome| match runner_cli_outcome {
            runner_cli_outcome::RunnerCliOutcome::Completed => Ok(()),
            runner_cli_outcome::RunnerCliOutcome::Failed => Err(()),
        };
    let render_run_commands_error = |run_commands_error: run_commands_error::RunCommandsError| {
        if let run_commands_error::RunCommandsError::CommandsFailed { command_failures }
        | run_commands_error::RunCommandsError::WriteReport {
            command_failures, ..
        } = &run_commands_error
        {
            command_failures
                .iter()
                .filter(|command_failure| {
                    matches!(
                        command_failure,
                        command_failure::CommandFailure::WriteOutput { .. }
                    )
                })
                .for_each(|command_failure| {
                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                        macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                        macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                            "{command_failure}{}",
                            constants_str::NEWLINE
                        )),
                    );
                });
        }
        if !matches!(
            run_commands_error,
            run_commands_error::RunCommandsError::CommandsFailed { .. }
        ) {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{run_commands_error}{}",
                    constants_str::NEWLINE
                )),
            );
        }
    };
    let run_commands = |commands_ref: commands_ref::CommandsRef<'_>| {
        run_commands::run_commands(commands_ref).map_err(render_run_commands_error)
    };
    let render_cargo_measurement_error =
        |cargo_measurement_error: cargo_measurement_error::CargoMeasurementError| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{cargo_measurement_error}{}",
                    constants_str::NEWLINE
                )),
            );
        };
    let mode = std::env::args().nth(constants_usize::ONE).map(|value| {
        runner_mode::RunnerMode::try_from(value).unwrap_or_else(runner_mode::RunnerMode::from)
    });
    let result = match mode.as_ref().map(runner_mode::RunnerMode::as_ref) {
        None | Some(constants_str::STATIC) => run_commands(commands_ref::CommandsRef::from(
            &constants_str::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS,
        )),
        Some(constants_str::DATABASE) => {
            match std::env::var(constants_str::ENV_NAMES_DATABASE_URL) {
                Ok(database_url) => {
                    match macro_helpers::validate_test_database_url::validate_test_database_url(
                        macro_helpers::url_ref::UrlRef::from(database_url.as_str()),
                    ) {
                        Ok(_target) => run_commands(commands_ref::CommandsRef::from(&[(
                            constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                            &constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS[..],
                        )])),
                        Err(error) => {
                            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{}{}", constants_str::RUNNER_CLI_TEXT_E7B425B0, error), constants_str::NEWLINE)));
                            Err(())
                        }
                    }
                }
                Err(error) => {
                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                        macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                        macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                            "{}{}",
                            format_args!("{}{}", constants_str::RUNNER_CLI_TEXT_80E68335, error),
                            constants_str::NEWLINE
                        )),
                    );
                    Err(())
                }
            }
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_WORKLOAD) => {
            let input = generate_pg_table_measure_input_token_stream::generate_pg_table_measure_input_token_stream(
                &quote::quote! {"False"},
            );
            let repeat_count = domain_types::DIRECT_GENERATION_REPEAT_COUNT;
            let output_bytes = (0..repeat_count).fold(constants_usize::ZERO, |accumulator, _| {
                let output = generate_pg_table_src::generate_pg_table::generate_pg_table(
                    macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                        input.as_ref(),
                    ),
                );
                accumulator.saturating_add(output.to_string().len())
            });
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!(
                        "{}{}{}{}",
                        constants_str::RUNNER_CLI_TEXT_A1E309C3,
                        repeat_count,
                        constants_str::RUNNER_CLI_TEXT_226B5908,
                        output_bytes
                    ),
                    constants_str::NEWLINE
                )),
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
            let repeat_count = domain_types::DIRECT_GENERATION_REPEAT_COUNT;
            let output_bytes = (0..repeat_count).fold(constants_usize::ZERO, |accumulator, _| {
                let output =
                    generate_pg_types_src::generate_pg_types_tokens::generate_pg_types_tokens(
                        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                            &input,
                        ),
                    );
                accumulator.saturating_add(output.to_string().len())
            });
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!(
                        "{}{}{}{}",
                        constants_str::RUNNER_CLI_TEXT_B9C0C850,
                        repeat_count,
                        constants_str::RUNNER_CLI_TEXT_226B5908,
                        output_bytes
                    ),
                    constants_str::NEWLINE
                )),
            );
            Ok(())
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_ADMIN_CONTRACT_FIXTURE) => {
            cli_result(run_admin_fixture_cli::run_admin_fixture_cli())
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_PG_CRUD_COMMON_QUERY_PART_WORKLOAD) => (|| {
            let output_bytes = (0..domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT)
                                .try_fold(constants_usize::ZERO, |series_accumulator, _| {
                                    (0..domain_types::MEASURE_REPEAT_COUNT).try_fold(
                                        series_accumulator,
                                        |accumulator, _| {
                                            let mut increment = constants_u64::ZERO;
                                            match pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                                                &pg_crud_common::pagination_base::PaginationBase::default(),
                                                &mut increment,
                                                pg_crud_common::sql_column_ref::SqlColumnRef::from(
                                                    &constants_str::COLUMN,
                                                ),
                                                pg_crud_common::add_operator::AddOperator::from(false),
                                            ) {
                                                Ok(fragment) => {
                                                    Ok(accumulator.saturating_add(fragment.as_ref().len()))
                                                }
                                                Err(error) => {
                                                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{}{:?}", constants_str::RUNNER_CLI_TEXT_53723818, error), constants_str::NEWLINE)));
                                                    Err(())
                                                }
                                            }
                                        },
                                    )
                                })?;
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!(
                        "{}{}{}{}{}{}",
                        constants_str::RUNNER_CLI_TEXT_CE093C07,
                        domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT,
                        constants_str::RUNNER_CLI_TEXT_333D0B67,
                        domain_types::MEASURE_REPEAT_COUNT,
                        constants_str::RUNNER_CLI_TEXT_226B5908,
                        output_bytes
                    ),
                    constants_str::NEWLINE
                )),
            );
            Ok(())
        })(
        ),
        Some(constants_str::WORKSPACE_TEST_RUNNER_WHERE_FILTERS_QUERY_PART_WORKLOAD) => (|| {
            let where_filters_values = (constants_i32::ZERO..64i32).collect::<Vec<i32>>();
            let where_filters_bounded_vec =
                match where_filters::pg_filter_vec::PgFilterVec::<i32, 64>::try_from(
                    where_filters_values,
                ) {
                    Ok(value) => value,
                    Err(error) => {
                        macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                            macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                            macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                                "{}{}",
                                format_args!(
                                    "{}{:?}",
                                    constants_str::RUNNER_CLI_TEXT_B9E9F340,
                                    error
                                ),
                                constants_str::NEWLINE
                            )),
                        );
                        return Err(());
                    }
                };
            let output_bytes = (0..domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT)
                                .try_fold(constants_usize::ZERO, |series_accumulator, _| {
                                    (0..domain_types::MEASURE_REPEAT_COUNT).try_fold(
                                        series_accumulator,
                                        |accumulator, _| {
                                            let mut increment = constants_u64::ZERO;
                                            match where_filters_bounded_vec.pg_type_query_part(
                                                &mut increment,
                                                pg_crud_common::sql_column_ref::SqlColumnRef::from(
                                                    &constants_str::COLUMN,
                                                ),
                                                pg_crud_common::add_operator::AddOperator::from(false),
                                            ) {
                                                Ok(fragment) => {
                                                    Ok(accumulator.saturating_add(fragment.as_ref().len()))
                                                }
                                                Err(error) => {
                                                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{}{:?}", constants_str::RUNNER_CLI_TEXT_D141B685, error), constants_str::NEWLINE)));
                                                    Err(())
                                                }
                                            }
                                        },
                                    )
                                })?;
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!(
                        "{}{}{}{}{}{}",
                        constants_str::RUNNER_CLI_TEXT_134D88FC,
                        domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT,
                        constants_str::RUNNER_CLI_TEXT_333D0B67,
                        domain_types::MEASURE_REPEAT_COUNT,
                        constants_str::RUNNER_CLI_TEXT_226B5908,
                        output_bytes
                    ),
                    constants_str::NEWLINE
                )),
            );
            Ok(())
        })(
        ),
        Some(constants_str::MACRO_GENERATION) => {
            macro_generation_measurements::macro_generation_measurements()
                .iter()
                .try_fold((), |(), (measurement_name, args)| {
                    measure_cargo_command::measure_cargo_command(*measurement_name, *args)
                        .map_err(render_cargo_measurement_error)
                })
        }
        Some(constants_str::TESTS_ALT) => {
            run_workspace_tests::run_workspace_tests().map_err(render_run_commands_error)
        }
        Some(constants_str::HEAVY_LOAD) => {
            if cargo_subcommand_available::cargo_subcommand_available(tool_name::ToolName::from(
                constants_str::NEXTEST,
            ))
            .get()
            {
                run_commands(commands_ref::CommandsRef::from(&[(
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                    &constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_HEAVY_ARGS[..],
                )]))
            } else {
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!("{}", constants_str::RUNNER_CLI_TEXT_86AA5651),
                        constants_str::NEWLINE
                    )),
                );
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
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!(
                            "{}{}{}{}",
                            constants_str::RUNNER_CLI_TEXT_AAB2F49E,
                            tool,
                            constants_str::RUNNER_CLI_TEXT_BF147B5B,
                            cargo_subcommand_available::cargo_subcommand_available(
                                tool_name::ToolName::from(tool)
                            )
                            .get()
                        ),
                        constants_str::NEWLINE
                    )),
                );
            });
            let mut commands =
                Vec::<(&str, &[&str])>::from(constants_str::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS);
            if cargo_subcommand_available::cargo_subcommand_available(tool_name::ToolName::from(
                constants_str::NEXTEST,
            ))
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
                cargo_subcommand_available::cargo_subcommand_available(tool_name::ToolName::from(
                    *subcommand,
                ))
                .get()
            })
            .for_each(|(_subcommand, args)| {
                commands.push((constants_str::WORKSPACE_TEST_RUNNER_CARGO, args));
            });
            run_commands(commands_ref::CommandsRef::from(commands.as_slice()))
        }
        Some(constants_str::MEASURE) => cli_result(run_measurements_cli::run_measurements_cli()),
        Some(constants_str::ALL_ALT) => run_commands(commands_ref::CommandsRef::from(
            &constants_str::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS,
        ))
        .and_then(|()| {
            run_workspace_tests::run_workspace_tests().map_err(render_run_commands_error)
        })
        .and_then(|()| {
            macro_generation_measurements::macro_generation_measurements()
                .iter()
                .try_fold((), |(), (measurement_name, args)| {
                    measure_cargo_command::measure_cargo_command(*measurement_name, *args)
                        .map_err(render_cargo_measurement_error)
                })
        }),
        Some(other) => {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!(
                        "{}{}{}",
                        constants_str::RUNNER_CLI_TEXT_A8DCE6DA,
                        other,
                        constants_str::RUNNER_CLI_TEXT_A774F989
                    ),
                    constants_str::NEWLINE
                )),
            );
            Err(())
        }
    };
    if result.is_err() {
        std::process::exit(1);
    }
}
