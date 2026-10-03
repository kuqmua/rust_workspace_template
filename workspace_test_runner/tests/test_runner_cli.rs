#![allow(
    unused_crate_dependencies,
    reason = "binary integration test uses only the process wrapper from the package dependency catalog"
)]

#[cfg(test)]
mod tests {
    #[test]
    fn test_oversized_mode_reports_length_error_without_unknown_mode_fallback() {
        assert!(
            [
                constants_str::X.repeat(1_025usize),
                '\u{00e9}'.to_string().repeat(513usize),
            ]
            .into_iter()
            .all(|mode| {
                let mut command = macro_helpers::tool_command::ToolCommand::new(
                    macro_helpers::tool_program_ref::ToolProgramRef::from(env!(
                        "CARGO_BIN_EXE_workspace_test_runner"
                    )),
                );
                let _command =
                    command.arg(macro_helpers::tool_arg_ref::ToolArgRef::from(mode.as_str()));
                let result = command.bounded_output(
                    macro_helpers::tool_output_limit::ToolOutputLimit::from(2_048usize),
                );
                result.is_ok_and(|output| {
                    let stderr = String::from_utf8_lossy(output.stderr.as_slice());
                    output.status.code() == Some(1i32)
                        && stderr.contains(mode.len().to_string().as_str())
                        && !stderr.contains(constants_str::RUNNER_CLI_TEXT_A8DCE6DA)
                        && output.stdout.is_empty()
                })
            })
        );
    }

    #[test]
    fn test_unknown_runner_modes_preserve_diagnostic_and_exit_status() {
        assert!(
            [
                String::new(),
                constants_str::X.to_owned(),
                constants_str::X.repeat(1_024usize),
                '\u{00e9}'.to_string().repeat(512usize),
            ]
            .into_iter()
            .all(|mode| {
                let mut command = macro_helpers::tool_command::ToolCommand::new(
                    macro_helpers::tool_program_ref::ToolProgramRef::from(env!(
                        "CARGO_BIN_EXE_workspace_test_runner"
                    )),
                );
                let _command =
                    command.arg(macro_helpers::tool_arg_ref::ToolArgRef::from(mode.as_str()));
                command
                    .bounded_output(macro_helpers::tool_output_limit::ToolOutputLimit::from(
                        2_048usize,
                    ))
                    .is_ok_and(|output| {
                        output.status.code() == Some(1i32)
                            && output.stdout.is_empty()
                            && std::str::from_utf8(output.stderr.as_slice()).is_ok_and(|stderr| {
                                stderr
                                    == format!(
                                        "{}{}{}{}",
                                        constants_str::RUNNER_CLI_TEXT_A8DCE6DA,
                                        mode,
                                        constants_str::RUNNER_CLI_TEXT_A774F989,
                                        constants_str::NEWLINE,
                                    )
                            })
                    })
            })
        );
    }

    #[test]
    fn test_admin_fixture_cli_emits_valid_shared_contract_payloads() {
        let mut command = macro_helpers::tool_command::ToolCommand::new(
            macro_helpers::tool_program_ref::ToolProgramRef::from(env!(
                "CARGO_BIN_EXE_workspace_test_runner"
            )),
        );
        let _command = command.arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
            constants_str::WORKSPACE_TEST_RUNNER_ADMIN_CONTRACT_FIXTURE,
        ));
        assert!(
            command
                .bounded_output(macro_helpers::tool_output_limit::ToolOutputLimit::from(
                    2_048usize
                ))
                .is_ok_and(|output| output.status.success()
                    && output.stdout.is_empty()
                    && output.stderr.is_empty())
        );
        let Some(workspace_directory) = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent()
        else {
            std::panic::panic_any(constants_str::PANIC_EDC94D17);
        };
        let fixture_path = workspace_directory
            .join(constants_str::TARGET)
            .join(constants_str::WORKSPACE_TEST_RUNNER_ADMIN_CONTRACT_FIXTURE_FILE);
        assert!(server_runtime_http::read_bounded_file::read_bounded_file(
        server_runtime_http::runtime_path_ref::RuntimePathRef::from(fixture_path.as_path()),
        server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(16_777_216usize),
    ).is_ok_and(|bytes| {
        serde_json::from_slice::<serde_json::Value>(bytes.into_inner().as_slice()).is_ok_and(|fixture| {
            let Some(parts) = fixture.as_array() else { return false; };
            let [route_values, rule_values, authenticated, users, roles, rules, audit, sessions, no_body, body_limit, open_api] = parts.as_slice() else { return false; };
            let contracts = <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::schema_contracts();
            assert!(route_values.as_array().is_some_and(|routes| routes.len() == contracts.as_ref().len()
                && routes.iter().zip(contracts.as_ref()).all(|(route, contract)| {
                    let Some(fields) = route.as_array() else { return false; };
                    let [operation, method, path, status, _request_schema, _response_schema] = fields.as_slice() else { return false; };
                    let metadata = contract.metadata();
                    operation.as_str() == Some(metadata.openapi_operation_id().as_ref())
                        && method.as_str() == Some(metadata.method().as_ref())
                        && path.as_str() == Some(metadata.path().as_ref())
                        && status.as_u64() == Some(u64::from(u16::from(metadata.success_status().transport_status())))
                })));
            assert!(serde_json::to_value(server_admin_contract::admin_rule::AdminRule::ALL.into_iter().map(|rule| rule.as_str().as_ref().to_owned()).collect::<Vec<_>>())
                .is_ok_and(|expected_rules| expected_rules == *rule_values));
            assert_eq!([
                serde_json::from_value::<server_admin_contract::authenticated_admin::AuthenticatedAdmin>(authenticated.clone()).is_ok(),
                serde_json::from_value::<server_admin_contract::admin_users_page::AdminUsersPage>(users.clone()).is_ok(),
                serde_json::from_value::<server_admin_contract::admin_roles_page::AdminRolesPage>(roles.clone()).is_ok(),
                serde_json::from_value::<server_admin_contract::admin_rules_page::AdminRulesPage>(rules.clone()).is_ok(),
                serde_json::from_value::<server_admin_contract::admin_audit_page::AdminAuditPage>(audit.clone()).is_ok(),
                serde_json::from_value::<server_admin_contract::admin_no_body::AdminNoBody>(no_body.clone()).is_ok(),
            ], [true; 6usize]);
            assert!(open_api.get(constants_str::COMPONENTS).and_then(|components| components.get(constants_str::SCHEMAS))
                .and_then(serde_json::Value::as_object).is_some_and(|schemas| !schemas.is_empty()));
            assert!(open_api.get(constants_str::PATHS).and_then(serde_json::Value::as_object)
                .is_some_and(|paths| contracts.as_ref().iter().all(|contract| {
                    let metadata = contract.metadata();
                    paths.get(metadata.path().as_ref())
                        .and_then(|path| path.get(metadata.method().as_ref().to_ascii_lowercase()))
                        .and_then(|operation| operation.get(stringify!(operationId)))
                        .and_then(serde_json::Value::as_str) == Some(metadata.openapi_operation_id().as_ref())
                })));
            assert!(sessions.as_array().is_some_and(|session_values| !session_values.is_empty() && session_values.iter().all(|session| {
                serde_json::from_value::<server_admin_contract::admin_session_view::AdminSessionView>(session.clone()).is_ok()
            })));
            assert_eq!(body_limit.as_u64(), <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::body_limit().and_then(|limit| u64::try_from(limit.get()).ok()));
            true
        })
    }));
    }
    #[cfg(unix)]
    #[test]
    #[ignore = "requires /usr/bin/true and /usr/bin/false; provisions isolated runner command fixtures"]
    fn test_workspace_executor_selection_records_matching_command_catalog() {
        assert!(
            [
                (
                    Some(constants_str::TEST_TRUE_EXECUTABLE_PATH),
                    constants_str::RUNNER_NEXTEST_ANNOUNCEMENT,
                    true,
                    &constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS, false
                ),
                (
                    Some(constants_str::TEST_FALSE_EXECUTABLE_PATH),
                    constants_str::RUNNER_CARGO_ANNOUNCEMENT,
                    false,
                    &constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS, false
                ),
                (None, constants_str::RUNNER_CARGO_ANNOUNCEMENT, false, &constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS, false),
                (Some(constants_str::TEST_TRUE_EXECUTABLE_PATH), constants_str::RUNNER_NEXTEST_ANNOUNCEMENT, false, &constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS, true),
                (None, constants_str::RUNNER_CARGO_ANNOUNCEMENT, false, &constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS, true),
            ]
            .into_iter()
            .all(|(executable, announcement, success, commands, blocked_reports)| {
                let directory =
                    std::env::temp_dir().join(format!("{}-{}", announcement, std::process::id()));
                let creation = std::fs::DirBuilder::new().create(&directory);
                assert!(creation.is_ok_and(|()| directory.is_dir()));
                let cargo = directory.join(constants_str::WORKSPACE_TEST_RUNNER_CARGO);
                let outcome = (|| {
                if let Some(fixture_executable) = executable {
                    std::os::unix::fs::symlink(fixture_executable, &cargo).map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                }
                let result_directory = directory.join(constants_str::WORKSPACE_TEST_RUNNER_RESULT_ROOT);
                if blocked_reports {
                    let parent = result_directory.parent().ok_or_else(|| macro_helpers::std_tool_io_error::StdToolIoError::from(std::io::Error::from(std::io::ErrorKind::InvalidInput)))?;
                    std::fs::create_dir_all(parent).map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                    std::fs::write(&result_directory, constants_str::X).map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                }
                let path = directory.to_str().ok_or_else(|| macro_helpers::std_tool_io_error::StdToolIoError::from(std::io::Error::from(std::io::ErrorKind::InvalidData)))?;
                let mut command = macro_helpers::tool_command::ToolCommand::new(macro_helpers::tool_program_ref::ToolProgramRef::from(env!("CARGO_BIN_EXE_workspace_test_runner")));
                let _command = command
                    .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(constants_str::TESTS_ALT))
                    .current_dir(macro_helpers::macro_path_ref::MacroPathRef::from(directory.as_path()))
                    .env(macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(constants_str::PATH_ALT), macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(path));
                let output = command.bounded_output(macro_helpers::tool_output_limit::ToolOutputLimit::from(16_384usize)).map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                let stdout = std::str::from_utf8(output.stdout.as_slice()).map_err(|source| macro_helpers::std_tool_io_error::StdToolIoError::from(std::io::Error::other(source)))?;
                if !stdout.contains(announcement) || output.status.success() != success { return Ok(false); }
                if blocked_reports {
                    return Ok(result_directory.metadata().is_ok_and(|metadata| metadata.is_file() && metadata.len() == 1u64)
                        && std::str::from_utf8(output.stderr.as_slice()).is_ok_and(|stderr| stderr.contains(constants_str::RUNNER_CREATE_DIRECTORY_MESSAGE) && !stderr.contains(constants_str::RUNNER_COMMAND_FAILURE_MESSAGE)));
                }
                let entries = std::fs::read_dir(result_directory).and_then(Iterator::collect::<Result<Vec<_>, _>>).map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                let [entry] = entries.as_slice() else { return Ok(false); };
                let summary_path = entry.path().join(constants_str::SUMMARY_TXT);
                let bytes = server_runtime_http::read_bounded_file::read_bounded_file(server_runtime_http::runtime_path_ref::RuntimePathRef::from(summary_path.as_path()), server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(16_384usize)).map_err(|source| macro_helpers::std_tool_io_error::StdToolIoError::from(std::io::Error::other(source)))?;
                let summary = String::from_utf8(bytes.into_inner()).map_err(|source| macro_helpers::std_tool_io_error::StdToolIoError::from(std::io::Error::other(source)))?;
                let expected_status = if executable.is_some() {
                    let process_exit_status = macro_helpers::process_exit_status::ProcessExitStatus::from(<std::process::ExitStatus as std::os::unix::process::ExitStatusExt>::from_raw(if success { 0i32 } else { 256i32 }));
                    format!("{}={process_exit_status}", stringify!(status))
                } else {
                    format!("{}={}", stringify!(status), constants_str::RUNNER_CLI_TEXT_5F9C0B1E)
                };
                let report_files = std::fs::read_dir(entry.path()).and_then(Iterator::collect::<Result<Vec<_>, _>>).map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                Ok::<_, macro_helpers::std_tool_io_error::StdToolIoError>(
                    summary.lines().count() == commands.len()
                        && report_files.len() == commands.len().saturating_add(1usize)
                        && commands.iter().zip(summary.lines()).enumerate().all(|(index, ((program, arguments), line))| {
                            line.starts_with(format!("{}={program} ", stringify!(command)).as_str())
                                && line.contains(format!("{}={arguments:?}", stringify!(args)).as_str())
                                && line.contains(expected_status.as_str())
                                && report_files.iter().any(|report_file| report_file.file_name().to_str().is_some_and(|name| name.starts_with(format!("{index:02}{}", constants_str::HYPHEN).as_str()) && line.contains(name)) && (executable.is_some() || server_runtime_http::read_bounded_file::read_bounded_file(server_runtime_http::runtime_path_ref::RuntimePathRef::from(report_file.path().as_path()), server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(16_384usize)).is_ok_and(|log_bytes| std::str::from_utf8(log_bytes.into_inner().as_slice()).is_ok_and(|log_text| log_text.starts_with(constants_str::RUNNER_CLI_TEXT_BE4BF145)))))
                        })
                )
            })();
            let cleanup = std::fs::remove_dir_all(&directory);
                assert!(cleanup.is_ok_and(|()| !directory.exists()));
                outcome.is_ok_and(|matched| matched)
            })
        );
    }
    #[cfg(unix)]
    #[test]
    #[ignore = "requires /bin/sh, /usr/bin/mkdir and /usr/bin/cat; provisions isolated command output and report write fixtures"]
    fn test_workspace_reports_preserve_failure_names_logs_and_exit_status() {
        let standard_failure_log = format!(
            "{}{}{}{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_ANSI_RED,
            constants_str::VALUE_E6CA5E47,
            constants_str::WORKSPACE_TEST_RUNNER_ANSI_RESET,
            constants_str::NEWLINE,
            constants_str::VALUE_E6CA5E47,
        );
        let oversized_failure_log = format!(
            "{}{}{}{}",
            constants_str::TEST_ALT,
            constants_str::X.repeat(constants_usize::VALUE_1_048_576),
            constants_str::FAILED_ALT,
            constants_str::NEWLINE,
        );
        let first_log = format!(
            "{:02}{}{}{}{}{}{}{}",
            0usize,
            constants_str::HYPHEN,
            constants_str::WORKSPACE_TEST_RUNNER_CARGO,
            constants_str::HYPHEN,
            constants_str::TEST_ALT_3,
            constants_str::HYPHEN,
            constants_str::SHARED_VALUES_LOCKED,
            constants_str::RUNNER_CLI_TEXT_2E4BB066,
        );
        assert!(
            [
                (constants_str::X, constants_str::RUNNER_COMMAND_FAILURE_MESSAGE),
                (
                    constants_str::RUNNER_SUMMARY_LIMIT_MESSAGE,
                    constants_str::RUNNER_SUMMARY_LIMIT_MESSAGE,
                ),
                (first_log.as_str(), constants_str::RUNNER_WRITE_LOG_MESSAGE),
                (
                    constants_str::SUMMARY_TXT,
                    constants_str::RUNNER_WRITE_SUMMARY_MESSAGE,
                ),
            ]
            .into_iter()
            .all(|(report_name, diagnostic)| {
                let failure_log = if report_name == constants_str::RUNNER_SUMMARY_LIMIT_MESSAGE {
                    oversized_failure_log.as_str()
                } else {
                    standard_failure_log.as_str()
                };
                let directory =
                    std::env::temp_dir().join(format!("{}-{}", diagnostic, std::process::id()));
                assert!(
                    std::fs::DirBuilder::new()
                        .create(&directory)
                        .is_ok_and(|()| directory.is_dir())
                );
                let outcome = (|| {
                    let cargo = directory.join(constants_str::WORKSPACE_TEST_RUNNER_CARGO);
                    let script_tail = if report_name == constants_str::X || report_name == constants_str::RUNNER_SUMMARY_LIMIT_MESSAGE {
                        std::fs::write(directory.join(constants_str::X), failure_log)
                            .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                        constants_str::RUNNER_REPORT_FIXTURE_FAILURE_COMMAND.to_owned()
                    } else {
                        format!("{}{}{}", constants_str::RUNNER_REPORT_FIXTURE_SHELL_TARGET, report_name, constants_str::NEWLINE)
                    };
                    let script = format!(
                        "{}{}{}{}/*{}",
                        constants_str::RUNNER_REPORT_FIXTURE_SHELL_PREFIX,
                        constants_str::WORKSPACE_TEST_RUNNER_RESULT_ROOT,
                        constants_str::RUNNER_REPORT_FIXTURE_SHELL_COMMAND,
                        constants_str::WORKSPACE_TEST_RUNNER_RESULT_ROOT,
                        script_tail,
                    );
                    std::fs::write(&cargo, script)
                        .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                    std::fs::set_permissions(
                        &cargo,
                        <std::fs::Permissions as std::os::unix::fs::PermissionsExt>::from_mode(
                            0o700u32,
                        ),
                    )
                    .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                    let path = directory.to_str().ok_or_else(|| {
                        macro_helpers::std_tool_io_error::StdToolIoError::from(
                            std::io::Error::from(std::io::ErrorKind::InvalidData),
                        )
                    })?;
                    let mut command = macro_helpers::tool_command::ToolCommand::new(
                        macro_helpers::tool_program_ref::ToolProgramRef::from(env!(
                            "CARGO_BIN_EXE_workspace_test_runner"
                        )),
                    );
                    let _command = command
                        .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
                            constants_str::TESTS_ALT,
                        ))
                        .current_dir(macro_helpers::macro_path_ref::MacroPathRef::from(
                            directory.as_path(),
                        ))
                        .env(
                            macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(
                                constants_str::PATH_ALT,
                            ),
                            macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(path),
                        );
                    let output = command
                        .bounded_output(macro_helpers::tool_output_limit::ToolOutputLimit::from(
                            16_384usize,
                        ))
                        .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                    let result_root =
                        directory.join(constants_str::WORKSPACE_TEST_RUNNER_RESULT_ROOT);
                    let entries = std::fs::read_dir(result_root)
                        .and_then(Iterator::collect::<Result<Vec<_>, _>>)
                        .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                    let [entry] = entries.as_slice() else {
                        return Ok(false);
                    };
                    let run_directory = entry.path();
                    if report_name == constants_str::RUNNER_SUMMARY_LIMIT_MESSAGE {
                        let log_path = run_directory.join(&first_log);
                        let log_bytes = server_runtime_http::read_bounded_file::read_bounded_file(
                            server_runtime_http::runtime_path_ref::RuntimePathRef::from(log_path.as_path()),
                            server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(constants_usize::VALUE_16_777_216),
                        ).map_err(|source| macro_helpers::std_tool_io_error::StdToolIoError::from(std::io::Error::other(source)))?;
                        let report_files = std::fs::read_dir(&run_directory)
                            .and_then(Iterator::collect::<Result<Vec<_>, _>>)
                            .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                        return Ok(output.status.code() == Some(1i32)
                            && log_bytes.into_inner().as_slice() == failure_log.as_bytes()
                            && report_files.len() == 1usize
                            && !run_directory.join(constants_str::SUMMARY_TXT).exists()
                            && failure_log.as_bytes().get(failure_log.len().saturating_sub(16_384usize)..) == Some(output.stdout.as_slice())
                            && std::str::from_utf8(output.stderr.as_slice()).is_ok_and(|stderr| stderr.contains(diagnostic)
                                && !stderr.contains(constants_str::RUNNER_WRITE_LOG_MESSAGE)
                                && !stderr.contains(constants_str::RUNNER_WRITE_SUMMARY_MESSAGE)
                                && !stderr.contains(constants_str::RUNNER_COMMAND_FAILURE_MESSAGE)));
                    }
                    if report_name == constants_str::X {
                        let summary_path = run_directory.join(constants_str::SUMMARY_TXT);
                        let summary_bytes = server_runtime_http::read_bounded_file::read_bounded_file(
                            server_runtime_http::runtime_path_ref::RuntimePathRef::from(summary_path.as_path()),
                            server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(16_384usize),
                        ).map_err(|source| macro_helpers::std_tool_io_error::StdToolIoError::from(std::io::Error::other(source)))?;
                        let log_path = run_directory.join(&first_log);
                        let log_bytes = server_runtime_http::read_bounded_file::read_bounded_file(
                            server_runtime_http::runtime_path_ref::RuntimePathRef::from(log_path.as_path()),
                            server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(16_384usize),
                        ).map_err(|source| macro_helpers::std_tool_io_error::StdToolIoError::from(std::io::Error::other(source)))?;
                        let expected_failure_names = format!("{}{}{}{}", constants_str::RUNNER_CLI_TEXT_94C4B52B, constants_str::VALUE_6B4D91DC, constants_str::TEXT_ALT_7, constants_str::VALUE_B40C5E30);
                        return Ok(output.status.code() == Some(1i32)
                            && log_bytes.into_inner().as_slice() == failure_log.as_bytes()
                            && std::str::from_utf8(summary_bytes.into_inner().as_slice()).is_ok_and(|summary| {
                                summary.lines().count() == constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS.len()
                                    && summary.lines().all(|line| line.ends_with(expected_failure_names.as_str()))
                                    && !summary.contains(constants_str::WORKSPACE_TEST_RUNNER_ANSI_RED)
                                    && !summary.contains(constants_str::WORKSPACE_TEST_RUNNER_ANSI_RESET)
                            })
                            && std::str::from_utf8(output.stdout.as_slice()).is_ok_and(|stdout| stdout.contains(constants_str::RUNNER_CARGO_ANNOUNCEMENT) && stdout.matches(failure_log).count() == constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS.len())
                            && output.stderr.is_empty());
                    }
                    let report_files = std::fs::read_dir(&run_directory)
                        .and_then(Iterator::collect::<Result<Vec<_>, _>>)
                        .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                    let logs_preserved = if report_name == constants_str::SUMMARY_TXT {
                        report_files.len()
                            == constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS
                                .len()
                                .saturating_add(1usize)
                            && report_files
                                .iter()
                                .filter(|report_file| report_file.path().is_file())
                                .count()
                                == constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS.len()
                            && run_directory
                                .join(&first_log)
                                .metadata()
                                .is_ok_and(|metadata| metadata.is_file() && metadata.len() == 0u64)
                    } else {
                        report_files.len() == 1usize
                            && !run_directory.join(constants_str::SUMMARY_TXT).exists()
                    };
                    Ok::<_, macro_helpers::std_tool_io_error::StdToolIoError>(
                        output.status.code() == Some(1i32)
                            && run_directory.join(report_name).is_dir()
                            && logs_preserved
                            && std::str::from_utf8(output.stdout.as_slice()).is_ok_and(|stdout| {
                                stdout.contains(constants_str::RUNNER_CARGO_ANNOUNCEMENT)
                                    && !stdout.contains(constants_str::RUNNER_NEXTEST_ANNOUNCEMENT)
                            })
                            && std::str::from_utf8(output.stderr.as_slice()).is_ok_and(|stderr| {
                                stderr.contains(diagnostic)
                                    && !stderr
                                        .contains(constants_str::RUNNER_COMMAND_FAILURE_MESSAGE)
                                    && (report_name == constants_str::SUMMARY_TXT
                                        || stderr.contains(report_name))
                            }),
                    )
                })();
                assert!(std::fs::remove_dir_all(&directory).is_ok_and(|()| !directory.exists()));
                outcome.is_ok_and(|matched| matched)
            })
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    #[ignore = "requires /dev/full, /bin/sh and /usr/bin/cat; provisions isolated successful commands with an unwritable stderr stream"]
    fn test_workspace_stderr_output_failures_preserve_successful_command_reports() {
        let directory = std::env::temp_dir().join(format!(
            "{}-{}",
            stringify!(test_workspace_stderr_output_failures_preserve_successful_command_reports),
            std::process::id()
        ));
        assert!(
            std::fs::DirBuilder::new()
                .create(&directory)
                .is_ok_and(|()| directory.is_dir())
        );
        let outcome = (|| {
            let cargo = directory.join(constants_str::WORKSPACE_TEST_RUNNER_CARGO);
            let launcher = directory.join(stringify!(stderr));
            let cargo_script = format!(
                "{}{}{}{}/*{}",
                constants_str::RUNNER_REPORT_FIXTURE_SHELL_PREFIX,
                constants_str::WORKSPACE_TEST_RUNNER_RESULT_ROOT,
                constants_str::RUNNER_REPORT_FIXTURE_SHELL_COMMAND,
                constants_str::WORKSPACE_TEST_RUNNER_RESULT_ROOT,
                constants_str::RUNNER_REPORT_FIXTURE_STDERR_SUCCESS_COMMAND,
            );
            std::fs::write(&cargo, cargo_script)
                .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
            std::fs::write(
                &launcher,
                constants_str::RUNNER_REPORT_FIXTURE_FULL_STDERR_COMMAND,
            )
            .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
            std::fs::write(directory.join(constants_str::X), constants_str::X)
                .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
            [&cargo, &launcher]
                .into_iter()
                .try_for_each(|executable| {
                    std::fs::set_permissions(
                        executable,
                        <std::fs::Permissions as std::os::unix::fs::PermissionsExt>::from_mode(
                            0o700u32,
                        ),
                    )
                })
                .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
            let path = directory.to_str().ok_or_else(|| {
                macro_helpers::std_tool_io_error::StdToolIoError::from(std::io::Error::from(
                    std::io::ErrorKind::InvalidData,
                ))
            })?;
            let launcher_program = launcher.to_str().ok_or_else(|| {
                macro_helpers::std_tool_io_error::StdToolIoError::from(std::io::Error::from(
                    std::io::ErrorKind::InvalidData,
                ))
            })?;
            let mut command = macro_helpers::tool_command::ToolCommand::new(
                macro_helpers::tool_program_ref::ToolProgramRef::from(launcher_program),
            );
            let _command = command
                .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(env!(
                    "CARGO_BIN_EXE_workspace_test_runner"
                )))
                .current_dir(macro_helpers::macro_path_ref::MacroPathRef::from(
                    directory.as_path(),
                ))
                .env(
                    macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(constants_str::PATH_ALT),
                    macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(path),
                );
            let output = command
                .bounded_output(macro_helpers::tool_output_limit::ToolOutputLimit::from(
                    16_384usize,
                ))
                .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
            let entries =
                std::fs::read_dir(directory.join(constants_str::WORKSPACE_TEST_RUNNER_RESULT_ROOT))
                    .and_then(Iterator::collect::<Result<Vec<_>, _>>)
                    .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
            let [entry] = entries.as_slice() else {
                return Ok(false);
            };
            let run_directory = entry.path();
            let report_files = std::fs::read_dir(&run_directory)
                .and_then(Iterator::collect::<Result<Vec<_>, _>>)
                .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
            let summary_path = run_directory.join(constants_str::SUMMARY_TXT);
            let summary_bytes = server_runtime_http::read_bounded_file::read_bounded_file(
                server_runtime_http::runtime_path_ref::RuntimePathRef::from(summary_path.as_path()),
                server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
                    16_384usize,
                ),
            )
            .map_err(|source| {
                macro_helpers::std_tool_io_error::StdToolIoError::from(std::io::Error::other(
                    source,
                ))
            })?;
            let successful_status = format!(
                "{}={}",
                stringify!(status),
                macro_helpers::process_exit_status::ProcessExitStatus::from(
                    <std::process::ExitStatus as std::os::unix::process::ExitStatusExt>::from_raw(
                        0i32
                    )
                )
            );
            Ok::<_, macro_helpers::std_tool_io_error::StdToolIoError>(output.status.code() == Some(1i32)
                && output.stderr.is_empty()
                && std::str::from_utf8(output.stdout.as_slice()).is_ok_and(|stdout| stdout == format!("{}{}", constants_str::RUNNER_CARGO_ANNOUNCEMENT, constants_str::NEWLINE))
                && report_files.len() == constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS.len().saturating_add(1usize)
                && std::str::from_utf8(summary_bytes.into_inner().as_slice()).is_ok_and(|summary| summary.lines().count() == constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS.len()
                    && summary.lines().all(|line| line.contains(successful_status.as_str()) && line.ends_with(constants_str::RUNNER_CLI_TEXT_94C4B52B)))
                && report_files.iter().filter(|report_file| report_file.path() != summary_path).all(|report_file| {
                    server_runtime_http::read_bounded_file::read_bounded_file(
                        server_runtime_http::runtime_path_ref::RuntimePathRef::from(report_file.path().as_path()),
                        server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(16_384usize),
                    ).is_ok_and(|bytes| bytes.into_inner().as_slice() == constants_str::X.as_bytes())
                }))
        })();
        assert!(std::fs::remove_dir_all(&directory).is_ok_and(|()| !directory.exists()));
        assert!(outcome.is_ok_and(|matched| matched));
    }

    #[test]
    fn test_measurement_cli_missing_cargo_reports_spawn_failure_and_exits() {
        let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(
            !directory
                .join(constants_str::WORKSPACE_TEST_RUNNER_CARGO)
                .exists()
        );
        let mut command = macro_helpers::tool_command::ToolCommand::new(
            macro_helpers::tool_program_ref::ToolProgramRef::from(env!(
                "CARGO_BIN_EXE_workspace_test_runner"
            )),
        );
        let _command = command
            .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
                constants_str::MEASURE,
            ))
            .current_dir(macro_helpers::macro_path_ref::MacroPathRef::from(directory))
            .env(
                macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(constants_str::PATH_ALT),
                macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(constants_str::EMPTY),
            );
        assert!(
            command
                .bounded_output(macro_helpers::tool_output_limit::ToolOutputLimit::from(
                    16_384usize
                ))
                .is_ok_and(|output| {
                    output.status.code() == Some(1i32)
                        && std::str::from_utf8(output.stderr.as_slice()).is_ok_and(|stderr| {
                            stderr.contains(constants_str::RUNNER_MEASUREMENT_PREFIX)
                                && stderr.contains(constants_str::CODE_STYLE)
                                && stderr.contains(constants_str::RUNNER_MEASUREMENT_SPAWN_PREFIX)
                        })
                        && std::str::from_utf8(output.stdout.as_slice()).is_ok_and(|stdout| {
                            stdout.contains(constants_str::RUNNER_CLI_TEXT_FCB569D3)
                        })
                })
        );
    }

    #[cfg(unix)]
    #[test]
    #[ignore = "requires provisioned /usr/bin/false, /usr/bin/gnuls and GNU time; creates isolated failing Cargo fixtures"]
    fn test_measurement_cli_failed_cargo_reports_exit_without_spawn_failure() {
        assert!(
            [
                (false, false, constants_str::CODE_STYLE),
                (true, false, constants_str::CODE_STYLE),
                (true, true, constants_str::CLIPPY)
            ]
            .into_iter()
            .all(|(with_output, first_stage_succeeds, expected_stage)| {
                let directory = std::env::temp_dir().join(format!(
                    "{}-{}",
                    constants_str::MEASURE,
                    std::process::id()
                ));
                assert!(
                    std::fs::DirBuilder::new()
                        .create(&directory)
                        .is_ok_and(|()| directory.is_dir())
                );
                let outcome = (|| {
                    let fixture_path = if with_output {
                        std::fs::write(directory.join(constants_str::TEST_ALT_3), constants_str::X)
                            .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                        if first_stage_succeeds {
                            std::fs::write(
                                directory.join(constants_str::TESTS_ALT),
                                constants_str::X,
                            )
                            .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                            std::fs::write(
                                directory.join(constants_str::CODE_STYLE),
                                constants_str::X,
                            )
                            .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                        }
                        std::path::Path::new(constants_str::TEST_FALSE_EXECUTABLE_PATH)
                            .with_file_name(stringify!(gnuls))
                    } else {
                        std::path::PathBuf::from(constants_str::TEST_FALSE_EXECUTABLE_PATH)
                    };
                    std::os::unix::fs::symlink(
                        &fixture_path,
                        directory.join(constants_str::WORKSPACE_TEST_RUNNER_CARGO),
                    )
                    .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                    let search_path = directory.to_str().ok_or_else(|| {
                        macro_helpers::std_tool_io_error::StdToolIoError::from(
                            std::io::Error::from(std::io::ErrorKind::InvalidData),
                        )
                    })?;
                    let mut command = macro_helpers::tool_command::ToolCommand::new(
                        macro_helpers::tool_program_ref::ToolProgramRef::from(env!(
                            "CARGO_BIN_EXE_workspace_test_runner"
                        )),
                    );
                    let _command = command
                        .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
                            constants_str::MEASURE,
                        ))
                        .current_dir(macro_helpers::macro_path_ref::MacroPathRef::from(
                            directory.as_path(),
                        ))
                        .env(
                            macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(
                                constants_str::PATH_ALT,
                            ),
                            macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(search_path),
                        );
                    let output = command
                        .bounded_output(macro_helpers::tool_output_limit::ToolOutputLimit::from(
                            16_384usize,
                        ))
                        .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                    Ok::<_, macro_helpers::std_tool_io_error::StdToolIoError>(
                        output.status.code() == Some(1i32)
                            && std::str::from_utf8(output.stderr.as_slice()).is_ok_and(|stderr| {
                                stderr.contains(constants_str::RUNNER_MEASUREMENT_PREFIX)
                                && stderr.contains(expected_stage)
                                && stderr.contains(constants_str::RUNNER_MEASUREMENT_EXIT_PREFIX)
                                && !stderr.contains(constants_str::RUNNER_MEASUREMENT_SPAWN_PREFIX)
                                && [
                                    constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX,
                                    constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX,
                                    constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX,
                                ]
                                .into_iter()
                                .all(|prefix| !stderr.contains(prefix))
                            })
                            && std::str::from_utf8(output.stdout.as_slice()).is_ok_and(|stdout| {
                                stdout.lines().any(|line| line == constants_str::TEST_ALT_3)
                                    == with_output
                            }),
                    )
                })();
                assert!(std::fs::remove_dir_all(&directory).is_ok_and(|()| !directory.exists()));
                outcome.is_ok_and(|matched| matched)
            })
        );
    }

    #[cfg(unix)]
    #[test]
    #[ignore = "requires provisioned /usr/bin/gnuprintf, /usr/bin/true, /usr/bin/rm and GNU time; uses isolated Cargo output and rustfmt fixtures"]
    fn test_measurement_cli_success_and_output_failures_preserve_stage_diagnostics() {
        assert!(
            [
                None,
                Some(constants_str::RUNNER_CLI_TEXT_C8CCFAD7),
                Some(constants_str::RUNNER_CLI_TEXT_9302A07B),
                Some(constants_str::RUNNER_CLI_TEXT_04C53EE4)
            ]
            .into_iter()
            .all(|failure_diagnostic| {
                let directory = std::env::temp_dir().join(format!(
                    "{}-{}-{}",
                    constants_str::MEASURE,
                    constants_str::ROOT,
                    std::process::id()
                ));
                assert!(
                    std::fs::DirBuilder::new()
                        .create(&directory)
                        .is_ok_and(|()| directory.is_dir())
                );
                let outcome = (|| {
                    [
                        constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                        constants_str::RUSTFMT,
                    ]
                    .into_iter()
                    .try_for_each(|program| {
                        let fixture_path = if program == constants_str::WORKSPACE_TEST_RUNNER_CARGO
                        {
                            std::path::Path::new(constants_str::TEST_TRUE_EXECUTABLE_PATH)
                                .with_file_name(stringify!(gnuprintf))
                        } else if failure_diagnostic
                            == Some(constants_str::RUNNER_CLI_TEXT_04C53EE4)
                        {
                            std::path::Path::new(constants_str::TEST_TRUE_EXECUTABLE_PATH)
                                .with_file_name(stringify!(rm))
                        } else {
                            std::path::PathBuf::from(constants_str::TEST_TRUE_EXECUTABLE_PATH)
                        };
                        std::os::unix::fs::symlink(&fixture_path, directory.join(program))
                            .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)
                    })?;
                    let generated_directory =
                        directory.join(constants_str::TARGET_MEASURE_GENERATE_PG_TABLE_WITH_TESTS);
                    match failure_diagnostic {
                        Some(constants_str::RUNNER_CLI_TEXT_C8CCFAD7) => {
                            let parent = generated_directory.parent().ok_or_else(|| {
                                macro_helpers::std_tool_io_error::StdToolIoError::from(
                                    std::io::Error::from(std::io::ErrorKind::InvalidInput),
                                )
                            })?;
                            std::fs::create_dir_all(parent)
                                .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                            std::fs::write(&generated_directory, constants_str::X)
                                .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                        }
                        Some(constants_str::RUNNER_CLI_TEXT_9302A07B) => {
                            std::fs::create_dir_all(
                                generated_directory.join(constants_str::RUSTFMT_TOML),
                            )
                            .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                        }
                        None | Some(constants_str::RUNNER_CLI_TEXT_04C53EE4) => {}
                        Some(_) => return Ok(false),
                    }
                    let search_path = directory.to_str().ok_or_else(|| {
                        macro_helpers::std_tool_io_error::StdToolIoError::from(
                            std::io::Error::from(std::io::ErrorKind::InvalidData),
                        )
                    })?;
                    let mut command = macro_helpers::tool_command::ToolCommand::new(
                        macro_helpers::tool_program_ref::ToolProgramRef::from(env!(
                            "CARGO_BIN_EXE_workspace_test_runner"
                        )),
                    );
                    let _command = command
                        .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
                            constants_str::MEASURE,
                        ))
                        .current_dir(macro_helpers::macro_path_ref::MacroPathRef::from(
                            directory.as_path(),
                        ))
                        .env(
                            macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(
                                constants_str::PATH_ALT,
                            ),
                            macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(search_path),
                        );
                    let output = command
                        .bounded_output(macro_helpers::tool_output_limit::ToolOutputLimit::from(
                            65_536usize,
                        ))
                        .map_err(macro_helpers::std_tool_io_error::StdToolIoError::from)?;
                    let forwarded_stdout =
                        std::str::from_utf8(output.stdout.as_slice()).is_ok_and(|stdout| {
                            [constants_str::TEST_ALT_3, constants_str::CLIPPY]
                                .into_iter()
                                .all(|program_text| {
                                    stdout.contains(
                                        format!(
                                            "{program_text}{}",
                                            constants_str::RUNNER_MEASUREMENT_PREFIX
                                        )
                                        .as_str(),
                                    )
                                })
                        });
                    let forwarded_stderr =
                        std::str::from_utf8(output.stderr.as_slice()).is_ok_and(|stderr| {
                            !stderr.is_empty()
                                && [
                                    constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX,
                                    constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX,
                                    constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX,
                                ]
                                .into_iter()
                                .all(|prefix| !stderr.contains(prefix))
                        });
                    if !forwarded_stdout || !forwarded_stderr {
                        return Ok(false);
                    }
                    let generated = directory
                        .join(constants_str::TARGET_MEASURE_GENERATE_PG_TABLE_WITH_TESTS)
                        .join(constants_str::GENERATE_PG_TABLE_TESTS_RS);
                    if let Some(diagnostic) = failure_diagnostic {
                        return Ok(output.status.code() == Some(1i32)
                            && !generated.exists()
                            && std::str::from_utf8(output.stderr.as_slice())
                                .is_ok_and(|stderr| stderr.contains(diagnostic))
                            && std::str::from_utf8(output.stdout.as_slice()).is_ok_and(
                                |stdout| {
                                    stdout.contains(constants_str::RUNNER_CLI_TEXT_251828F4)
                                        && !stdout.contains(constants_str::RUNNER_CLI_TEXT_360892CE)
                                },
                            ));
                    }
                    Ok::<_, macro_helpers::std_tool_io_error::StdToolIoError>(
                        output.status.success()
                            && generated
                                .metadata()
                                .is_ok_and(|metadata| metadata.is_file() && metadata.len() > 0u64)
                            && std::str::from_utf8(output.stdout.as_slice()).is_ok_and(|stdout| {
                                [
                                    constants_str::RUNNER_CLI_TEXT_C0746021,
                                    constants_str::RUNNER_CLI_TEXT_251828F4,
                                    constants_str::RUNNER_CLI_TEXT_DBC31E58,
                                    constants_str::RUNNER_CLI_TEXT_B2120D92,
                                    constants_str::RUNNER_CLI_TEXT_360892CE,
                                ]
                                .into_iter()
                                .all(|summary| stdout.contains(summary))
                            }),
                    )
                })();
                assert!(std::fs::remove_dir_all(&directory).is_ok_and(|()| !directory.exists()));
                outcome.is_ok_and(|matched| matched)
            })
        );
    }
}
