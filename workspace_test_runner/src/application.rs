fn measure_memusage_command(
    measurement_name: crate::domain_types::MeasurementName,
    program: crate::domain_types::ProgramPathRef<'_>,
    args: crate::domain_types::ProgramArgsRef<'_>,
    memusage_prog_name: crate::domain_types::MemusageProgNameRef<'_>,
) -> Result<(), ()> {
    let measurement_name_value = measurement_name.get();
    if !std::path::Path::new(constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH).exists() {
        println!(
            "measurement={measurement_name_value}_allocations status=unavailable reason=libmemusage_not_found path={}",
            constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH
        );
        return Ok(());
    }
    let command_output = macro_helpers::domain_types::tool_command::ToolCommand::new(
        macro_helpers::domain_types::tool_command::ToolProgramRef::from(program.get()),
    )
    .args(macro_helpers::domain_types::tool_command::ToolArgsRef::from(args.get()))
    .env(
        macro_helpers::domain_types::tool_command::ToolEnvKeyRef::from(constants_str::LD_PRELOAD),
        macro_helpers::domain_types::tool_command::ToolEnvValueRef::from(
            constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH,
        ),
    )
    .env(
        macro_helpers::domain_types::tool_command::ToolEnvKeyRef::from(
            constants_str::MEMUSAGE_PROG_NAME,
        ),
        macro_helpers::domain_types::tool_command::ToolEnvValueRef::from(memusage_prog_name.get()),
    )
    .output();
    match command_output {
        Ok(output) if output.status.success() => {
            {
                let stdout = String::from_utf8_lossy(output.stdout.as_slice());
                if !stdout.is_empty() {
                    print!("{stdout}");
                }
            }
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            crate::adapters::reporting::print_without_memusage_footer(
                crate::domain_types::StderrTextRef::from(stderr.as_ref()),
            );
            let clean = crate::domain_types::strip_ansi_codes(
                crate::domain_types::AnsiTextRef::from(stderr.as_ref()),
            );
            let heap_total = crate::domain_types::memusage_heap_value(
                &clean,
                crate::domain_types::MemusageKey::from(constants_str::HEAP_TOTAL),
            )
            .get();
            let heap_peak = crate::domain_types::memusage_heap_value(
                &clean,
                crate::domain_types::MemusageKey::from(constants_str::HEAP_PEAK),
            )
            .get();
            let stack_peak = crate::domain_types::memusage_heap_value(
                &clean,
                crate::domain_types::MemusageKey::from(constants_str::STACK_PEAK),
            )
            .get();
            let malloc_calls = crate::domain_types::memusage_table_value(
                &clean,
                crate::domain_types::MemusageRowName::from(constants_str::MALLOC),
                crate::domain_types::MemusageColumnIdx::from(0),
            )
            .get();
            let malloc_memory = crate::domain_types::memusage_table_value(
                &clean,
                crate::domain_types::MemusageRowName::from(constants_str::MALLOC),
                crate::domain_types::MemusageColumnIdx::from(1),
            )
            .get();
            let malloc_failed = crate::domain_types::memusage_table_value(
                &clean,
                crate::domain_types::MemusageRowName::from(constants_str::MALLOC),
                crate::domain_types::MemusageColumnIdx::from(2),
            )
            .get();
            let realloc_calls = crate::domain_types::memusage_table_value(
                &clean,
                crate::domain_types::MemusageRowName::from(constants_str::REALLOC),
                crate::domain_types::MemusageColumnIdx::from(0),
            )
            .get();
            let realloc_memory = crate::domain_types::memusage_table_value(
                &clean,
                crate::domain_types::MemusageRowName::from(constants_str::REALLOC),
                crate::domain_types::MemusageColumnIdx::from(1),
            )
            .get();
            let realloc_failed = crate::domain_types::memusage_table_value(
                &clean,
                crate::domain_types::MemusageRowName::from(constants_str::REALLOC),
                crate::domain_types::MemusageColumnIdx::from(2),
            )
            .get();
            let calloc_calls = crate::domain_types::memusage_table_value(
                &clean,
                crate::domain_types::MemusageRowName::from(constants_str::CALLOC),
                crate::domain_types::MemusageColumnIdx::from(0),
            )
            .get();
            let calloc_memory = crate::domain_types::memusage_table_value(
                &clean,
                crate::domain_types::MemusageRowName::from(constants_str::CALLOC),
                crate::domain_types::MemusageColumnIdx::from(1),
            )
            .get();
            let calloc_failed = crate::domain_types::memusage_table_value(
                &clean,
                crate::domain_types::MemusageRowName::from(constants_str::CALLOC),
                crate::domain_types::MemusageColumnIdx::from(2),
            )
            .get();
            let free_calls = crate::domain_types::memusage_table_value(
                &clean,
                crate::domain_types::MemusageRowName::from(constants_str::FREE),
                crate::domain_types::MemusageColumnIdx::from(0),
            )
            .get();
            let free_memory = crate::domain_types::memusage_table_value(
                &clean,
                crate::domain_types::MemusageRowName::from(constants_str::FREE),
                crate::domain_types::MemusageColumnIdx::from(1),
            )
            .get();
            println!(
                "measurement={measurement_name_value}_allocations status=ok tool=libmemusage heap_total_bytes={heap_total} heap_peak_bytes={heap_peak} stack_peak_bytes={stack_peak} malloc_calls={malloc_calls} malloc_bytes={malloc_memory} malloc_failed={malloc_failed} realloc_calls={realloc_calls} realloc_bytes={realloc_memory} realloc_failed={realloc_failed} calloc_calls={calloc_calls} calloc_bytes={calloc_memory} calloc_failed={calloc_failed} free_calls={free_calls} free_bytes={free_memory}"
            );
            Ok(())
        }
        Ok(output) => {
            {
                let stdout = String::from_utf8_lossy(output.stdout.as_slice());
                if !stdout.is_empty() {
                    print!("{stdout}");
                }
            }
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            crate::adapters::reporting::print_without_memusage_footer(
                crate::domain_types::StderrTextRef::from(stderr.as_ref()),
            );
            eprintln!(
                "measurement={measurement_name_value}_allocations status=failed exit_status={}",
                output.status
            );
            Err(())
        }
        Err(error) => {
            eprintln!(
                "measurement={measurement_name_value}_allocations status=spawn_failed error={error}"
            );
            Err(())
        }
    }
}
fn measure_cargo_command(
    measurement_name: crate::domain_types::MeasurementName,
    args: crate::domain_types::CargoArgs,
) -> Result<(), ()> {
    let measurement_name_value = measurement_name.get();
    let started = std::time::Instant::now();
    let command_output = {
        let measurement_format = format!(
            "{}%M\n{}%R\n{}%F",
            constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX,
            constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX,
            constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX,
        );
        macro_helpers::domain_types::tool_command::ToolCommand::new(
            macro_helpers::domain_types::tool_command::ToolProgramRef::from(
                constants_str::WORKSPACE_TEST_RUNNER_TIME_PATH,
            ),
        )
        .arg(macro_helpers::domain_types::tool_command::ToolArgRef::from(
            constants_str::F,
        ))
        .arg(macro_helpers::domain_types::tool_command::ToolArgRef::from(
            measurement_format.as_str(),
        ))
        .arg(macro_helpers::domain_types::tool_command::ToolArgRef::from(
            constants_str::WORKSPACE_TEST_RUNNER_CARGO,
        ))
        .args(macro_helpers::domain_types::tool_command::ToolArgsRef::from(args.get()))
        .output()
    };
    let duration = started.elapsed();
    match command_output {
        Ok(output) if output.status.success() => {
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            let peak_rss_kb = stderr
                .lines()
                .find_map(|line| {
                    line.trim()
                        .strip_prefix(constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX)
                })
                .unwrap_or(constants_str::UNAVAILABLE);
            let minor_page_faults = stderr
                .lines()
                .find_map(|line| {
                    line.trim()
                        .strip_prefix(constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX)
                })
                .unwrap_or(constants_str::UNAVAILABLE);
            let major_page_faults = stderr
                .lines()
                .find_map(|line| {
                    line.trim()
                        .strip_prefix(constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX)
                })
                .unwrap_or(constants_str::UNAVAILABLE);
            {
                let stdout = String::from_utf8_lossy(output.stdout.as_slice());
                if !stdout.is_empty() {
                    print!("{stdout}");
                }
            }
            crate::adapters::reporting::print_without_measurement_footer(
                crate::domain_types::StderrTextRef::from(stderr.as_ref()),
            );
            println!(
                "measurement={measurement_name_value} wall_ms={} memory_proxy_peak_rss_kb={} memory_proxy_minor_page_faults={} memory_proxy_major_page_faults={} status=ok",
                duration.as_millis(),
                peak_rss_kb,
                minor_page_faults,
                major_page_faults
            );
            Ok(())
        }
        Ok(output) => {
            {
                let stdout = String::from_utf8_lossy(output.stdout.as_slice());
                if !stdout.is_empty() {
                    print!("{stdout}");
                }
            }
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            crate::adapters::reporting::print_without_measurement_footer(
                crate::domain_types::StderrTextRef::from(stderr.as_ref()),
            );
            eprintln!(
                "measurement={measurement_name_value} status=failed exit_status={}",
                output.status
            );
            Err(())
        }
        Err(error) => {
            eprintln!("measurement={measurement_name_value} status=spawn_failed error={error}");
            Err(())
        }
    }
}
fn generate_pg_table_measure_input_token_stream(
    tests_write_into_file: &dyn quote::ToTokens,
) -> crate::domain_types::QuoteTokenStreamGeneratePgTableMeasureInputTokenStream {
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    crate::domain_types::QuoteTokenStreamGeneratePgTableMeasureInputTokenStream::from(
        quote::quote! {
            #allow_clippy_arbitrary_src_item_ordering
            #[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
            #[generate_pg_table::generate_pg_table_config{{
                "cm_write_into_file": "False",
                "co_write_into_file": "False",
                "rm_write_into_file": "False",
                "ro_write_into_file": "False",
                "um_write_into_file": "False",
                "uo_write_into_file": "False",
                "dm_write_into_file": "False",
                "dlo_write_into_file": "False",
                "tests_write_into_file": #tests_write_into_file,
                "common_write_into_file": "False",
                "whole_write_into_file": "False"
            }}]
            #[generate_pg_table::common_error_variants{
                enum CommonErrorVariants {
                    CheckCommit {
                        #[eo_location]
                        check_commit: route_validators::domain_types::check_commit::CommitError,
                        location: location_lib::domain_types::Location,
                    },
                }
            }]
            #[generate_pg_table::cm_logic{}]
            #[generate_pg_table::co_logic{}]
            #[generate_pg_table::rm_logic{}]
            #[generate_pg_table::ro_logic{}]
            #[generate_pg_table::um_logic{}]
            #[generate_pg_table::uo_logic{}]
            #[generate_pg_table::dm_logic{}]
            #[generate_pg_table::dlo_logic{}]
            #[generate_pg_table::common_logic{}]
            pub struct TableExample {
                #[generate_pg_table_primary_key]
                pub primary_key_column: pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPg,
                pub column_0: pg_types_numeric::I16AsNonNullInt2,
                pub column_1: pg_types_numeric::OptionalI16AsNullableInt2,
                pub column_2: pg_types_numeric::I32AsNonNullInt4,
            }
        },
    )
}
// Allocation workloads are separate process entry points dispatched by CLI mode.
#[allow(clippy::single_call_fn)]
fn run_alloc_workload_generate_pg_table_src() {
    let input = generate_pg_table_measure_input_token_stream(&quote::quote! {"False"});
    let output_bytes = (0..crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT).fold(
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
        repeat_count = crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT,
    );
}
// Allocation workloads are separate process entry points dispatched by CLI mode.
#[allow(clippy::single_call_fn)]
fn run_alloc_workload_generate_pg_types_src() {
    let input = quote::quote! {
        {
            "pg_table_cols_write_into_file": "False",
            "whole_write_into_file": "False",
            "variant": "All"
        }
    };
    let output_bytes = (0..crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT).fold(
        constants_usize::ZERO,
        |accumulator, _| {
            let output = generate_pg_types_src::domain_types::source::generate_pg_types(
                macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(&input),
            );
            accumulator.saturating_add(output.to_string().len())
        },
    );
    println!(
        "allocation_workload=generate_pg_types_src repeat_count={repeat_count} output_bytes={output_bytes}",
        repeat_count = crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT,
    );
}
// Allocation workloads are separate process entry points dispatched by CLI mode.
#[allow(clippy::single_call_fn)]
fn run_alloc_workload_pg_crud_common_query_part() -> Result<(), ()> {
    let output_bytes =
        (0..crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(constants_usize::ZERO, |series_accumulator, _| {
            (0..crate::domain_types::MEASURE_REPEAT_COUNT).try_fold(series_accumulator, |accumulator, _| {
                let mut increment = constants_u64::ZERO;
                match pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                    &pg_crud_common::domain_types::PaginationBase::default(),
                    &mut increment,
                    pg_crud_common::domain_types::SqlColumnRef::from(&constants_str::COLUMN),
                    pg_crud_common::domain_types::AddOperator::from(false),
                ) {
                    Ok(fragment) => Ok(accumulator.saturating_add(fragment.as_ref().len())),
                    Err(error) => {
                        eprintln!(
                            "allocation_workload=pg_crud_common_query_part status=failed error={error:?}"
                        );
                        Err(())
                    }
                }
            })
        })?;
    println!(
        "allocation_workload=pg_crud_common_query_part series_count={series_count} repeat_count={repeat_count} output_bytes={output_bytes}",
        series_count = crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT,
        repeat_count = crate::domain_types::MEASURE_REPEAT_COUNT,
    );
    Ok(())
}
// Allocation workloads are separate process entry points dispatched by CLI mode.
#[allow(clippy::single_call_fn)]
fn run_alloc_workload_where_filters_query_part() -> Result<(), ()> {
    let where_filters_values = (constants_i32::ZERO..64i32).collect::<Vec<i32>>();
    let where_filters_bounded_vec =
        match where_filters::domain_types::BoundedVec::<i32, 64>::try_from(where_filters_values) {
            Ok(value) => value,
            Err(error) => {
                eprintln!(
                    "allocation_workload=where_filters_query_part status=setup_failed error={error:?}"
                );
                return Err(());
            }
        };
    let output_bytes =
        (0..crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(constants_usize::ZERO, |series_accumulator, _| {
            (0..crate::domain_types::MEASURE_REPEAT_COUNT).try_fold(series_accumulator, |accumulator, _| {
                let mut increment = constants_u64::ZERO;
                match where_filters_bounded_vec.pg_type_query_part(
                    &mut increment,
                    pg_crud_common::domain_types::SqlColumnRef::from(&constants_str::COLUMN),
                    pg_crud_common::domain_types::AddOperator::from(false),
                ) {
                    Ok(fragment) => Ok(accumulator.saturating_add(fragment.as_ref().len())),
                    Err(error) => {
                        eprintln!("allocation_workload=where_filters_query_part status=failed error={error:?}");
                        Err(())
                    }
                }
            })
        })?;
    println!(
        "allocation_workload=where_filters_query_part series_count={series_count} repeat_count={repeat_count} output_bytes={output_bytes}",
        series_count = crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT,
        repeat_count = crate::domain_types::MEASURE_REPEAT_COUNT,
    );
    Ok(())
}
fn cargo_subcommand_available(
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
#[allow(
    clippy::needless_for_each,
    clippy::single_call_fn,
    reason = "keeps release-tool reporting separate and repository policy forbids for loops"
)]
fn print_optional_release_tools() {
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
            cargo_subcommand_available(crate::domain_types::ToolName::from(tool)).get()
        );
    });
}
#[allow(clippy::single_call_fn)] // release orchestration is an explicit CLI mode boundary
fn run_release() -> Result<(), ()> {
    print_optional_release_tools();
    let mut commands =
        Vec::<(&str, &[&str])>::from(constants_str::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS);
    if cargo_subcommand_available(crate::domain_types::ToolName::from(constants_str::NEXTEST)).get()
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
        cargo_subcommand_available(crate::domain_types::ToolName::from(*subcommand)).get()
    })
    .for_each(|(_subcommand, args)| {
        commands.push((constants_str::WORKSPACE_TEST_RUNNER_CARGO, args));
    });
    crate::adapters::execution::run_commands(crate::adapters::execution::CommandsRef::from(
        commands.as_slice(),
    ))
}
fn run_workspace_tests() -> Result<(), ()> {
    if cargo_subcommand_available(crate::domain_types::ToolName::from(constants_str::NEXTEST)).get()
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
#[allow(
    clippy::single_call_fn,
    reason = "the executable adapter delegates test orchestration to its owned module"
)]
pub(crate) fn run_main() {
    let mode = crate::adapters::discovery::mode();
    let result = match mode.as_ref().map(crate::domain_types::RunnerMode::as_ref) {
        None | Some(constants_str::STATIC) => {
            crate::adapters::execution::run_commands(crate::adapters::execution::CommandsRef::from(
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
                        Ok(_target) => crate::adapters::execution::run_commands(
                            crate::adapters::execution::CommandsRef::from(&[(
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
            run_alloc_workload_generate_pg_table_src();
            Ok(())
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TYPES_WORKLOAD) => {
            run_alloc_workload_generate_pg_types_src();
            Ok(())
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_ADMIN_CONTRACT_FIXTURE) => {
            crate::adapters::admin_fixture::write_admin_contract_fixture()
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_PG_CRUD_COMMON_QUERY_PART_WORKLOAD) => {
            run_alloc_workload_pg_crud_common_query_part()
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_WHERE_FILTERS_QUERY_PART_WORKLOAD) => {
            run_alloc_workload_where_filters_query_part()
        }
        Some(constants_str::MACRO_GENERATION) => {
            crate::domain_types::macro_generation_measurements()
                .iter()
                .try_fold((), |(), (measurement_name, args)| {
                    measure_cargo_command(*measurement_name, *args)
                })
        }
        Some(constants_str::TESTS_ALT) => run_workspace_tests(),
        Some(constants_str::HEAVY_LOAD) => {
            if cargo_subcommand_available(crate::domain_types::ToolName::from(
                constants_str::NEXTEST,
            ))
            .get()
            {
                crate::adapters::execution::run_commands(
                    crate::adapters::execution::CommandsRef::from(&[(
                        constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                        &constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_HEAVY_ARGS[..],
                    )]),
                )
            } else {
                eprintln!("heavy-load mode requires cargo-nextest; optional tool is unavailable");
                Err(())
            }
        }
        Some(constants_str::RELEASE) => run_release(),
        Some(constants_str::MEASURE) => {
            let allocation_tools_printed: Result<(), std::convert::Infallible> =
                crate::domain_types::allocation_tools()
                    .iter()
                    .try_fold((), |(), tool| {
                        let available = crate::adapters::discovery::tool_available(tool.path());
                        crate::adapters::reporting::allocation_tool(
                            tool.name(),
                            tool.path(),
                            available,
                        );
                        Ok(())
                    });
            match allocation_tools_printed {
                Ok(()) => {}
                Err(error) => match error {},
            }
            if std::path::Path::new(constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH).exists() {
                println!(
                    "measurement=exact_allocations status=available tool=libmemusage path={}",
                    constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH
                );
                measure_memusage_command(
                    crate::domain_types::MeasurementName::from(constants_str::CODE_STYLE),
                    crate::domain_types::ProgramPathRef::from(
                        constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                    ),
                    crate::domain_types::ProgramArgsRef::from(&[
                        constants_str::TEST_ALT_3,
                        constants_str::P,
                        constants_str::TESTS_ALT,
                        constants_str::CODE_STYLE,
                    ]),
                    crate::domain_types::MemusageProgNameRef::from(
                        constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                    ),
                )
                .unwrap_or_else(|()| std::process::exit(1));
                let current_exe = match std::env::current_exe() {
                    Ok(value) => value,
                    Err(error) => {
                        eprintln!(
                            "measurement=exact_allocations status=current_exe_failed error={error}"
                        );
                        std::process::exit(1);
                    }
                };
                let current_exe_string = current_exe.to_string_lossy().to_string();
                let current_exe_prog_name = current_exe
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or(constants_str::WORKSPACE_TEST_RUNNER_ALT);
                [
                    (
                        crate::domain_types::MeasurementName::from(
                            constants_str::GENERATE_PG_TABLE_SRC,
                        ),
                        constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_WORKLOAD,
                    ),
                    (
                        crate::domain_types::MeasurementName::from(
                            constants_str::GENERATE_PG_TYPES_SRC,
                        ),
                        constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TYPES_WORKLOAD,
                    ),
                    (
                        crate::domain_types::MeasurementName::from(
                            constants_str::PG_CRUD_COMMON_QUERY_PART,
                        ),
                        constants_str::WORKSPACE_TEST_RUNNER_PG_CRUD_COMMON_QUERY_PART_WORKLOAD,
                    ),
                    (
                        crate::domain_types::MeasurementName::from(
                            constants_str::WHERE_FILTERS_QUERY_PART,
                        ),
                        constants_str::WORKSPACE_TEST_RUNNER_WHERE_FILTERS_QUERY_PART_WORKLOAD,
                    ),
                ]
                .into_iter()
                .try_fold((), |(), (measurement_name, workload_mode)| {
                    measure_memusage_command(
                        measurement_name,
                        crate::domain_types::ProgramPathRef::from(current_exe_string.as_str()),
                        crate::domain_types::ProgramArgsRef::from(&[workload_mode]),
                        crate::domain_types::MemusageProgNameRef::from(current_exe_prog_name),
                    )
                })
                .unwrap_or_else(|()| std::process::exit(1));
            } else {
                println!(
                    "measurement=exact_allocations status=unavailable reason=no_safe_allocator_counter_or_external_allocation_profiler memory_proxy_fields=memory_proxy_peak_rss_kb,memory_proxy_minor_page_faults,memory_proxy_major_page_faults"
                );
            }
            measure_cargo_command(
                crate::domain_types::MeasurementName::from(constants_str::CODE_STYLE),
                crate::domain_types::CargoArgs::from(&[
                    constants_str::TEST_ALT_3,
                    constants_str::P,
                    constants_str::TESTS_ALT,
                    constants_str::CODE_STYLE,
                ]),
            )
            .unwrap_or_else(|()| std::process::exit(1));
            measure_cargo_command(
                crate::domain_types::MeasurementName::from(constants_str::CLIPPY),
                crate::domain_types::CargoArgs::from(
                    &constants_str::WORKSPACE_TEST_RUNNER_CARGO_CLIPPY_ARGS,
                ),
            )
            .unwrap_or_else(|()| std::process::exit(1));
            let generate_pg_table_input_token_stream =
                generate_pg_table_measure_input_token_stream(&quote::quote! {"False"});
            let generate_pg_table_input_with_tests_token_stream =
                generate_pg_table_measure_input_token_stream(&quote::quote! {"True"});
            let parse_started = std::time::Instant::now();
            let parsed = generate_pg_table_src::domain_types::pipeline::parse_generate_pg_table(
                macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(
                    generate_pg_table_input_token_stream.as_ref(),
                ),
            )
            .unwrap_or_else(|error| panic!("d6399cbf: {error}"));
            let parse_us = parse_started.elapsed().as_micros();
            let build_started = std::time::Instant::now();
            let built =
                generate_pg_table_src::domain_types::pipeline::build_generate_pg_table(parsed)
                    .unwrap_or_else(|error| panic!("6acb4e92: {error}"));
            let build_us = build_started.elapsed().as_micros();
            let validate_started = std::time::Instant::now();
            let validated =
                generate_pg_table_src::domain_types::pipeline::validate_generate_pg_table(built)
                    .unwrap_or_else(|error| panic!("4533a758: {error}"));
            let validate_us = validate_started.elapsed().as_micros();
            let emit_started = std::time::Instant::now();
            let staged_output =
                generate_pg_table_src::domain_types::source::emit_generate_pg_table(validated);
            let emit_us = emit_started.elapsed().as_micros();
            println!(
                "measurement=generate_pg_table_typed_stages parse_us={parse_us} build_us={build_us} validate_us={validate_us} emit_us={emit_us} output_bytes={}",
                staged_output.to_string().len()
            );
            let generate_pg_table_measurement =
                (0..crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT).fold(
                    (
                        u128::MAX,
                        constants_u128::ZERO,
                        constants_u128::ZERO,
                        constants_usize::ZERO,
                        constants_usize::ZERO,
                    ),
                    |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                        let started = std::time::Instant::now();
                        let output = generate_pg_table_src::domain_types::source::generate_pg_table(
                            macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(
                                generate_pg_table_input_token_stream.as_ref(),
                            ),
                        );
                        let wall_us = started.elapsed().as_micros();
                        (
                            min_wall_us.min(wall_us),
                            max_wall_us.max(wall_us),
                            total_wall_us.saturating_add(wall_us),
                            output.to_string().len(),
                            output.as_ref().clone().into_iter().count(),
                        )
                    },
                );
            println!(
                "measurement=generate_pg_table_src repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                generate_pg_table_measurement.0,
                generate_pg_table_measurement.2,
                generate_pg_table_measurement.1,
                generate_pg_table_measurement.3,
                generate_pg_table_measurement.4
            );
            let generate_pg_table_with_tests_dir =
                std::path::Path::new(constants_str::TARGET_MEASURE_GENERATE_PG_TABLE_WITH_TESTS);
            if let Err(error) = std::fs::create_dir_all(generate_pg_table_with_tests_dir) {
                eprintln!(
                    "measurement=generate_pg_table_src_with_tests status=create_dir_failed error={error}"
                );
                std::process::exit(1);
            }
            if let Err(error) = std::fs::write(
                generate_pg_table_with_tests_dir.join(constants_str::RUSTFMT_TOML),
                constants_str::EDITION_2024_NEWLINE,
            ) {
                eprintln!(
                    "measurement=generate_pg_table_src_with_tests status=rustfmt_config_write_failed error={error}"
                );
                std::process::exit(1);
            }
            let current_dir = match std::env::current_dir() {
                Ok(value) => value,
                Err(error) => {
                    eprintln!(
                        "measurement=generate_pg_table_src_with_tests status=current_dir_failed error={error}"
                    );
                    std::process::exit(1);
                }
            };
            if let Err(error) = std::env::set_current_dir(generate_pg_table_with_tests_dir) {
                eprintln!(
                    "measurement=generate_pg_table_src_with_tests status=set_current_dir_failed error={error}"
                );
                std::process::exit(1);
            }
            let generate_pg_table_with_tests_measurement =
                (0..crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT).fold(
                    (
                        u128::MAX,
                        constants_u128::ZERO,
                        constants_u128::ZERO,
                        constants_usize::ZERO,
                        constants_usize::ZERO,
                    ),
                    |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                        let started = std::time::Instant::now();
                        let output = generate_pg_table_src::domain_types::source::generate_pg_table(
                            macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(
                                generate_pg_table_input_with_tests_token_stream.as_ref(),
                            ),
                        );
                        let wall_us = started.elapsed().as_micros();
                        (
                            min_wall_us.min(wall_us),
                            max_wall_us.max(wall_us),
                            total_wall_us.saturating_add(wall_us),
                            output.to_string().len(),
                            output.as_ref().clone().into_iter().count(),
                        )
                    },
                );
            if let Err(error) = std::env::set_current_dir(current_dir) {
                eprintln!(
                    "measurement=generate_pg_table_src_with_tests status=restore_current_dir_failed error={error}"
                );
                std::process::exit(1);
            }
            let generate_pg_table_tests_stage_output_path =
                generate_pg_table_with_tests_dir.join(constants_str::GENERATE_PG_TABLE_TESTS_RS);
            let generate_pg_table_tests_stage_output =
                match server_runtime_http::domain_types::read_bounded_file(
                    server_runtime_http::domain_types::PathRef::from(
                        generate_pg_table_tests_stage_output_path.as_path(),
                    ),
                    server_runtime_http::domain_types::BoundedReadMaximumBytes::from(
                        constants_usize::VALUE_16_777_216,
                    ),
                )
                .and_then(server_runtime_http::domain_types::BoundedText::try_from)
                {
                    Ok(content) => (content.as_ref().len(), content.as_ref().lines().count()),
                    Err(error) => {
                        eprintln!(
                            "measurement=generate_pg_table_tests_stage_output status=read_failed error={error}"
                        );
                        std::process::exit(1);
                    }
                };
            println!(
                "measurement=generate_pg_table_src_with_tests repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                generate_pg_table_with_tests_measurement.0,
                generate_pg_table_with_tests_measurement.2,
                generate_pg_table_with_tests_measurement.1,
                generate_pg_table_with_tests_measurement.3,
                generate_pg_table_with_tests_measurement.4
            );
            println!(
                "measurement=generate_pg_table_tests_stage_output bytes={} lines={}",
                generate_pg_table_tests_stage_output.0, generate_pg_table_tests_stage_output.1
            );
            println!(
                "measurement=generate_pg_table_tests_emit_delta repeat_count={} wall_total_delta_us={} wall_min_delta_us={} wall_max_delta_us={} output_bytes_delta={}",
                crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                generate_pg_table_with_tests_measurement
                    .2
                    .saturating_sub(generate_pg_table_measurement.2),
                generate_pg_table_with_tests_measurement
                    .0
                    .saturating_sub(generate_pg_table_measurement.0),
                generate_pg_table_with_tests_measurement
                    .1
                    .saturating_sub(generate_pg_table_measurement.1),
                generate_pg_table_with_tests_measurement
                    .3
                    .saturating_sub(generate_pg_table_measurement.3)
            );
            let generate_pg_types_input_token_stream = quote::quote! {
                {
                    "pg_table_cols_write_into_file": "False",
                    "whole_write_into_file": "False",
                    "variant": "All"
                }
            };
            let pg_types_parse_started = std::time::Instant::now();
            let parsed_pg_types =
                generate_pg_types_src::domain_types::source::parse_generate_pg_types(
                    macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(
                        &generate_pg_types_input_token_stream,
                    ),
                )
                .unwrap_or_else(|error| panic!("a19c725e: {error}"));
            let pg_types_parse_us = pg_types_parse_started.elapsed().as_micros();
            let pg_types_build_started = std::time::Instant::now();
            let built_pg_types =
                generate_pg_types_src::domain_types::source::build_generate_pg_types(
                    parsed_pg_types,
                )
                .unwrap_or_else(|error| panic!("c47612bd: {error}"));
            let pg_types_build_us = pg_types_build_started.elapsed().as_micros();
            let pg_types_validate_started = std::time::Instant::now();
            let validated_pg_types =
                generate_pg_types_src::domain_types::source::validate_generate_pg_types(
                    built_pg_types,
                )
                .unwrap_or_else(|error| panic!("d3e581a4: {error}"));
            let pg_types_validate_us = pg_types_validate_started.elapsed().as_micros();
            let pg_types_emit_started = std::time::Instant::now();
            let staged_pg_types =
                generate_pg_types_src::domain_types::source::emit_generate_pg_types(
                    validated_pg_types,
                );
            let pg_types_emit_us = pg_types_emit_started.elapsed().as_micros();
            println!(
                "measurement=generate_pg_types_typed_stages parse_us={pg_types_parse_us} build_us={pg_types_build_us} validate_us={pg_types_validate_us} emit_us={pg_types_emit_us} output_bytes={}",
                staged_pg_types.to_string().len()
            );
            let generate_pg_types_measurement =
                (0..crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT).fold(
                    (
                        u128::MAX,
                        constants_u128::ZERO,
                        constants_u128::ZERO,
                        constants_usize::ZERO,
                        constants_usize::ZERO,
                    ),
                    |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                        let started = std::time::Instant::now();
                        let output = generate_pg_types_src::domain_types::source::generate_pg_types(
                            macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(
                                &generate_pg_types_input_token_stream,
                            ),
                        );
                        let wall_us = started.elapsed().as_micros();
                        (
                            min_wall_us.min(wall_us),
                            max_wall_us.max(wall_us),
                            total_wall_us.saturating_add(wall_us),
                            output.to_string().len(),
                            output.as_ref().clone().into_iter().count(),
                        )
                    },
                );
            println!(
                "measurement=generate_pg_types_src repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                generate_pg_types_measurement.0,
                generate_pg_types_measurement.2,
                generate_pg_types_measurement.1,
                generate_pg_types_measurement.3,
                generate_pg_types_measurement.4
            );
            let generate_where_filters_input_token_stream = quote::quote! {
                {
                    "pg_types_write_into_file": "False",
                    "whole_write_into_file": "False"
                }
            };
            let where_filters_parse_started = std::time::Instant::now();
            let parsed_where_filters = generate_where_filters_src::domain_types::source::parse_generate_where_filters(
                generate_where_filters_src::domain_types::source::ProcMacro2GenerateWhereFiltersInput::from(
                    &generate_where_filters_input_token_stream,
                ),
            )
            .unwrap_or_else(|error| panic!("8f246dc1: {error}"));
            let where_filters_parse_us = where_filters_parse_started.elapsed().as_micros();
            let where_filters_build_started = std::time::Instant::now();
            let built_where_filters =
                generate_where_filters_src::domain_types::source::build_generate_where_filters(
                    parsed_where_filters,
                )
                .unwrap_or_else(|error| panic!("912f6bce: {error}"));
            let where_filters_build_us = where_filters_build_started.elapsed().as_micros();
            let where_filters_validate_started = std::time::Instant::now();
            let validated_where_filters =
                generate_where_filters_src::domain_types::source::validate_generate_where_filters(
                    built_where_filters,
                )
                .unwrap_or_else(|error| panic!("54b73a29: {error}"));
            let where_filters_validate_us = where_filters_validate_started.elapsed().as_micros();
            let where_filters_emit_started = std::time::Instant::now();
            let staged_where_filters =
                generate_where_filters_src::domain_types::source::emit_generate_where_filters(
                    validated_where_filters,
                );
            let where_filters_emit_us = where_filters_emit_started.elapsed().as_micros();
            println!(
                "measurement=generate_where_filters_typed_stages parse_us={where_filters_parse_us} build_us={where_filters_build_us} validate_us={where_filters_validate_us} emit_us={where_filters_emit_us} output_bytes={}",
                staged_where_filters.to_string().len()
            );
            let generate_where_filters_measurement = (0..crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT).fold(
                (
                    u128::MAX,
                    constants_u128::ZERO,
                    constants_u128::ZERO,
                    constants_usize::ZERO,
                    constants_usize::ZERO,
                ),
                |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                    let started = std::time::Instant::now();
                    let output = generate_where_filters_src::domain_types::source::generate_where_filters(
                        generate_where_filters_src::domain_types::source::ProcMacro2GenerateWhereFiltersInput::from(
                            &generate_where_filters_input_token_stream,
                        ),
                    );
                    let wall_us = started.elapsed().as_micros();
                    (
                        min_wall_us.min(wall_us),
                        max_wall_us.max(wall_us),
                        total_wall_us.saturating_add(wall_us),
                        output.to_string().len(),
                        output.as_ref().clone().into_iter().count(),
                    )
                },
            );
            println!(
                "measurement=generate_where_filters_src repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                generate_where_filters_measurement.0,
                generate_where_filters_measurement.2,
                generate_where_filters_measurement.1,
                generate_where_filters_measurement.3,
                generate_where_filters_measurement.4
            );
            let pg_crud_common_query_part: Result<
                (u128, u128, u128, usize),
                pg_crud_common::domain_types::QueryPartError,
            > =
                (0..crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(
                    (
                        u128::MAX,
                        constants_u128::ZERO,
                        constants_u128::ZERO,
                        constants_usize::ZERO,
                    ),
                    |(min_wall_us, max_wall_us, total_wall_us, _), _| {
                        let started = std::time::Instant::now();
                        let output_bytes = (0..crate::domain_types::MEASURE_REPEAT_COUNT)
                            .try_fold(constants_usize::ZERO, |accumulator, _| {
                                let mut increment = constants_u64::ZERO;
                                match pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                                    &pg_crud_common::domain_types::PaginationBase::default(),
                                    &mut increment,
                                    pg_crud_common::domain_types::SqlColumnRef::from(
                                        &constants_str::COLUMN,
                                    ),
                                    pg_crud_common::domain_types::AddOperator::from(false),
                                ) {
                                    Ok(fragment) => {
                                        Ok(accumulator.saturating_add(fragment.as_ref().len()))
                                    }
                                    Err(error) => Err(error),
                                }
                            })?;
                        let wall_us = started.elapsed().as_micros();
                        Ok((
                            min_wall_us.min(wall_us),
                            max_wall_us.max(wall_us),
                            total_wall_us.saturating_add(wall_us),
                            output_bytes,
                        ))
                    },
                );
            match pg_crud_common_query_part {
                Ok((min_wall_us, max_wall_us, total_wall_us, output_bytes)) => {
                    println!(
                        "measurement=pg_crud_common_query_part series_count={series_count} repeat_count={repeat_count} wall_min_us={min_wall_us} wall_total_us={total_wall_us} wall_max_us={max_wall_us} output_bytes={output_bytes}",
                        series_count = crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT,
                        repeat_count = crate::domain_types::MEASURE_REPEAT_COUNT,
                    );
                }
                Err(error) => {
                    eprintln!(
                        "measurement=pg_crud_common_query_part status=failed error={error:?}"
                    );
                    std::process::exit(1);
                }
            }
            let where_filters_values = (constants_i32::ZERO..64i32).collect::<Vec<i32>>();
            let where_filters_bounded_vec =
                match where_filters::domain_types::BoundedVec::<i32, 64>::try_from(
                    where_filters_values,
                ) {
                    Ok(value) => value,
                    Err(error) => {
                        eprintln!(
                            "measurement=where_filters_query_part status=setup_failed error={error:?}"
                        );
                        std::process::exit(1);
                    }
                };
            let where_filters_query_part: Result<
                (u128, u128, u128, usize),
                pg_crud_common::domain_types::QueryPartError,
            > =
                (0..crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(
                    (
                        u128::MAX,
                        constants_u128::ZERO,
                        constants_u128::ZERO,
                        constants_usize::ZERO,
                    ),
                    |(min_wall_us, max_wall_us, total_wall_us, _), _| {
                        let started = std::time::Instant::now();
                        let output_bytes = (0..crate::domain_types::MEASURE_REPEAT_COUNT)
                            .try_fold(constants_usize::ZERO, |accumulator, _| {
                                let mut increment = constants_u64::ZERO;
                                match where_filters_bounded_vec.pg_type_query_part(
                                    &mut increment,
                                    pg_crud_common::domain_types::SqlColumnRef::from(
                                        &constants_str::COLUMN,
                                    ),
                                    pg_crud_common::domain_types::AddOperator::from(false),
                                ) {
                                    Ok(fragment) => {
                                        Ok(accumulator.saturating_add(fragment.as_ref().len()))
                                    }
                                    Err(error) => Err(error),
                                }
                            })?;
                        let wall_us = started.elapsed().as_micros();
                        Ok((
                            min_wall_us.min(wall_us),
                            max_wall_us.max(wall_us),
                            total_wall_us.saturating_add(wall_us),
                            output_bytes,
                        ))
                    },
                );
            match where_filters_query_part {
                Ok((min_wall_us, max_wall_us, total_wall_us, output_bytes)) => {
                    println!(
                        "measurement=where_filters_query_part series_count={series_count} repeat_count={repeat_count} wall_min_us={min_wall_us} wall_total_us={total_wall_us} wall_max_us={max_wall_us} output_bytes={output_bytes}",
                        series_count = crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT,
                        repeat_count = crate::domain_types::MEASURE_REPEAT_COUNT,
                    );
                    Ok(())
                }
                Err(error) => {
                    eprintln!("measurement=where_filters_query_part status=failed error={error:?}");
                    Err(())
                }
            }
        }
        Some(constants_str::ALL_ALT) => {
            crate::adapters::execution::run_commands(crate::adapters::execution::CommandsRef::from(
                &constants_str::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS,
            ))
            .and_then(|()| run_workspace_tests())
            .and_then(|()| {
                crate::domain_types::macro_generation_measurements()
                    .iter()
                    .try_fold((), |(), (measurement_name, args)| {
                        measure_cargo_command(*measurement_name, *args)
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
