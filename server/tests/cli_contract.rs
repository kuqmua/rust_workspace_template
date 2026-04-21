#[cfg(test)]
mod integration_tests {
    use std::process::{Command, Output};

    use shared_logic as _;

    const ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT: &str = "CALCULATION_REPORT_FORMAT";

    fn run_server_command(command_line_arguments: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_server"))
            .args(command_line_arguments)
            .output()
            .expect("8e1c4d7a")
    }

    fn run_server_command_with_report_format(
        command_line_arguments: &[&str],
        report_format: &str,
    ) -> Output {
        Command::new(env!("CARGO_BIN_EXE_server"))
            .env(ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT, report_format)
            .args(command_line_arguments)
            .output()
            .expect("9a3e7c1d")
    }

    #[test]
    fn prints_help_without_arguments() {
        let output = run_server_command(&[]);
        let standard_output = String::from_utf8(output.stdout).expect("4d8a1f2c");

        assert!(output.status.success());
        assert!(standard_output.contains("Usage:"));
        assert!(standard_output.contains("server --wire-format <left|operation|right>"));
        assert!(standard_output.contains("CALCULATION_REPORT_FORMAT=text|json"));
    }

    #[test]
    fn prints_help_for_short_help_flag() {
        let output = run_server_command(&["-h"]);
        let standard_output = String::from_utf8(output.stdout).expect("7f1d3a9c");

        assert!(output.status.success());
        assert!(standard_output.contains("Usage:"));
    }

    #[test]
    fn computes_report_from_position_arguments() {
        let output = run_server_command(&["10", "+", "5"]);
        let standard_output = String::from_utf8(output.stdout).expect("1c9e4a7d");

        assert!(output.status.success());
        assert!(standard_output.contains("operation=+ left=10 right=5 result=15"));
    }

    #[test]
    fn computes_report_from_wire_format_arguments() {
        let output = run_server_command(&["--wire-format", "9|*|7"]);
        let standard_output = String::from_utf8(output.stdout).expect("7b2f1d9a");

        assert!(output.status.success());
        assert!(standard_output.contains("operation=* left=9 right=7 result=63"));
    }

    #[test]
    fn emits_json_report_when_report_format_environment_variable_is_set() {
        let output = run_server_command_with_report_format(&["10", "+", "5"], "json");
        let standard_output = String::from_utf8(output.stdout).expect("5c1e8a3d");

        assert!(output.status.success());
        assert_eq!(
            standard_output.trim_end(),
            "{\"operation\":\"+\",\"left\":10,\"right\":5,\"result\":15}"
        );
    }

    #[test]
    fn returns_failure_for_invalid_arguments() {
        let output = run_server_command(&["1", "+", "2", "extra"]);
        let standard_error = String::from_utf8(output.stderr).expect("2a8d4f1e");

        assert_eq!(output.status.code(), Some(2i32));
        assert!(standard_error.contains("invalid arguments:"));
        assert!(standard_error.contains("Usage:"));
    }

    #[test]
    fn returns_failure_for_unknown_report_format_environment_variable() {
        let output = run_server_command_with_report_format(&["10", "+", "5"], "yaml");
        let standard_error = String::from_utf8(output.stderr).expect("8b2d6e1a");

        assert_eq!(output.status.code(), Some(2i32));
        assert!(standard_error.contains("unknown calculation report format: yaml"));
        assert!(standard_error.contains("expected 'text' or 'json'"));
    }
}
