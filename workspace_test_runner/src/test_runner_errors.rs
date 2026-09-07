fn runner_spawn_failure_fixture(
    command_index: crate::command_index::CommandIndex,
) -> crate::command_failure::CommandFailure {
    crate::command_failure::CommandFailure::Spawn {
        command_index,
        execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError::from(
            std::io::Error::from(std::io::ErrorKind::NotFound),
        ),
    }
}

#[test]
fn test_runner_summary_rejects_overflow_without_changing_existing_text() {
    let mut summary = crate::summary_text::SummaryText::default();
    let input = constants_str::X.repeat(constants_usize::VALUE_1_048_576);
    summary
        .push_str(crate::text_ref::TextRef::from(input.as_str()))
        .expect(constants_str::DIAGNOSTIC_225F922E);
    assert!(matches!(
        summary.push_str(crate::text_ref::TextRef::from(constants_str::X)),
        Err(crate::summary_text_append_error::SummaryTextAppendError::CapacityExceeded)
    ));
    assert_eq!(summary.as_ref(), input);
}

#[test]
fn test_runner_report_result_preserves_all_command_failures() {
    let mut failures = crate::command_failures_vec_deque::CommandFailuresVecDeque::default();
    failures.push(runner_spawn_failure_fixture(
        crate::command_index::CommandIndex::from(constants_usize::ZERO),
    ));
    failures.push(runner_spawn_failure_fixture(
        crate::command_index::CommandIndex::from(constants_usize::ONE),
    ));
    let result = crate::run_commands_error::RunCommandsError::from_report_result(failures, Ok(()));
    assert!(matches!(&result,
        Err(crate::run_commands_error::RunCommandsError::CommandsFailed { command_failures })
            if command_failures.len() == constants_usize::TWO
    ));
    if let Err(crate::run_commands_error::RunCommandsError::CommandsFailed { command_failures }) =
        result
    {
        assert!(command_failures.iter().all(|failure| matches!(
            failure,
            crate::command_failure::CommandFailure::Spawn {
                execution_io_error: error, ..
            } if error.kind() == std::io::ErrorKind::NotFound
        )));
    }
}

#[test]
fn test_runner_report_failure_retains_command_context_and_io_source() {
    let mut failures = crate::command_failures_vec_deque::CommandFailuresVecDeque::default();
    failures.push(runner_spawn_failure_fixture(
        crate::command_index::CommandIndex::from(constants_usize::ZERO),
    ));
    let report_error = crate::run_report_error::RunReportError::WriteSummary {
        execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError::from(
            std::io::Error::from(std::io::ErrorKind::PermissionDenied),
        ),
    };
    let expected = report_error.to_string();
    let error = crate::run_commands_error::RunCommandsError::from_report_result(
        failures,
        Err(report_error),
    )
    .expect_err(constants_str::DIAGNOSTIC_8690840B);
    assert_eq!(error.to_string(), expected);
    assert!(std::error::Error::source(&error).is_some());
    assert!(matches!(error,
        crate::run_commands_error::RunCommandsError::WriteReport {
            command_failures,
            run_report_error: crate::run_report_error::RunReportError::WriteSummary {
                execution_io_error: io_error,
            },
        } if command_failures.len() == constants_usize::ONE && io_error.kind() == std::io::ErrorKind::PermissionDenied
    ));
}

#[test]
fn test_runner_report_success_requires_no_command_failures() {
    crate::run_commands_error::RunCommandsError::from_report_result(
        crate::command_failures_vec_deque::CommandFailuresVecDeque::default(),
        Ok(()),
    )
    .expect(constants_str::DIAGNOSTIC_378BC649);
}

#[test]
fn test_runner_measurement_spawn_errors_keep_legacy_text_and_source() {
    let measurement_name = crate::measurement_name::MeasurementName::from(constants_str::STATIC);
    let io_error = std::io::Error::from(std::io::ErrorKind::NotFound);
    let io_text = io_error.to_string();
    let cargo_error = crate::cargo_measurement_error::CargoMeasurementError::Spawn {
        measurement_name,
        execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError::from(io_error),
    };
    assert_eq!(
        cargo_error.to_string(),
        format!(
            "{}{} {}{}",
            constants_str::RUNNER_MEASUREMENT_PREFIX,
            measurement_name,
            constants_str::RUNNER_MEASUREMENT_SPAWN_PREFIX,
            io_text,
        )
    );
    assert!(std::error::Error::source(&cargo_error).is_some());
    let memory_error = crate::memusage_measurement_error::MemusageMeasurementError::Spawn {
        measurement_name,
        execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError::from(
            std::io::Error::from(std::io::ErrorKind::NotFound),
        ),
    };
    assert_eq!(
        memory_error.to_string(),
        format!(
            "{}{}{} {}{}",
            constants_str::RUNNER_MEASUREMENT_PREFIX,
            measurement_name,
            constants_str::RUNNER_ALLOCATION_SUFFIX,
            constants_str::RUNNER_MEASUREMENT_SPAWN_PREFIX,
            io_text,
        )
    );
    assert!(std::error::Error::source(&memory_error).is_some());
}

#[test]
fn test_runner_fixture_conversion_retains_input_and_domain_validation_errors() {
    assert!(matches!(
        crate::create_admin_fixture_string::create_admin_fixture_string::<
            server_admin_contract::admin_login::AdminLogin,
        >(String::new()),
        Err(crate::admin_fixture_conversion_error::AdminFixtureConversionError::Login(_))
    ));
    let input = constants_str::X.repeat(constants_usize::VALUE_1_048_576 + constants_usize::ONE);
    let result = crate::create_admin_fixture_string::create_admin_fixture_string::<
        server_admin_contract::admin_text::AdminText,
    >(input);
    assert!(matches!(
        result,
        Err(crate::admin_fixture_conversion_error::AdminFixtureConversionError::Input(_))
    ));
}

#[test]
fn test_runner_output_failure_joins_command_failures_without_losing_sources() {
    let mut failures = crate::command_failures_vec_deque::CommandFailuresVecDeque::default();
    failures.push(runner_spawn_failure_fixture(
        crate::command_index::CommandIndex::from(constants_usize::ZERO),
    ));
    let mut output_failures = crate::command_failures_vec_deque::CommandFailuresVecDeque::default();
    output_failures.push(crate::command_failure::CommandFailure::WriteOutput {
        command_index: crate::command_index::CommandIndex::from(constants_usize::ONE),
        tool_console_write_error:
            macro_helpers::tool_console_write_error::ToolConsoleWriteError::StandardOutput(
                macro_helpers::std_tool_io_error::StdToolIoError::from(std::io::Error::from(
                    std::io::ErrorKind::BrokenPipe,
                )),
            ),
    });
    failures.append(&mut output_failures);
    assert!(output_failures.is_empty());
    assert_eq!(failures.len(), constants_usize::TWO);
    let result = crate::run_commands_error::RunCommandsError::from_report_result(failures, Ok(()));
    assert!(matches!(result,
        Err(crate::run_commands_error::RunCommandsError::CommandsFailed { command_failures })
        if command_failures.iter().any(|command_failure| matches!(command_failure,
            crate::command_failure::CommandFailure::WriteOutput {
                command_index,
                tool_console_write_error: macro_helpers::tool_console_write_error::ToolConsoleWriteError::StandardOutput(std_tool_io_error),
            } if usize::from(*command_index) == constants_usize::ONE && std_tool_io_error.kind() == std::io::ErrorKind::BrokenPipe
        )) && command_failures.len() == constants_usize::TWO
    ));
}
