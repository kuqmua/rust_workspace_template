#[test]
fn ansi_cleanup_handles_multiple_and_unterminated_sequences() {
    let clean = super::strip_ansi_codes(super::AnsiTextRef::from(
        "plain \u{1b}[31mred\u{1b}[0m tail\u{1b}[",
    ));
    assert_eq!(clean.0, "plain red tail");
    assert_eq!(
        super::strip_ansi_codes(super::AnsiTextRef::from("plain")).0,
        "plain"
    );
}
#[test]
fn memusage_parsers_distinguish_values_and_missing_fields() {
    let text = super::CleanAnsiText::try_from(String::from(
        "Heap total: 1,234 bytes\nmalloc | 7 89 0\nfree | 6 78\n",
    ))
    .expect("afa44055 memusage_parsers_distinguish_values_and_missing_fields invariant must hold");
    assert_eq!(
        super::memusage_heap_value(&text, super::MemusageKey::from("Heap total:")).get(),
        "1"
    );
    assert_eq!(
        super::memusage_heap_value(&text, super::MemusageKey::from("Stack peak:")).get(),
        str_constants::UNAVAILABLE
    );
    assert_eq!(
        super::memusage_table_value(
            &text,
            super::MemusageRowName::from("malloc"),
            super::MemusageColumnIdx::from(usize_constants::ONE)
        )
        .get(),
        "89"
    );
    assert_eq!(
        super::memusage_table_value(
            &text,
            super::MemusageRowName::from("calloc"),
            super::MemusageColumnIdx::from(usize_constants::ZERO)
        )
        .get(),
        str_constants::UNAVAILABLE
    );
    assert_eq!(
        super::memusage_table_value(
            &text,
            super::MemusageRowName::from("free"),
            super::MemusageColumnIdx::from(9usize)
        )
        .get(),
        str_constants::UNAVAILABLE
    );
}
#[test]
fn measurement_catalogs_are_complete_and_ordered() {
    let measurements = super::macro_generation_measurements();
    assert_eq!(measurements.len(), 3usize);
    assert_eq!(
        measurements[0].0.get(),
        str_constants::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_MEASUREMENT
    );
    assert_eq!(
        measurements[2].0.get(),
        str_constants::WORKSPACE_TEST_RUNNER_GENERATE_WHERE_FILTERS_MEASUREMENT
    );
    let tools = super::allocation_tools();
    assert_eq!(tools.len(), 6usize);
    assert_eq!(
        tools[0].name.get(),
        str_constants::WORKSPACE_TEST_RUNNER_LIBMEMUSAGE_TOOL
    );
    assert_eq!(tools[5].name.get(), str_constants::PG_CRUD_PG_TIME);
}
#[test]
fn tool_discovery_checks_the_exact_path() {
    assert!(
        super::discovery::tool_available(super::ToolPath::from(env!("CARGO_MANIFEST_DIR"))).get()
    );
    assert!(
        !super::discovery::tool_available(super::ToolPath::from(
            "/definitely/not/a/workspace/tool"
        ))
        .get()
    );
}
#[test]
fn database_mode_runs_the_workspace_ignored_suite() {
    assert!(str_constants::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS.contains(&"--workspace"));
    assert!(
        str_constants::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS.contains(&"--all-features")
    );
    assert!(str_constants::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS.contains(&"--ignored"));
}
#[test]
fn tests_mode_leaves_ignored_suite_to_database_mode() {
    assert!(
        str_constants::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS
            .iter()
            .all(|(_program, args)| !args.contains(&"--ignored"))
    );
    assert!(
        str_constants::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS
            .iter()
            .all(|(_program, args)| !args.contains(&"--run-ignored"))
    );
}
