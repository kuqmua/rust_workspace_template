#[test]
fn ansi_cleanup_processes_multiple_and_unterminated_sequences() {
    let clean = crate::strip_ansi_codes::strip_ansi_codes(crate::ansi_text_ref::AnsiTextRef::from(
        constants_str::test_fixtures::VALUE_22233BC3,
    ));
    assert_eq!(clean.as_ref(), "plain red tail");
    assert_eq!(
        crate::strip_ansi_codes::strip_ansi_codes(crate::ansi_text_ref::AnsiTextRef::from("plain"))
            .as_ref(),
        "plain"
    );
}
#[test]
fn memusage_parsers_distinguish_values_and_missing_fields() {
    let text = crate::clean_ansi_text::CleanAnsiText::try_from(String::from(
        constants_str::test_fixtures::VALUE_D36CD261,
    ))
    .expect("afa44055 memusage_parsers_distinguish_values_and_missing_fields invariant must hold");
    assert_eq!(
        crate::memusage_heap_value::memusage_heap_value(
            &text,
            crate::memusage_key::MemusageKey::from("Heap total:")
        )
        .get(),
        "1"
    );
    assert_eq!(
        crate::memusage_heap_value::memusage_heap_value(
            &text,
            crate::memusage_key::MemusageKey::from("Stack peak:")
        )
        .get(),
        constants_str::catalog::UNAVAILABLE
    );
    assert_eq!(
        crate::memusage_table_value::memusage_table_value(
            &text,
            crate::memusage_row_name::MemusageRowName::from("malloc"),
            crate::memusage_column_idx::MemusageColumnIdx::from(constants_usize::ONE)
        )
        .get(),
        "89"
    );
    assert_eq!(
        crate::memusage_table_value::memusage_table_value(
            &text,
            crate::memusage_row_name::MemusageRowName::from("calloc"),
            crate::memusage_column_idx::MemusageColumnIdx::from(constants_usize::ZERO)
        )
        .get(),
        constants_str::catalog::UNAVAILABLE
    );
    assert_eq!(
        crate::memusage_table_value::memusage_table_value(
            &text,
            crate::memusage_row_name::MemusageRowName::from("free"),
            crate::memusage_column_idx::MemusageColumnIdx::from(9usize)
        )
        .get(),
        constants_str::catalog::UNAVAILABLE
    );
}
#[test]
fn measurement_catalogs_are_complete_and_ordered() {
    let measurements = crate::macro_generation_measurements::macro_generation_measurements();
    assert_eq!(measurements.len(), 3usize);
    assert_eq!(
        measurements[0].0.get(),
        constants_str::catalog::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_MEASUREMENT
    );
    assert_eq!(
        measurements[2].0.get(),
        constants_str::catalog::WORKSPACE_TEST_RUNNER_GENERATE_WHERE_FILTERS_MEASUREMENT
    );
    let tools = crate::allocation_tools::allocation_tools();
    assert_eq!(tools.len(), 6usize);
    assert_eq!(
        tools[0].get_name().get(),
        constants_str::catalog::WORKSPACE_TEST_RUNNER_LIBMEMUSAGE_TOOL
    );
    assert_eq!(
        tools[5].get_name().get(),
        constants_str::catalog::PG_CRUD_PG_TIME
    );
}
#[test]
fn tool_discovery_checks_the_exact_path() {
    assert!(
        crate::check_tool_available::check_tool_available(crate::tool_path::ToolPath::from(env!(
            "CARGO_MANIFEST_DIR"
        )))
        .get()
    );
    assert!(
        !crate::check_tool_available::check_tool_available(crate::tool_path::ToolPath::from(
            "/definitely/not/a/workspace/tool"
        ))
        .get()
    );
}
#[test]
fn database_mode_runs_the_workspace_ignored_suite() {
    assert!(constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS.contains(&"--workspace"));
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS.contains(&"--all-features")
    );
    assert!(constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS.contains(&"--ignored"));
}
#[test]
fn tests_mode_leaves_ignored_suite_to_database_mode() {
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS
            .iter()
            .all(|(_program, args)| !args.contains(&"--ignored"))
    );
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS
            .iter()
            .all(|(_program, args)| !args.contains(&"--run-ignored"))
    );
}

#[test]
fn tests_mode_runs_code_style_once() {
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_STYLE_ARGS
            .contains(&constants_str::catalog::TESTS_CODE_STYLE)
    );
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_WORKSPACE_ARGS
            .windows(constants_usize::TWO)
            .any(|args| {
                args == [
                    constants_str::catalog::SHARED_VALUES_EXCLUDE,
                    constants_str::catalog::TESTS_CODE_STYLE,
                ]
            })
    );
}
