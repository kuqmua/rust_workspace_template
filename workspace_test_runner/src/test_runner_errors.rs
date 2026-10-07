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
        crate::command_index::CommandIndex::from(constants_usize::TWO),
    ));
    failures.push(runner_spawn_failure_fixture(
        crate::command_index::CommandIndex::from(constants_usize::ZERO),
    ));
    let result = crate::run_commands_error::RunCommandsError::from_report_result(failures, Ok(()));
    assert!(matches!(&result,
        Err(crate::run_commands_error::RunCommandsError::CommandsFailed { command_failures })
            if command_failures.len() == constants_usize::TWO
    ));
    if let Err(crate::run_commands_error::RunCommandsError::CommandsFailed { command_failures }) =
        result
    {
        assert!(command_failures.iter().zip([constants_usize::TWO, constants_usize::ZERO]).all(|(failure, expected_index)| matches!(
            failure,
            crate::command_failure::CommandFailure::Spawn {
                command_index, execution_io_error: error,
            } if usize::from(*command_index) == expected_index && error.kind() == std::io::ErrorKind::NotFound
        )));
    }
}

#[test]
fn test_runner_report_failure_retains_command_context_and_io_source() {
    let mut failures = crate::command_failures_vec_deque::CommandFailuresVecDeque::default();
    failures.push(runner_spawn_failure_fixture(
        crate::command_index::CommandIndex::from(constants_usize::TWO),
    ));
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
        } if command_failures.len() == constants_usize::TWO && io_error.kind() == std::io::ErrorKind::PermissionDenied
            && command_failures.iter().zip([constants_usize::TWO, constants_usize::ZERO]).all(|(failure, expected_index)| matches!(failure,
                crate::command_failure::CommandFailure::Spawn { command_index, execution_io_error }
                    if usize::from(*command_index) == expected_index && execution_io_error.kind() == std::io::ErrorKind::NotFound
            ))
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
fn test_runner_report_errors_preserve_diagnostics_and_sources_without_command_failures() {
    let io_error = std::io::Error::from(std::io::ErrorKind::PermissionDenied);
    let io_text = io_error.to_string();
    let log_path = std::path::PathBuf::from(constants_str::WORKSPACE_TEST_RUNNER_RESULT_ROOT);
    [
        (
            crate::run_report_error::RunReportError::from(
                crate::summary_text_append_error::SummaryTextAppendError::CapacityExceeded,
            ),
            constants_str::RUNNER_SUMMARY_LIMIT_MESSAGE.to_owned(),
            None,
        ),
        (
            crate::run_report_error::RunReportError::MissingCommand {
                command_index: crate::command_index::CommandIndex::from(constants_usize::ONE),
            },
            format!("{} {}", constants_str::RUNNER_COMMAND_MISSING_MESSAGE, constants_usize::ONE),
            None,
        ),
        (
            crate::run_report_error::RunReportError::WriteLog {
                written_file_path_buf: macro_helpers::written_file_path_buf::WrittenFilePathBuf::from(log_path.clone()),
                execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError::from(io_error),
            },
            format!("{} {}: {}", constants_str::RUNNER_WRITE_LOG_MESSAGE, log_path.display(), io_text),
            Some(std::io::ErrorKind::PermissionDenied),
        ),
        (
            crate::run_report_error::RunReportError::WriteSummary {
                execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError::from(
                    std::io::Error::from(std::io::ErrorKind::NotFound),
                ),
            },
            format!("{}: {}", constants_str::RUNNER_WRITE_SUMMARY_MESSAGE, std::io::Error::from(std::io::ErrorKind::NotFound)),
            Some(std::io::ErrorKind::NotFound),
        ),
    ].into_iter().for_each(|(report_error, expected, expected_kind)| {
        assert_eq!(report_error.to_string(), expected);
        let result = crate::run_commands_error::RunCommandsError::from_report_result(
            crate::command_failures_vec_deque::CommandFailuresVecDeque::default(),
            Err(report_error),
        );
        let error = result.expect_err(constants_str::DIAGNOSTIC_8690840B);
        assert_eq!(error.to_string(), expected);
        assert!(std::error::Error::source(&error).is_some_and(|source| {
            assert!(source.downcast_ref::<crate::run_report_error::RunReportError>().is_some());
            expected_kind.map_or_else(
                || source.source().is_none(),
                |kind| source.source()
                    .and_then(|io_error_source| io_error_source.downcast_ref::<macro_helpers::std_tool_io_error::StdToolIoError>())
                    .is_some_and(|io_source| io_source.kind() == kind),
            )
        }));
        assert!(matches!(error,
            crate::run_commands_error::RunCommandsError::WriteReport { command_failures, .. }
                if command_failures.is_empty()
        ));
    });
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
    let login_result = crate::create_admin_fixture_string::create_admin_fixture_string::<
        server_admin_contract::admin_login::AdminLogin,
    >(String::new());
    assert!(matches!(
        login_result.as_ref(),
        Err(crate::admin_fixture_conversion_error::AdminFixtureConversionError::Login(_))
    ));
    assert!(
        login_result
            .as_ref()
            .err()
            .is_some_and(|error| std::error::Error::source(error).is_some())
    );
    let input = constants_str::X.repeat(constants_usize::VALUE_1_048_576 + constants_usize::ONE);
    let result = crate::create_admin_fixture_string::create_admin_fixture_string::<
        server_admin_contract::admin_text::AdminText,
    >(input);
    assert!(matches!(
        result.as_ref(),
        Err(crate::admin_fixture_conversion_error::AdminFixtureConversionError::Input(_))
    ));
    assert!(
        result
            .as_ref()
            .err()
            .is_some_and(|error| std::error::Error::source(error).is_some())
    );
}

#[test]
fn test_runner_output_failure_joins_command_failures_without_losing_sources() {
    let mut failures = crate::command_failures_vec_deque::CommandFailuresVecDeque::default();
    failures.push(runner_spawn_failure_fixture(
        crate::command_index::CommandIndex::from(constants_usize::TWO),
    ));
    let mut output_failures = crate::command_failures_vec_deque::CommandFailuresVecDeque::default();
    output_failures.push(crate::command_failure::CommandFailure::WriteOutput {
        command_index: crate::command_index::CommandIndex::from(constants_usize::ZERO),
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
        if command_failures.len() == constants_usize::TWO
            && command_failures.iter().enumerate().all(|(position, failure)| {
                matches!((position, failure),
                    (constants_usize::ZERO, crate::command_failure::CommandFailure::Spawn { command_index, execution_io_error })
                        if usize::from(*command_index) == constants_usize::TWO && execution_io_error.kind() == std::io::ErrorKind::NotFound
                ) || matches!((position, failure),
                    (constants_usize::ONE, crate::command_failure::CommandFailure::WriteOutput {
                        command_index,
                        tool_console_write_error: macro_helpers::tool_console_write_error::ToolConsoleWriteError::StandardOutput(std_tool_io_error),
                    }) if usize::from(*command_index) == constants_usize::ZERO && std_tool_io_error.kind() == std::io::ErrorKind::BrokenPipe
                )
            })
    ));
}

#[test]
fn test_runner_summary_recovers_after_utf8_overflow_and_accepts_empty_append() {
    let maximum = constants_usize::VALUE_1_048_576;
    let initial = constants_str::X.repeat(maximum - 1usize);
    assert!(
        crate::summary_text::SummaryText::try_from(initial).is_ok_and(|mut summary| {
            let multibyte = '\u{00e9}'.to_string();
            assert!(matches!(
                summary.push_str(crate::text_ref::TextRef::from(multibyte.as_str())),
                Err(crate::summary_text_append_error::SummaryTextAppendError::CapacityExceeded),
            ));
            assert_eq!(summary.as_ref().len(), maximum - 1usize);
            assert!(
                summary
                    .as_ref()
                    .bytes()
                    .eq(constants_str::X.bytes().cycle().take(maximum - 1usize))
            );
            assert!(matches!(
                summary.push_str(crate::text_ref::TextRef::from(constants_str::X)),
                Ok(()),
            ));
            assert!(matches!(
                summary.push_str(crate::text_ref::TextRef::from(
                    constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
                )),
                Ok(()),
            ));
            summary.as_ref().len() == maximum
                && summary
                    .as_ref()
                    .bytes()
                    .eq(constants_str::X.bytes().cycle().take(maximum))
        })
    );
}

fn fixture_domain_conversion_error<Value>()
-> Option<crate::admin_fixture_conversion_error::AdminFixtureConversionError>
where
    Value: TryFrom<String>,
    Value::Error: Into<crate::admin_fixture_conversion_error::AdminFixtureConversionError>,
{
    crate::create_admin_fixture_string::create_admin_fixture_string::<Value>(
        constants_str::X.repeat(8193usize),
    )
    .err()
    .filter(|error| {
        std::error::Error::source(error)
            .is_some_and(|source| source.to_string() == error.to_string())
    })
}

#[test]
fn test_fixture_admin_audit_timestamp_conversion_preserves_domain_error_source() {
    assert!(matches!(
        fixture_domain_conversion_error::<
            server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp,
        >(),
        Some(crate::admin_fixture_conversion_error::AdminFixtureConversionError::AuditTimestamp(_))
    ));
}

#[test]
fn test_fixture_admin_display_name_conversion_preserves_domain_error_source() {
    assert!(matches!(
        fixture_domain_conversion_error::<
            server_admin_contract::admin_display_name::AdminDisplayName,
        >(),
        Some(crate::admin_fixture_conversion_error::AdminFixtureConversionError::DisplayName(_))
    ));
}

#[test]
fn test_fixture_admin_role_name_conversion_preserves_domain_error_source() {
    assert!(matches!(
        fixture_domain_conversion_error::<server_admin_contract::admin_role_name::AdminRoleName>(),
        Some(crate::admin_fixture_conversion_error::AdminFixtureConversionError::RoleName(_))
    ));
}

#[test]
fn test_fixture_admin_role_timestamp_conversion_preserves_domain_error_source() {
    assert!(matches!(
        fixture_domain_conversion_error::<
            server_admin_contract::admin_role_timestamp::AdminRoleTimestamp,
        >(),
        Some(crate::admin_fixture_conversion_error::AdminFixtureConversionError::RoleTimestamp(_))
    ));
}

#[test]
fn test_fixture_admin_rule_value_conversion_preserves_domain_error_source() {
    assert!(matches!(
        fixture_domain_conversion_error::<server_admin_contract::admin_rule_value::AdminRuleValue>(
        ),
        Some(crate::admin_fixture_conversion_error::AdminFixtureConversionError::RuleValue(_))
    ));
}

#[test]
fn test_fixture_admin_session_identifier_conversion_preserves_domain_error_source() {
    assert!(matches!(
        fixture_domain_conversion_error::<
            server_admin_contract::admin_session_identifier::AdminSessionIdentifier,
        >(),
        Some(
            crate::admin_fixture_conversion_error::AdminFixtureConversionError::SessionIdentifier(
                _
            )
        )
    ));
}

#[test]
fn test_fixture_admin_session_timestamp_conversion_preserves_domain_error_source() {
    assert!(matches!(
        fixture_domain_conversion_error::<
            server_admin_contract::admin_session_timestamp::AdminSessionTimestamp,
        >(),
        Some(
            crate::admin_fixture_conversion_error::AdminFixtureConversionError::SessionTimestamp(_)
        )
    ));
}

#[test]
fn test_fixture_admin_text_conversion_preserves_domain_error_source() {
    assert!(matches!(
        fixture_domain_conversion_error::<server_admin_contract::admin_text::AdminText>(),
        Some(crate::admin_fixture_conversion_error::AdminFixtureConversionError::Text(_))
    ));
}

#[test]
fn test_fixture_conversion_preserves_exact_unicode_character_limit_and_empty_text() {
    let unicode = char::from(233u8).to_string();
    [constants_usize::ZERO, 8192usize]
        .into_iter()
        .fold((), |(), characters| {
            let input = unicode.repeat(characters);
            let expected_bytes = input.len();
            let result = crate::create_admin_fixture_string::create_admin_fixture_string::<
                server_admin_contract::admin_text::AdminText,
            >(input);
            assert!(result.is_ok_and(|text| {
                let rendered = text.to_string();
                rendered.len() == expected_bytes
                    && rendered.chars().count() == characters
                    && rendered
                        .chars()
                        .all(|character| character == char::from(233u8))
            }));
        });
}

#[test]
fn test_fixture_conversion_separates_unicode_character_errors_from_input_byte_errors() {
    let maximum = constants_usize::VALUE_1_048_576;
    let unicode = char::from(233u8).to_string();
    let input_characters = maximum.checked_div(unicode.len()).unwrap_or_default();
    [8193usize, input_characters, input_characters.saturating_add(constants_usize::ONE)]
        .into_iter().fold((), |(), characters| {
            let input = unicode.repeat(characters);
            let bytes = input.len();
            let result = crate::create_admin_fixture_string::create_admin_fixture_string::<server_admin_contract::admin_text::AdminText>(input);
            assert!(result.as_ref().is_err_and(|error| std::error::Error::source(error).is_some()));
            if bytes <= maximum {
                assert!(matches!(result, Err(crate::admin_fixture_conversion_error::AdminFixtureConversionError::Text(
                    server_admin_contract::admin_text::AdminTextTryFromStringError::TooLong { len, max }
                )) if len == characters && max == 8192usize));
            } else {
                assert!(matches!(result, Err(crate::admin_fixture_conversion_error::AdminFixtureConversionError::Input(
                    crate::admin_fixture_string::AdminFixtureStringTryFromStringError::TooLong { len, max }
                )) if len == bytes && max == maximum));
            }
        });
}
