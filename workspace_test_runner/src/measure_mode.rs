#[allow(
    clippy::panic_in_result_fn,
    reason = "the measurement fixture treats invalid compile-time generator inputs as diagnostic invariant failures"
)]
pub(crate) fn measure_mode() -> Result<(), ()> {
    {
        let allocation_tools_printed: Result<(), std::convert::Infallible> =
            crate::domain_types::allocation_tools()
                .iter()
                .try_fold((), |(), tool| {
                    let available = crate::check_tool_available::check_tool_available(tool.path());
                    println!(
                        "measurement=allocation_tool_available tool={} path={} available={}",
                        tool.name().get(),
                        tool.path().get(),
                        available.get()
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
            super::measure_memusage_command::measure_memusage_command(
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
                super::measure_memusage_command::measure_memusage_command(
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
        super::measure_cargo_command::measure_cargo_command(
            crate::domain_types::MeasurementName::from(constants_str::CODE_STYLE),
            crate::domain_types::CargoArgs::from(&[
                constants_str::TEST_ALT_3,
                constants_str::P,
                constants_str::TESTS_ALT,
                constants_str::CODE_STYLE,
            ]),
        )
        .unwrap_or_else(|()| std::process::exit(1));
        super::measure_cargo_command::measure_cargo_command(
            crate::domain_types::MeasurementName::from(constants_str::CLIPPY),
            crate::domain_types::CargoArgs::from(
                &constants_str::WORKSPACE_TEST_RUNNER_CARGO_CLIPPY_ARGS,
            ),
        )
        .unwrap_or_else(|()| std::process::exit(1));
        let generate_pg_table_input_token_stream =
            super::generate_pg_table_measure_input_token_stream::generate_pg_table_measure_input_token_stream(
                &quote::quote! {"False"},
            );
        let generate_pg_table_input_with_tests_token_stream =
            super::generate_pg_table_measure_input_token_stream::generate_pg_table_measure_input_token_stream(
                &quote::quote! {"True"},
            );
        let parse_started = std::time::Instant::now();
        let parsed = generate_pg_table_src::domain_types::pipeline::parse_generate_pg_table(
            macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(
                generate_pg_table_input_token_stream.as_ref(),
            ),
        )
        .unwrap_or_else(|error| panic!("d6399cbf: {error}"));
        let parse_us = parse_started.elapsed().as_micros();
        let build_started = std::time::Instant::now();
        let built = generate_pg_table_src::domain_types::pipeline::build_generate_pg_table(parsed)
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
        let parsed_pg_types = generate_pg_types_src::domain_types::source::parse_generate_pg_types(
            macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(
                &generate_pg_types_input_token_stream,
            ),
        )
        .unwrap_or_else(|error| panic!("a19c725e: {error}"));
        let pg_types_parse_us = pg_types_parse_started.elapsed().as_micros();
        let pg_types_build_started = std::time::Instant::now();
        let built_pg_types =
            generate_pg_types_src::domain_types::source::build_generate_pg_types(parsed_pg_types)
                .unwrap_or_else(|error| panic!("c47612bd: {error}"));
        let pg_types_build_us = pg_types_build_started.elapsed().as_micros();
        let pg_types_validate_started = std::time::Instant::now();
        let validated_pg_types =
            generate_pg_types_src::domain_types::source::validate_generate_pg_types(built_pg_types)
                .unwrap_or_else(|error| panic!("d3e581a4: {error}"));
        let pg_types_validate_us = pg_types_validate_started.elapsed().as_micros();
        let pg_types_emit_started = std::time::Instant::now();
        let staged_pg_types =
            generate_pg_types_src::domain_types::source::emit_generate_pg_types(validated_pg_types);
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
                    let output =
                        generate_pg_types_src::domain_types::source::generate_pg_types_tokens(
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
                        let output = generate_where_filters_src::domain_types::source::generate_where_filters_source(
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
        > = (0..crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(
            (
                u128::MAX,
                constants_u128::ZERO,
                constants_u128::ZERO,
                constants_usize::ZERO,
            ),
            |(min_wall_us, max_wall_us, total_wall_us, _), _| {
                let started = std::time::Instant::now();
                let output_bytes = (0..crate::domain_types::MEASURE_REPEAT_COUNT).try_fold(
                    constants_usize::ZERO,
                    |accumulator, _| {
                        let mut increment = constants_u64::ZERO;
                        match pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                            &pg_crud_common::domain_types::PaginationBase::default(),
                            &mut increment,
                            pg_crud_common::domain_types::SqlColumnRef::from(
                                &constants_str::COLUMN,
                            ),
                            pg_crud_common::domain_types::AddOperator::from(false),
                        ) {
                            Ok(fragment) => Ok(accumulator.saturating_add(fragment.as_ref().len())),
                            Err(error) => Err(error),
                        }
                    },
                )?;
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
                eprintln!("measurement=pg_crud_common_query_part status=failed error={error:?}");
                std::process::exit(1);
            }
        }
        let where_filters_values = (constants_i32::ZERO..64i32).collect::<Vec<i32>>();
        let where_filters_bounded_vec =
            match where_filters::domain_types::BoundedVec::<i32, 64>::try_from(where_filters_values)
            {
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
        > = (0..crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(
            (
                u128::MAX,
                constants_u128::ZERO,
                constants_u128::ZERO,
                constants_usize::ZERO,
            ),
            |(min_wall_us, max_wall_us, total_wall_us, _), _| {
                let started = std::time::Instant::now();
                let output_bytes = (0..crate::domain_types::MEASURE_REPEAT_COUNT).try_fold(
                    constants_usize::ZERO,
                    |accumulator, _| {
                        let mut increment = constants_u64::ZERO;
                        match where_filters_bounded_vec.pg_type_query_part(
                            &mut increment,
                            pg_crud_common::domain_types::SqlColumnRef::from(
                                &constants_str::COLUMN,
                            ),
                            pg_crud_common::domain_types::AddOperator::from(false),
                        ) {
                            Ok(fragment) => Ok(accumulator.saturating_add(fragment.as_ref().len())),
                            Err(error) => Err(error),
                        }
                    },
                )?;
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
}
