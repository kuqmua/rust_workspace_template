#[test]
fn test_bounded_capture_fits_command_log_after_utf8_replacement() {
    let stream_limit = constants_usize::VALUE_1_048_576 * constants_usize::TWO;
    let invalid_bytes = vec![u8::MAX; stream_limit];
    let text = String::from_utf8_lossy(invalid_bytes.as_slice());
    let log = format!("{text}{text}");
    assert!(
        crate::command_text::CommandText::try_from(log).is_ok_and(|command_text| {
            command_text.as_ref().len()
                == stream_limit * constants_usize::TWO * constants_usize::THREE
        })
    );
}

#[test]
fn test_ansi_adapters_preserve_distinct_length_error_fallbacks() {
    let maximum = constants_usize::VALUE_16_777_216;
    let length = maximum + constants_usize::ONE;
    let input = constants_str::X.repeat(length);
    let command_text =
        crate::strip_ansi::strip_ansi(macro_helpers::tool_ansi_chars::ToolAnsiChars::from(
            macro_helpers::tool_ansi_text_ref::ToolAnsiTextRef::from(input.as_str()),
        ));
    let clean_ansi_text = crate::strip_ansi_codes::strip_ansi_codes(
        macro_helpers::tool_ansi_chars::ToolAnsiChars::from(
            macro_helpers::tool_ansi_text_ref::ToolAnsiTextRef::from(input.as_str()),
        ),
    );
    assert_eq!(
        command_text.as_ref(),
        crate::command_text::CommandText::from(
            crate::command_text::CommandTextTryFromStringError::TooLong {
                len: length,
                max: maximum
            },
        )
        .as_ref()
    );
    assert_eq!(
        clean_ansi_text.as_ref(),
        crate::clean_ansi_text::CleanAnsiText::from(
            crate::clean_ansi_text::CleanAnsiTextTryFromStringError::TooLong {
                len: length,
                max: maximum
            },
        )
        .as_ref()
    );
}
#[test]
fn test_memusage_program_text_preserves_output_without_footer() {
    let result =
        crate::clean_ansi_text::CleanAnsiText::try_from(String::from(constants_str::VALUE_1));
    assert!(result.is_ok_and(|clean| {
        crate::memusage_program_text::memusage_program_text(&clean).get() == constants_str::VALUE_1
    }));
}

#[test]
fn test_memusage_program_text_keeps_lines_before_final_summary() {
    let input = [
        constants_str::VALUE_1,
        constants_str::MEMORY_USAGE_SUMMARY,
        constants_str::VALUE_2,
        constants_str::MEMORY_USAGE_SUMMARY,
        constants_str::VALUE_4,
    ]
    .join(constants_str::NEWLINE);
    let result = crate::clean_ansi_text::CleanAnsiText::try_from(input);
    assert!(result.is_ok_and(|clean| {
        crate::memusage_program_text::memusage_program_text(&clean)
            .get()
            .lines()
            .eq([
                constants_str::VALUE_1,
                constants_str::MEMORY_USAGE_SUMMARY,
                constants_str::VALUE_2,
            ])
    }));
}

#[test]
fn test_memusage_parsers_require_summary_marker() {
    let result = crate::clean_ansi_text::CleanAnsiText::try_from(format!(
        "{}{}{}{}{}",
        constants_str::HEAP_TOTAL,
        constants_str::SPACE,
        constants_str::VALUE_1,
        constants_str::NEWLINE,
        constants_str::MALLOC,
    ));
    assert!(result.is_ok_and(|text| {
        crate::memusage_heap_value::memusage_heap_value(
            &text,
            crate::memusage_key::MemusageKey::from(constants_str::HEAP_TOTAL),
        )
        .get()
            == constants_str::UNAVAILABLE
            && crate::memusage_table_value::memusage_table_value(
                &text,
                crate::memusage_row_name::MemusageRowName::from(constants_str::MALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(
                    constants_usize::ZERO,
                ),
            )
            .get()
                == constants_str::UNAVAILABLE
    }));
}

#[test]
fn test_memusage_parsers_ignore_program_stderr_before_summary() {
    let text = [
        format!(
            "{}{}{}",
            constants_str::HEAP_TOTAL,
            constants_str::SPACE,
            constants_str::VALUE_1
        ),
        format!(
            "{}{}{}",
            constants_str::MALLOC,
            constants_str::SPACE,
            constants_str::VALUE_1
        ),
        format!(
            "{}{}{}{}{}",
            constants_str::MEMORY_USAGE_SUMMARY,
            constants_str::SPACE,
            constants_str::HEAP_TOTAL,
            constants_str::SPACE,
            constants_str::VALUE_2
        ),
        format!(
            "{}{}{}",
            constants_str::MALLOC,
            constants_str::SPACE,
            constants_str::VALUE_4
        ),
    ]
    .join(constants_str::NEWLINE);
    let result = crate::clean_ansi_text::CleanAnsiText::try_from(text);
    assert!(result.is_ok_and(|clean| {
        crate::memusage_heap_value::memusage_heap_value(
            &clean,
            crate::memusage_key::MemusageKey::from(constants_str::HEAP_TOTAL),
        )
        .get()
            == constants_str::VALUE_2
            && crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::MALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(
                    constants_usize::ZERO,
                ),
            )
            .get()
                == constants_str::VALUE_4
    }));
}

#[test]
fn test_memusage_parsers_distinguish_values_and_missing_fields() {
    let text = crate::clean_ansi_text::CleanAnsiText::try_from(format!(
        "{}{}{}",
        constants_str::MEMORY_USAGE_SUMMARY,
        constants_str::NEWLINE,
        constants_str::VALUE_D36CD261
    ))
    .expect(constants_str::DIAGNOSTIC_AFA44055);
    assert_eq!(
        crate::memusage_heap_value::memusage_heap_value(
            &text,
            crate::memusage_key::MemusageKey::from(constants_str::VALUE_557B66DC)
        )
        .get(),
        constants_str::VALUE_1
    );
    assert_eq!(
        crate::memusage_heap_value::memusage_heap_value(
            &text,
            crate::memusage_key::MemusageKey::from(constants_str::VALUE_9164CD33)
        )
        .get(),
        constants_str::UNAVAILABLE
    );
    assert_eq!(
        crate::memusage_table_value::memusage_table_value(
            &text,
            crate::memusage_row_name::MemusageRowName::from(constants_str::VALUE_E3C52EBF),
            crate::memory_usage_column_index::MemoryUsageColumnIndex::from(constants_usize::ONE)
        )
        .get(),
        constants_str::VALUE_CD70BEA0
    );
    assert_eq!(
        crate::memusage_table_value::memusage_table_value(
            &text,
            crate::memusage_row_name::MemusageRowName::from(constants_str::VALUE_30EBF387),
            crate::memory_usage_column_index::MemoryUsageColumnIndex::from(constants_usize::ZERO)
        )
        .get(),
        constants_str::UNAVAILABLE
    );
    assert_eq!(
        crate::memusage_table_value::memusage_table_value(
            &text,
            crate::memusage_row_name::MemusageRowName::from(constants_str::VALUE_AD95D5FA),
            crate::memory_usage_column_index::MemoryUsageColumnIndex::from(9usize)
        )
        .get(),
        constants_str::UNAVAILABLE
    );
}
#[test]
fn test_measurement_catalogs_are_complete_and_ordered() {
    let measurements = crate::macro_generation_measurements::macro_generation_measurements();
    assert_eq!(measurements.len(), 3usize);
    assert_eq!(
        measurements[0].0.get(),
        constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_MEASUREMENT
    );
    assert_eq!(
        measurements[2].0.get(),
        constants_str::WORKSPACE_TEST_RUNNER_GENERATE_WHERE_FILTERS_MEASUREMENT
    );
    let tools = crate::allocation_tools::allocation_tools();
    assert_eq!(tools.len(), 6usize);
    assert_eq!(
        tools[0].get_name().get(),
        constants_str::WORKSPACE_TEST_RUNNER_LIBMEMUSAGE_TOOL
    );
    assert_eq!(tools[5].get_name().get(), constants_str::PG_CRUD_PG_TIME);
}
#[test]
fn test_tool_discovery_checks_the_exact_path() {
    assert!(
        !crate::check_tool_available::check_tool_available(crate::tool_path::ToolPath::from(env!(
            "CARGO_MANIFEST_DIR"
        )))
        .get()
    );
    assert!(
        !crate::check_tool_available::check_tool_available(crate::tool_path::ToolPath::from(
            constants_str::VALUE_54283E25
        ))
        .get()
    );
}
#[test]
fn test_database_mode_runs_the_workspace_ignored_suite() {
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS
            .contains(&constants_str::SHARED_VALUES_WORKSPACE)
    );
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS
            .contains(&constants_str::SHARED_VALUES_ALL_FEATURES)
    );
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS
            .contains(&constants_str::SHARED_VALUES_IGNORED)
    );
}
#[test]
fn test_tests_mode_leaves_ignored_suite_to_database_mode() {
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS
            .iter()
            .all(|(_program, args)| !args.contains(&constants_str::SHARED_VALUES_IGNORED))
    );
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS
            .iter()
            .all(|(_program, args)| !args.contains(&constants_str::SHARED_VALUES_RUN_IGNORED))
    );
}
