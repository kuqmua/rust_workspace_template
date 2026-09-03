#[test]
fn test_ansi_cleanup_processes_multiple_and_unterminated_sequences() {
    let clean = crate::strip_ansi_codes::strip_ansi_codes(crate::ansi_text_ref::AnsiTextRef::from(
        constants_str::VALUE_22233BC3,
    ));
    assert_eq!(clean.as_ref(), constants_str::VALUE_14D8B1DB);
    assert_eq!(
        crate::strip_ansi_codes::strip_ansi_codes(crate::ansi_text_ref::AnsiTextRef::from(
            constants_str::VALUE_A116C9ED
        ))
        .as_ref(),
        constants_str::VALUE_A116C9ED
    );
}
#[test]
fn test_memusage_parsers_distinguish_values_and_missing_fields() {
    let text = crate::clean_ansi_text::CleanAnsiText::try_from(String::from(
        constants_str::VALUE_D36CD261,
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
        crate::check_tool_available::check_tool_available(crate::tool_path::ToolPath::from(env!(
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

#[test]
fn test_tests_mode_runs_code_style_once() {
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_STYLE_ARGS
            .contains(&constants_str::TESTS_CODE_STYLE_RUST)
    );
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_WORKSPACE_ARGS
            .windows(constants_usize::TWO)
            .any(|args| {
                args == [
                    constants_str::SHARED_VALUES_EXCLUDE,
                    constants_str::TESTS_CODE_STYLE_RUST,
                ]
            })
    );
}
