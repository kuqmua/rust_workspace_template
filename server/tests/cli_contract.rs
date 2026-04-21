#[cfg(test)]
mod integration_tests {
    #[cfg(unix)]
    use std::{ffi::OsString, os::unix::ffi::OsStringExt as _};

    use shared_logic as _;
    use test_helpers::{
        run_server_command, run_server_command_with_report_format, stderr_as_utf8, stdout_as_utf8,
    };

    const SERVER_BINARY_PATH: &str = env!("CARGO_BIN_EXE_server");

    #[test]
    fn prints_help_without_arguments() {
        let output = run_server_command(SERVER_BINARY_PATH, &[]).expect("8e1c4d7a");
        let standard_output = stdout_as_utf8(&output).expect("4d8a1f2c");

        assert!(output.status.success());
        assert!(standard_output.contains("Usage:"));
        assert!(standard_output.contains("server --wire-format <left|operation|right>"));
        assert!(standard_output.contains("CALCULATION_REPORT_FORMAT=text|json"));
    }

    #[test]
    fn prints_help_for_short_help_flag() {
        let output = run_server_command(SERVER_BINARY_PATH, &["-h"]).expect("7f1d3a9c");
        let standard_output = stdout_as_utf8(&output).expect("3c8a1e7d");

        assert!(output.status.success());
        assert!(standard_output.contains("Usage:"));
    }

    #[test]
    fn prints_help_for_long_help_flag() {
        let output = run_server_command(SERVER_BINARY_PATH, &["--help"]).expect("9a7e1c3d");
        let standard_output = stdout_as_utf8(&output).expect("5d1c8a4f");

        assert!(output.status.success());
        assert!(standard_output.contains("Usage:"));
        assert!(standard_output.contains("server --help"));
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
        let output =
            run_server_command(SERVER_BINARY_PATH, &["1", "+", "2", "extra"]).expect("2a8d4f1e");
        let standard_error = stderr_as_utf8(&output).expect("6d1c8a3f");

        assert_eq!(output.status.code(), Some(2i32));
        assert!(standard_error.contains("invalid arguments:"));
        assert!(standard_error.contains("Usage:"));
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
}
