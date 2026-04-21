#[cfg(test)]
mod integration_tests {
    use std::process::Output;
    #[cfg(unix)]
    use std::{ffi::OsString, os::unix::ffi::OsStringExt as _};

    use shared_logic as _;
    use test_helpers::{
        run_server_command, run_server_command_with_report_format, stderr_as_utf8, stdout_as_utf8,
    };

    const SERVER_BINARY_PATH: &str = env!("CARGO_BIN_EXE_server");

    fn run_and_decode_stdout(command_line_arguments: &[&str]) -> (Output, String) {
        let output =
            run_server_command(SERVER_BINARY_PATH, command_line_arguments).expect("6f1b2d8a");
        let standard_output = stdout_as_utf8(&output).expect("9c3e1a7d").to_owned();
        (output, standard_output)
    }

    fn run_and_decode_stderr(command_line_arguments: &[&str]) -> (Output, String) {
        let output =
            run_server_command(SERVER_BINARY_PATH, command_line_arguments).expect("3a7d1e9c");
        let standard_error = stderr_as_utf8(&output).expect("1d9c4a7e").to_owned();
        (output, standard_error)
    }

    #[test]
    fn prints_help_without_arguments() {
        let (output, standard_output) = run_and_decode_stdout(&[]);

        assert!(output.status.success());
        assert!(standard_output.contains("Usage:"));
        assert!(standard_output.contains("server --wire-format <left|operation|right>"));
        assert!(standard_output.contains("CALCULATION_REPORT_FORMAT=text|json"));
    }

    #[test]
    fn prints_help_for_short_help_flag() {
        let (output, standard_output) = run_and_decode_stdout(&["-h"]);

        assert!(output.status.success());
        assert!(standard_output.contains("Usage:"));
    }

    #[test]
    fn prints_help_for_long_help_flag() {
        let (output, standard_output) = run_and_decode_stdout(&["--help"]);

        assert!(output.status.success());
        assert!(standard_output.contains("Usage:"));
        assert!(standard_output.contains("server --help"));
    }

    #[test]
    fn prints_identical_help_for_short_and_long_help_flags() {
        let (short_help_output, short_help_standard_output) = run_and_decode_stdout(&["-h"]);
        let (long_help_output, long_help_standard_output) = run_and_decode_stdout(&["--help"]);

        assert!(short_help_output.status.success());
        assert!(long_help_output.status.success());
        assert_eq!(short_help_standard_output.trim_end(), long_help_standard_output.trim_end());
    }

    #[test]
    fn prints_help_even_when_report_format_environment_variable_is_invalid() {
        let output =
            run_server_command_with_report_format(SERVER_BINARY_PATH, &["--help"], "yaml".as_ref())
                .expect("1d7a3e9c");
        let standard_output = stdout_as_utf8(&output).expect("6f2c8a1d");
        let standard_error = stderr_as_utf8(&output).expect("4a9e1d7c");

        assert!(output.status.success());
        assert!(standard_output.contains("Usage:"));
        assert!(standard_error.trim_end().is_empty());
    }

    #[test]
    fn prints_help_for_short_help_flag_when_report_format_environment_variable_is_invalid() {
        let output =
            run_server_command_with_report_format(SERVER_BINARY_PATH, &["-h"], "yaml".as_ref())
                .expect("2e9a4d1c");
        let standard_output = stdout_as_utf8(&output).expect("4f1c8a7d");
        let standard_error = stderr_as_utf8(&output).expect("7a3d1e9c");

        assert!(output.status.success());
        assert!(standard_output.contains("Usage:"));
        assert!(standard_error.trim_end().is_empty());
    }

    #[test]
    fn prints_exact_help_contract() {
        let (output, standard_output) = run_and_decode_stdout(&["--help"]);

        assert!(output.status.success());
        assert_eq!(
            standard_output.trim_end(),
            "Usage:\n  server <left_operand> <operation> <right_operand>\n  server --wire-format \
             <left|operation|right>\n  server --help\nEnvironment: \
             CALCULATION_REPORT_FORMAT=text|json (default: text)"
        );
    }

    #[test]
    fn computes_report_from_position_arguments() {
        let output = run_server_command(SERVER_BINARY_PATH, &["10", "+", "5"]).expect("1c9e4a7d");
        let standard_output = stdout_as_utf8(&output).expect("4a7f1e2c");

        assert!(output.status.success());
        assert!(standard_output.contains("operation=+ left=10 right=5 result=15"));
    }

    #[test]
    fn computes_report_from_wire_format_arguments() {
        let output =
            run_server_command(SERVER_BINARY_PATH, &["--wire-format", "9|*|7"]).expect("7b2f1d9a");
        let standard_output = stdout_as_utf8(&output).expect("2f8d1c4a");

        assert!(output.status.success());
        assert!(standard_output.contains("operation=* left=9 right=7 result=63"));
    }

    #[test]
    fn emits_json_report_when_report_format_environment_variable_is_set() {
        let output = run_server_command_with_report_format(
            SERVER_BINARY_PATH,
            &["10", "+", "5"],
            "json".as_ref(),
        )
        .expect("5c1e8a3d");
        let standard_output = stdout_as_utf8(&output).expect("1f7d4c9a");

        assert!(output.status.success());
        assert_eq!(
            standard_output.trim_end(),
            "{\"operation\":\"+\",\"left\":10,\"right\":5,\"result\":15}"
        );
    }

    #[test]
    fn emits_json_report_for_wire_format_arguments_when_environment_variable_is_set() {
        let output = run_server_command_with_report_format(
            SERVER_BINARY_PATH,
            &["--wire-format", "9|*|7"],
            "json".as_ref(),
        )
        .expect("5f2a8c1d");
        let standard_output = stdout_as_utf8(&output).expect("8a1d3c7e");

        assert!(output.status.success());
        assert_eq!(
            standard_output.trim_end(),
            "{\"operation\":\"*\",\"left\":9,\"right\":7,\"result\":63}"
        );
    }

    #[test]
    fn returns_failure_for_invalid_arguments() {
        let (output, standard_error) = run_and_decode_stderr(&["1", "+", "2", "extra"]);

        assert_eq!(output.status.code(), Some(2i32));
        assert!(standard_error.contains("invalid arguments:"));
        assert!(standard_error.contains("Usage:"));
    }

    #[test]
    fn returns_failure_for_unknown_flag() {
        let (output, standard_error) = run_and_decode_stderr(&["--unknown"]);

        assert_eq!(output.status.code(), Some(2i32));
        assert!(standard_error.contains("invalid arguments:"));
        assert!(standard_error.contains("received 1"));
        assert!(standard_error.contains("Usage:"));
    }

    #[test]
    fn invalid_arguments_error_first_line_contract_is_stable() {
        let (output, standard_error) = run_and_decode_stderr(&["1", "+", "2", "extra"]);
        let first_error_line = standard_error.lines().next().expect("4c8e1a7d");

        assert_eq!(output.status.code(), Some(2i32));
        assert_eq!(
            first_error_line,
            "invalid arguments: expected no args, '--wire-format <value>', or exactly 3 \
             positional args, received 4"
        );
    }

    #[test]
    fn returns_failure_for_unknown_report_format_environment_variable() {
        let output = run_server_command_with_report_format(
            SERVER_BINARY_PATH,
            &["10", "+", "5"],
            "yaml".as_ref(),
        )
        .expect("8b2d6e1a");
        let standard_error = stderr_as_utf8(&output).expect("3e9f1a7c");

        assert_eq!(output.status.code(), Some(2i32));
        assert!(standard_error.contains("unknown calculation report format: yaml"));
        assert!(standard_error.contains("expected 'text' or 'json'"));
    }

    #[test]
    fn unknown_report_format_error_first_line_contract_is_stable() {
        let output = run_server_command_with_report_format(
            SERVER_BINARY_PATH,
            &["10", "+", "5"],
            "yaml".as_ref(),
        )
        .expect("1e7c3a9d");
        let standard_error = stderr_as_utf8(&output).expect("6a1d8c2f");
        let first_error_line = standard_error.lines().next().expect("3b9e1d7a");

        assert_eq!(output.status.code(), Some(2i32));
        assert_eq!(
            first_error_line,
            "unknown calculation report format: yaml; expected 'text' or 'json'"
        );
    }

    #[test]
    fn returns_failure_for_malformed_wire_format_input() {
        let output = run_server_command(SERVER_BINARY_PATH, &["--wire-format", "1|+|2|extra"])
            .expect("7cd12a4e");
        let standard_error = stderr_as_utf8(&output).expect("9c1d7a4e");

        assert_eq!(output.status.code(), Some(2i32));
        assert!(
            standard_error
                .contains("wire format must contain exactly 3 parts separated by '|': 1|+|2|extra")
        );
        assert!(standard_error.contains("Usage:"));
    }

    #[test]
    fn returns_failure_for_division_by_zero_from_position_arguments() {
        let output = run_server_command(SERVER_BINARY_PATH, &["11", "/", "0"]).expect("2f7a4d1c");
        let standard_error = stderr_as_utf8(&output).expect("6e2d9a1f");

        assert_eq!(output.status.code(), Some(2i32));
        assert!(standard_error.contains("division by zero is not allowed"));
        assert!(standard_error.contains("Usage:"));
    }

    #[test]
    fn returns_failure_for_invalid_integer_from_position_arguments() {
        let output =
            run_server_command(SERVER_BINARY_PATH, &["invalid", "+", "5"]).expect("3e9a1d7c");
        let standard_error = stderr_as_utf8(&output).expect("1d8a4f7c");

        assert_eq!(output.status.code(), Some(2i32));
        assert!(standard_error.contains("invalid integer value: invalid"));
        assert!(standard_error.contains("Usage:"));
    }

    #[test]
    fn returns_failure_for_unknown_operation_from_wire_format_arguments() {
        let output =
            run_server_command(SERVER_BINARY_PATH, &["--wire-format", "10|%|5"]).expect("7a2f1c9d");
        let standard_error = stderr_as_utf8(&output).expect("2d7e1c8f");

        assert_eq!(output.status.code(), Some(2i32));
        assert!(standard_error.contains("unknown arithmetic operation: %"));
        assert!(standard_error.contains("Usage:"));
    }

    #[test]
    fn returns_failure_for_overflow_from_wire_format_arguments() {
        let output =
            run_server_command(SERVER_BINARY_PATH, &["--wire-format", "-9223372036854775808|/|-1"])
                .expect("5b1e7c2d");
        let standard_error = stderr_as_utf8(&output).expect("8c2f1a7d");

        assert_eq!(output.status.code(), Some(2i32));
        assert!(standard_error.contains("arithmetic overflow while evaluating operation"));
        assert!(standard_error.contains("Usage:"));
    }

    #[cfg(unix)]
    #[test]
    fn returns_failure_for_non_unicode_report_format_environment_variable() {
        let non_unicode_report_format = OsString::from_vec(vec![0xff, 0xfe]);
        let output = run_server_command_with_report_format(
            SERVER_BINARY_PATH,
            &["10", "+", "5"],
            &non_unicode_report_format,
        )
        .expect("4b7d2e1c");
        let standard_error = stderr_as_utf8(&output).expect("7e3a1c9d");

        assert_eq!(output.status.code(), Some(2i32));
        assert!(
            standard_error.contains(
                "environment variable CALCULATION_REPORT_FORMAT contains non-unicode data"
            )
        );
        assert!(standard_error.contains("Usage:"));
    }

    #[cfg(unix)]
    #[test]
    fn prints_help_with_non_unicode_report_format_environment_variable() {
        let non_unicode_report_format = OsString::from_vec(vec![0xff, 0xfe]);
        let output = run_server_command_with_report_format(
            SERVER_BINARY_PATH,
            &["--help"],
            &non_unicode_report_format,
        )
        .expect("9b3e1a7c");
        let standard_output = stdout_as_utf8(&output).expect("2e8d1c7a");
        let standard_error = stderr_as_utf8(&output).expect("7d1a3e9c");

        assert!(output.status.success());
        assert!(standard_output.contains("Usage:"));
        assert!(standard_error.trim_end().is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn prints_help_for_short_help_flag_with_non_unicode_report_format_environment_variable() {
        let non_unicode_report_format = OsString::from_vec(vec![0xff, 0xfe]);
        let output = run_server_command_with_report_format(
            SERVER_BINARY_PATH,
            &["-h"],
            &non_unicode_report_format,
        )
        .expect("1c8f4a2d");
        let standard_output = stdout_as_utf8(&output).expect("8d2e1a7c");
        let standard_error = stderr_as_utf8(&output).expect("3f7a1d9c");

        assert!(output.status.success());
        assert!(standard_output.contains("Usage:"));
        assert!(standard_error.trim_end().is_empty());
    }
}
