pub(crate) fn run_measurements_cli() -> crate::runner_cli_outcome::RunnerCliOutcome {
    let render_cargo_measurement_error =
        |cargo_measurement_error: crate::cargo_measurement_error::CargoMeasurementError| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{cargo_measurement_error}{}",
                    constants_str::NEWLINE
                )),
            );
        };
    let render_memusage_measurement_error =
        |memusage_measurement_error: crate::memusage_measurement_error::MemusageMeasurementError| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{memusage_measurement_error}{}",
                    constants_str::NEWLINE
                )),
            );
        };
    let result = (|| {
        let allocation_tools_printed: Result<(), std::convert::Infallible> =
            crate::allocation_tools::allocation_tools()
                .iter()
                .try_fold((), |(), tool| {
                    let available =
                        crate::check_tool_available::check_tool_available(*tool.get_path());
                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                        macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
                        macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                            "{}{}",
                            format_args!(
                                "{}{}{}{}{}{}",
                                constants_str::RUNNER_CLI_TEXT_FCB569D3,
                                tool.get_name().get(),
                                constants_str::RUNNER_CLI_TEXT_3098F3EA,
                                tool.get_path().get(),
                                constants_str::RUNNER_CLI_TEXT_BF147B5B,
                                available.get()
                            ),
                            constants_str::NEWLINE
                        )),
                    );
                    Ok(())
                });
        match allocation_tools_printed {
            Ok(()) => {}
            Err(error) => match error {},
        }
        if std::path::Path::new(constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH).exists() {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!(
                        "{}{}",
                        constants_str::RUNNER_CLI_TEXT_61684F18,
                        constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH
                    ),
                    constants_str::NEWLINE
                )),
            );
            crate::measure_memusage_command::measure_memusage_command(
                crate::measurement_name::MeasurementName::from(constants_str::CODE_STYLE),
                crate::program_path_ref::ProgramPathRef::from(
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                ),
                crate::program_args_ref::ProgramArgsRef::from(&[
                    constants_str::TEST_ALT_3,
                    constants_str::P,
                    constants_str::TESTS_ALT,
                    constants_str::CODE_STYLE,
                ]),
                crate::memusage_prog_name_ref::MemusageProgNameRef::from(
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                ),
            )
            .map_err(render_memusage_measurement_error)
            .unwrap_or_else(|()| std::process::exit(1));
            let current_exe = match std::env::current_exe() {
                Ok(value) => value,
                Err(error) => {
                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                        macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                        macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                            "{}{}",
                            format_args!("{}{}", constants_str::RUNNER_CLI_TEXT_AC15D350, error),
                            constants_str::NEWLINE
                        )),
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
                    crate::measurement_name::MeasurementName::from(
                        constants_str::GENERATE_PG_TABLE_SRC,
                    ),
                    constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_WORKLOAD,
                ),
                (
                    crate::measurement_name::MeasurementName::from(
                        constants_str::GENERATE_PG_TYPES_SRC,
                    ),
                    constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TYPES_WORKLOAD,
                ),
                (
                    crate::measurement_name::MeasurementName::from(
                        constants_str::PG_CRUD_COMMON_QUERY_PART,
                    ),
                    constants_str::WORKSPACE_TEST_RUNNER_PG_CRUD_COMMON_QUERY_PART_WORKLOAD,
                ),
                (
                    crate::measurement_name::MeasurementName::from(
                        constants_str::WHERE_FILTERS_QUERY_PART,
                    ),
                    constants_str::WORKSPACE_TEST_RUNNER_WHERE_FILTERS_QUERY_PART_WORKLOAD,
                ),
            ]
            .into_iter()
            .try_fold((), |(), (measurement_name, workload_mode)| {
                crate::measure_memusage_command::measure_memusage_command(
                    measurement_name,
                    crate::program_path_ref::ProgramPathRef::from(current_exe_string.as_str()),
                    crate::program_args_ref::ProgramArgsRef::from(&[workload_mode]),
                    crate::memusage_prog_name_ref::MemusageProgNameRef::from(current_exe_prog_name),
                )
                .map_err(render_memusage_measurement_error)
            })
            .unwrap_or_else(|()| std::process::exit(1));
        } else {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{}", constants_str::RUNNER_CLI_TEXT_83C7CC56),
                    constants_str::NEWLINE
                )),
            );
        }
        crate::measure_cargo_command::measure_cargo_command(
            crate::measurement_name::MeasurementName::from(constants_str::CODE_STYLE),
            crate::cargo_args::CargoArgs::from(&[
                constants_str::TEST_ALT_3,
                constants_str::P,
                constants_str::TESTS_ALT,
                constants_str::CODE_STYLE,
            ]),
        )
        .map_err(render_cargo_measurement_error)
        .unwrap_or_else(|()| std::process::exit(1));
        crate::measure_cargo_command::measure_cargo_command(
            crate::measurement_name::MeasurementName::from(constants_str::CLIPPY),
            crate::cargo_args::CargoArgs::from(
                &constants_str::WORKSPACE_TEST_RUNNER_CARGO_CLIPPY_ARGS,
            ),
        )
        .map_err(render_cargo_measurement_error)
        .unwrap_or_else(|()| std::process::exit(1));
        let generate_pg_table_input_token_stream =
                    crate::generate_pg_table_measure_input_token_stream::generate_pg_table_measure_input_token_stream(
                        &quote::quote! {"False"},
                    );
        let generate_pg_table_input_with_tests_token_stream =
                    crate::generate_pg_table_measure_input_token_stream::generate_pg_table_measure_input_token_stream(
                        &quote::quote! {"True"},
                    );
        let generate_pg_table_stage_measurement =
            crate::measure_generation_stages::measure_generation_stages(
                macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                    generate_pg_table_input_token_stream.as_ref(),
                ),
                generate_pg_table_src::parse_generate_pg_table::parse_generate_pg_table,
                generate_pg_table_src::build_generate_pg_table::build_generate_pg_table,
                generate_pg_table_src::validate_generate_pg_table::validate_generate_pg_table,
                generate_pg_table_src::emit_generate_pg_table::emit_generate_pg_table,
                |output| output.to_string().len(),
            )
            .unwrap_or_else(|error| {
                std::panic::panic_any(constants_str::PANIC_D6399CBF.replacen(
                    constants_str::PANIC_PLACEHOLDER_81240055,
                    error.to_string().as_str(),
                    1usize,
                ))
            });
        macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
            macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                "{}{}",
                format_args!(
                    "{}{}{}{}{}{}{}{}{}{}",
                    constants_str::RUNNER_CLI_TEXT_C0746021,
                    generate_pg_table_stage_measurement.get_parse_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_A5E71122,
                    generate_pg_table_stage_measurement.get_build_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_765A69E6,
                    generate_pg_table_stage_measurement.get_validate_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_95584412,
                    generate_pg_table_stage_measurement.get_emit_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_226B5908,
                    generate_pg_table_stage_measurement.get_output_bytes()
                ),
                constants_str::NEWLINE
            )),
        );
        let generate_pg_table_measurement =
            crate::measure_direct_generation::measure_direct_generation(
                || {
                    generate_pg_table_src::generate_pg_table::generate_pg_table(
                        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                            generate_pg_table_input_token_stream.as_ref(),
                        ),
                    )
                },
                |output| {
                    crate::direct_generation_output_measurement::DirectGenerationOutputMeasurement::new(
                        output.to_string().len(),
                        output.as_ref().clone().into_iter().count(),
                    )
                },
            );
        macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
            macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                "{}{}",
                format_args!(
                    "{}{}{}{}{}{}{}{}{}{}{}{}",
                    constants_str::RUNNER_CLI_TEXT_251828F4,
                    crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                    constants_str::RUNNER_CLI_TEXT_227E2FDB,
                    generate_pg_table_measurement.get_minimum_wall_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_986DA580,
                    generate_pg_table_measurement.get_total_wall_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_3E2272C4,
                    generate_pg_table_measurement.get_maximum_wall_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_226B5908,
                    generate_pg_table_measurement.get_output_bytes(),
                    constants_str::RUNNER_CLI_TEXT_26762D92,
                    generate_pg_table_measurement.get_output_token_trees()
                ),
                constants_str::NEWLINE
            )),
        );
        let generate_pg_table_with_tests_dir =
            std::path::Path::new(constants_str::TARGET_MEASURE_GENERATE_PG_TABLE_WITH_TESTS);
        if let Err(error) = std::fs::create_dir_all(generate_pg_table_with_tests_dir) {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{}{}", constants_str::RUNNER_CLI_TEXT_C8CCFAD7, error),
                    constants_str::NEWLINE
                )),
            );
            std::process::exit(1);
        }
        if let Err(error) = std::fs::write(
            generate_pg_table_with_tests_dir.join(constants_str::RUSTFMT_TOML),
            constants_str::EDITION_2024_NEWLINE,
        ) {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{}{}", constants_str::RUNNER_CLI_TEXT_9302A07B, error),
                    constants_str::NEWLINE
                )),
            );
            std::process::exit(1);
        }
        let current_dir = match std::env::current_dir() {
            Ok(value) => value,
            Err(error) => {
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!("{}{}", constants_str::RUNNER_CLI_TEXT_58F8DDDC, error),
                        constants_str::NEWLINE
                    )),
                );
                std::process::exit(1);
            }
        };
        if let Err(error) = std::env::set_current_dir(generate_pg_table_with_tests_dir) {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{}{}", constants_str::RUNNER_CLI_TEXT_ABA7CD78, error),
                    constants_str::NEWLINE
                )),
            );
            std::process::exit(1);
        }
        let generate_pg_table_with_tests_measurement =
            crate::measure_direct_generation::measure_direct_generation(
                || {
                    generate_pg_table_src::generate_pg_table::generate_pg_table(
                        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                            generate_pg_table_input_with_tests_token_stream.as_ref(),
                        ),
                    )
                },
                |output| {
                    crate::direct_generation_output_measurement::DirectGenerationOutputMeasurement::new(
                        output.to_string().len(),
                        output.as_ref().clone().into_iter().count(),
                    )
                },
            );
        if let Err(error) = std::env::set_current_dir(current_dir) {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{}{}", constants_str::RUNNER_CLI_TEXT_0F58572C, error),
                    constants_str::NEWLINE
                )),
            );
            std::process::exit(1);
        }
        let generate_pg_table_tests_stage_output_path =
            generate_pg_table_with_tests_dir.join(constants_str::GENERATE_PG_TABLE_TESTS_RS);
        let generate_pg_table_tests_stage_output =
            match server_runtime_http::read_bounded_file::read_bounded_file(
                server_runtime_http::runtime_path_ref::RuntimePathRef::from(
                    generate_pg_table_tests_stage_output_path.as_path(),
                ),
                server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
                    constants_usize::VALUE_16_777_216,
                ),
            )
            .and_then(server_runtime_http::bounded_text::BoundedText::try_from)
            {
                Ok(content) => (content.as_ref().len(), content.as_ref().lines().count()),
                Err(error) => {
                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                        macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                        macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                            "{}{}",
                            format_args!("{}{}", constants_str::RUNNER_CLI_TEXT_04C53EE4, error),
                            constants_str::NEWLINE
                        )),
                    );
                    std::process::exit(1);
                }
            };
        macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
            macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                "{}{}",
                format_args!(
                    "{}{}{}{}{}{}{}{}{}{}{}{}",
                    constants_str::RUNNER_CLI_TEXT_FCEF5963,
                    crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                    constants_str::RUNNER_CLI_TEXT_227E2FDB,
                    generate_pg_table_with_tests_measurement.get_minimum_wall_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_986DA580,
                    generate_pg_table_with_tests_measurement.get_total_wall_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_3E2272C4,
                    generate_pg_table_with_tests_measurement.get_maximum_wall_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_226B5908,
                    generate_pg_table_with_tests_measurement.get_output_bytes(),
                    constants_str::RUNNER_CLI_TEXT_26762D92,
                    generate_pg_table_with_tests_measurement.get_output_token_trees()
                ),
                constants_str::NEWLINE
            )),
        );
        macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
            macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                "{}{}",
                format_args!(
                    "{}{}{}{}",
                    constants_str::RUNNER_CLI_TEXT_B1219889,
                    generate_pg_table_tests_stage_output.0,
                    constants_str::RUNNER_CLI_TEXT_61022FDB,
                    generate_pg_table_tests_stage_output.1
                ),
                constants_str::NEWLINE
            )),
        );
        macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
            macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                "{}{}",
                format_args!(
                    "{}{}{}{}{}{}{}{}{}{}",
                    constants_str::RUNNER_CLI_TEXT_BB491181,
                    crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                    constants_str::RUNNER_CLI_TEXT_08C0CDCD,
                    generate_pg_table_with_tests_measurement
                        .get_total_wall_microseconds()
                        .saturating_sub(
                            *generate_pg_table_measurement.get_total_wall_microseconds()
                        ),
                    constants_str::RUNNER_CLI_TEXT_E6FE696A,
                    generate_pg_table_with_tests_measurement
                        .get_minimum_wall_microseconds()
                        .saturating_sub(
                            *generate_pg_table_measurement.get_minimum_wall_microseconds()
                        ),
                    constants_str::RUNNER_CLI_TEXT_10C3D59C,
                    generate_pg_table_with_tests_measurement
                        .get_maximum_wall_microseconds()
                        .saturating_sub(
                            *generate_pg_table_measurement.get_maximum_wall_microseconds()
                        ),
                    constants_str::RUNNER_CLI_TEXT_E16A49EB,
                    generate_pg_table_with_tests_measurement
                        .get_output_bytes()
                        .saturating_sub(*generate_pg_table_measurement.get_output_bytes())
                ),
                constants_str::NEWLINE
            )),
        );
        let generate_pg_types_input_token_stream = quote::quote! {
            {
                "pg_table_cols_write_into_file": "False",
                "whole_write_into_file": "False",
                "variant": "All"
            }
        };
        let generate_pg_types_stage_measurement =
            crate::measure_generation_stages::measure_generation_stages(
                macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                    &generate_pg_types_input_token_stream,
                ),
                generate_pg_types_src::parse_generate_pg_types::parse_generate_pg_types,
                generate_pg_types_src::build_generate_pg_types::build_generate_pg_types,
                generate_pg_types_src::validate_generate_pg_types::validate_generate_pg_types,
                generate_pg_types_src::emit_generate_pg_types::emit_generate_pg_types,
                |output| output.to_string().len(),
            )
            .unwrap_or_else(|error| {
                std::panic::panic_any(constants_str::PANIC_A19C725E.replacen(
                    constants_str::PANIC_PLACEHOLDER_81240055,
                    error.to_string().as_str(),
                    1usize,
                ))
            });
        macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
            macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                "{}{}",
                format_args!(
                    "{}{}{}{}{}{}{}{}{}{}",
                    constants_str::RUNNER_CLI_TEXT_D33C82E2,
                    generate_pg_types_stage_measurement.get_parse_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_A5E71122,
                    generate_pg_types_stage_measurement.get_build_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_765A69E6,
                    generate_pg_types_stage_measurement.get_validate_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_95584412,
                    generate_pg_types_stage_measurement.get_emit_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_226B5908,
                    generate_pg_types_stage_measurement.get_output_bytes()
                ),
                constants_str::NEWLINE
            )),
        );
        let generate_pg_types_measurement =
            crate::measure_direct_generation::measure_direct_generation(
                || {
                    generate_pg_types_src::generate_pg_types_tokens::generate_pg_types_tokens(
                        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                            &generate_pg_types_input_token_stream,
                        ),
                    )
                },
                |output| {
                    crate::direct_generation_output_measurement::DirectGenerationOutputMeasurement::new(
                        output.to_string().len(),
                        output.as_ref().clone().into_iter().count(),
                    )
                },
            );
        macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
            macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                "{}{}",
                format_args!(
                    "{}{}{}{}{}{}{}{}{}{}{}{}",
                    constants_str::RUNNER_CLI_TEXT_1FD0C7EF,
                    crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                    constants_str::RUNNER_CLI_TEXT_227E2FDB,
                    generate_pg_types_measurement.get_minimum_wall_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_986DA580,
                    generate_pg_types_measurement.get_total_wall_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_3E2272C4,
                    generate_pg_types_measurement.get_maximum_wall_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_226B5908,
                    generate_pg_types_measurement.get_output_bytes(),
                    constants_str::RUNNER_CLI_TEXT_26762D92,
                    generate_pg_types_measurement.get_output_token_trees()
                ),
                constants_str::NEWLINE
            )),
        );
        let generate_where_filters_input_token_stream = quote::quote! {
            {
                "pg_types_write_into_file": "False",
                "whole_write_into_file": "False"
            }
        };
        let generate_where_filters_stage_measurement =
            crate::measure_generation_stages::measure_generation_stages(
                            generate_where_filters_src::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(
                                &generate_where_filters_input_token_stream,
                            ),
                generate_where_filters_src::parse_generate_where_filters::parse_generate_where_filters,
                generate_where_filters_src::build_generate_where_filters::build_generate_where_filters,
                generate_where_filters_src::validate_generate_where_filters::validate_generate_where_filters,
                generate_where_filters_src::emit_generate_where_filters::emit_generate_where_filters,
                |output| output.to_string().len(),
            )
                        .unwrap_or_else(|error| std::panic::panic_any(constants_str::PANIC_8F246DC1.replacen(constants_str::PANIC_PLACEHOLDER_81240055, error.to_string().as_str(), 1usize)));
        macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
            macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                "{}{}",
                format_args!(
                    "{}{}{}{}{}{}{}{}{}{}",
                    constants_str::RUNNER_CLI_TEXT_623915E8,
                    generate_where_filters_stage_measurement.get_parse_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_A5E71122,
                    generate_where_filters_stage_measurement.get_build_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_765A69E6,
                    generate_where_filters_stage_measurement.get_validate_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_95584412,
                    generate_where_filters_stage_measurement.get_emit_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_226B5908,
                    generate_where_filters_stage_measurement.get_output_bytes()
                ),
                constants_str::NEWLINE
            )),
        );
        let generate_where_filters_measurement =
            crate::measure_direct_generation::measure_direct_generation(
                || {
                    generate_where_filters_src::generate_where_filters_source::generate_where_filters_source(
                        generate_where_filters_src::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(
                            &generate_where_filters_input_token_stream,
                        ),
                    )
                },
                |output| {
                    crate::direct_generation_output_measurement::DirectGenerationOutputMeasurement::new(
                        output.to_string().len(),
                        output.as_ref().clone().into_iter().count(),
                    )
                },
            );
        macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
            macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                "{}{}",
                format_args!(
                    "{}{}{}{}{}{}{}{}{}{}{}{}",
                    constants_str::RUNNER_CLI_TEXT_DBC31E58,
                    crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                    constants_str::RUNNER_CLI_TEXT_227E2FDB,
                    generate_where_filters_measurement.get_minimum_wall_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_986DA580,
                    generate_where_filters_measurement.get_total_wall_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_3E2272C4,
                    generate_where_filters_measurement.get_maximum_wall_microseconds(),
                    constants_str::RUNNER_CLI_TEXT_226B5908,
                    generate_where_filters_measurement.get_output_bytes(),
                    constants_str::RUNNER_CLI_TEXT_26762D92,
                    generate_where_filters_measurement.get_output_token_trees()
                ),
                constants_str::NEWLINE
            )),
        );
        let pg_crud_common_query_part: Result<
            (u128, u128, u128, usize),
            pg_crud_common::query_part_error::QueryPartError,
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
                        match pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                            &pg_crud_common::pagination_base::PaginationBase::default(),
                            &mut increment,
                            pg_crud_common::sql_column_ref::SqlColumnRef::from(
                                &constants_str::COLUMN,
                            ),
                            pg_crud_common::add_operator::AddOperator::from(false),
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
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!(
                            "{}{}{}{}{}{}{}{}{}{}{}{}",
                            constants_str::RUNNER_CLI_TEXT_B2120D92,
                            crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT,
                            constants_str::RUNNER_CLI_TEXT_333D0B67,
                            crate::domain_types::MEASURE_REPEAT_COUNT,
                            constants_str::RUNNER_CLI_TEXT_227E2FDB,
                            min_wall_us,
                            constants_str::RUNNER_CLI_TEXT_986DA580,
                            total_wall_us,
                            constants_str::RUNNER_CLI_TEXT_3E2272C4,
                            max_wall_us,
                            constants_str::RUNNER_CLI_TEXT_226B5908,
                            output_bytes
                        ),
                        constants_str::NEWLINE
                    )),
                );
            }
            Err(error) => {
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!("{}{:?}", constants_str::RUNNER_CLI_TEXT_122D1700, error),
                        constants_str::NEWLINE
                    )),
                );
                std::process::exit(1);
            }
        }
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
                            format_args!("{}{:?}", constants_str::RUNNER_CLI_TEXT_ED029694, error),
                            constants_str::NEWLINE
                        )),
                    );
                    std::process::exit(1);
                }
            };
        let where_filters_query_part: Result<
            (u128, u128, u128, usize),
            pg_crud_common::query_part_error::QueryPartError,
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
                            pg_crud_common::sql_column_ref::SqlColumnRef::from(
                                &constants_str::COLUMN,
                            ),
                            pg_crud_common::add_operator::AddOperator::from(false),
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
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!(
                            "{}{}{}{}{}{}{}{}{}{}{}{}",
                            constants_str::RUNNER_CLI_TEXT_360892CE,
                            crate::domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT,
                            constants_str::RUNNER_CLI_TEXT_333D0B67,
                            crate::domain_types::MEASURE_REPEAT_COUNT,
                            constants_str::RUNNER_CLI_TEXT_227E2FDB,
                            min_wall_us,
                            constants_str::RUNNER_CLI_TEXT_986DA580,
                            total_wall_us,
                            constants_str::RUNNER_CLI_TEXT_3E2272C4,
                            max_wall_us,
                            constants_str::RUNNER_CLI_TEXT_226B5908,
                            output_bytes
                        ),
                        constants_str::NEWLINE
                    )),
                );
                Ok(())
            }
            Err(error) => {
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!("{}{:?}", constants_str::RUNNER_CLI_TEXT_9A443735, error),
                        constants_str::NEWLINE
                    )),
                );
                Err(())
            }
        }
    })();
    match result {
        Ok(()) => crate::runner_cli_outcome::RunnerCliOutcome::Completed,
        Err(()) => crate::runner_cli_outcome::RunnerCliOutcome::Failed,
    }
}
