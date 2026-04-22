#![forbid(unsafe_code)]

use std::{
    ffi::OsStr,
    io::Error as IoError,
    path::PathBuf,
    process::{Command, Output},
    str::{Utf8Error, from_utf8},
};

const ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT: &str = "CALCULATION_REPORT_FORMAT";
const ENVIRONMENT_VARIABLE_LANGUAGE: &str = "LANG";
const ENVIRONMENT_VARIABLE_LOCALE_ALL: &str = "LC_ALL";
const ENVIRONMENT_VARIABLE_LOCALE_NUMERIC: &str = "LC_NUMERIC";
const ENVIRONMENT_VARIABLE_LOCALE_TIME: &str = "LC_TIME";
const ENVIRONMENT_VARIABLE_TIMEZONE: &str = "TZ";
const DETERMINISTIC_LOCALE_VALUE: &str = "C";
const DETERMINISTIC_TIMEZONE_VALUE: &str = "UTC";

fn configure_deterministic_process_environment(command: &mut Command) -> &mut Command {
    command
        .env(ENVIRONMENT_VARIABLE_LOCALE_ALL, DETERMINISTIC_LOCALE_VALUE)
        .env(ENVIRONMENT_VARIABLE_LOCALE_NUMERIC, DETERMINISTIC_LOCALE_VALUE)
        .env(ENVIRONMENT_VARIABLE_LOCALE_TIME, DETERMINISTIC_LOCALE_VALUE)
        .env(ENVIRONMENT_VARIABLE_LANGUAGE, DETERMINISTIC_LOCALE_VALUE)
        .env(ENVIRONMENT_VARIABLE_TIMEZONE, DETERMINISTIC_TIMEZONE_VALUE)
}

#[must_use = "golden-file path must be used by test caller"]
pub fn build_calculation_report_golden_file_path(manifest_directory: &str) -> PathBuf {
    PathBuf::from(manifest_directory)
        .join("tests")
        .join("golden")
        .join("calculation_report.txt")
}

#[must_use = "standard deterministic division operands must be used by test caller"]
pub const fn build_standard_division_operands() -> (i64, i64) {
    (27, 3)
}

#[must_use = "standard operand triplet must be used by test caller"]
pub const fn build_standard_operand_text_triplet() -> (&'static str, &'static str, &'static str) {
    ("27", "/", "3")
}

#[must_use = "standard positional and wire-format operation cases must be used by test caller"]
pub const fn build_standard_positional_and_wire_format_operation_cases()
-> [(&'static str, &'static str, &'static str, &'static str); 4] {
    [
        ("11", "+", "7", "11|+|7"),
        ("11", "-", "7", "11|-|7"),
        ("11", "*", "7", "11|*|7"),
        ("21", "/", "7", "21|/|7"),
    ]
}

#[must_use = "invalid wire-format fixture must be used by test caller"]
pub const fn build_invalid_wire_format_with_extra_parts() -> &'static str {
    "1|+|2|extra"
}

#[must_use = "command execution result must be handled by test caller"]
pub fn run_server_command(
    server_binary_path: &str,
    command_line_arguments: &[&str],
) -> Result<Output, IoError> {
    let mut server_command = Command::new(server_binary_path);
    configure_deterministic_process_environment(&mut server_command)
        .env_remove(ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT)
        .args(command_line_arguments)
        .output()
}

#[must_use = "command execution result must be handled by test caller"]
pub fn run_server_command_with_os_arguments(
    server_binary_path: &str,
    command_line_arguments: &[&OsStr],
) -> Result<Output, IoError> {
    let mut server_command = Command::new(server_binary_path);
    configure_deterministic_process_environment(&mut server_command)
        .env_remove(ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT)
        .args(command_line_arguments)
        .output()
}

#[must_use = "command execution result must be handled by test caller"]
pub fn run_server_command_with_os_arguments_and_report_format(
    server_binary_path: &str,
    command_line_arguments: &[&OsStr],
    report_format: &OsStr,
) -> Result<Output, IoError> {
    let mut server_command = Command::new(server_binary_path);
    configure_deterministic_process_environment(&mut server_command)
        .env_remove(ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT)
        .env(ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT, report_format)
        .args(command_line_arguments)
        .output()
}

#[must_use = "command execution result must be handled by test caller"]
pub fn run_server_command_with_report_format(
    server_binary_path: &str,
    command_line_arguments: &[&str],
    report_format: &OsStr,
) -> Result<Output, IoError> {
    let mut server_command = Command::new(server_binary_path);
    configure_deterministic_process_environment(&mut server_command)
        .env_remove(ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT)
        .env(ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT, report_format)
        .args(command_line_arguments)
        .output()
}

#[must_use = "stdout decoding result must be handled by test caller"]
pub fn stdout_as_utf8(output: &Output) -> Result<&str, Utf8Error> {
    from_utf8(&output.stdout)
}

#[must_use = "stdout line decoding result must be handled by test caller"]
pub fn stdout_lines(output: &Output) -> Result<Vec<&str>, Utf8Error> {
    stdout_as_utf8(output).map(|standard_output| standard_output.lines().collect())
}

#[must_use = "first stdout line decoding result must be handled by test caller"]
pub fn first_stdout_line(output: &Output) -> Result<Option<&str>, Utf8Error> {
    stdout_as_utf8(output).map(|standard_output| standard_output.lines().next())
}

#[must_use = "stderr decoding result must be handled by test caller"]
pub fn stderr_as_utf8(output: &Output) -> Result<&str, Utf8Error> {
    from_utf8(&output.stderr)
}

#[must_use = "stderr line decoding result must be handled by test caller"]
pub fn stderr_lines(output: &Output) -> Result<Vec<&str>, Utf8Error> {
    stderr_as_utf8(output).map(|standard_error| standard_error.lines().collect())
}

#[must_use = "first stderr line decoding result must be handled by test caller"]
pub fn first_stderr_line(output: &Output) -> Result<Option<&str>, Utf8Error> {
    stderr_as_utf8(output).map(|standard_error| standard_error.lines().next())
}

#[must_use = "command execution result must be handled by test caller"]
pub fn run_binary_command(
    binary_path: &str,
    command_line_arguments: &[&str],
) -> Result<Output, IoError> {
    let mut binary_command = Command::new(binary_path);
    configure_deterministic_process_environment(&mut binary_command)
        .args(command_line_arguments)
        .output()
}

#[must_use = "command execution result must be handled by test caller"]
pub fn run_binary_command_with_os_arguments(
    binary_path: &str,
    command_line_arguments: &[&OsStr],
) -> Result<Output, IoError> {
    let mut binary_command = Command::new(binary_path);
    configure_deterministic_process_environment(&mut binary_command)
        .args(command_line_arguments)
        .output()
}

#[must_use = "command execution result must be handled by test caller"]
pub fn run_binary_command_with_environment_variable(
    binary_path: &str,
    command_line_arguments: &[&str],
    environment_variable_name: &str,
    environment_variable_value: &OsStr,
) -> Result<Output, IoError> {
    let mut binary_command = Command::new(binary_path);
    configure_deterministic_process_environment(&mut binary_command)
        .env(environment_variable_name, environment_variable_value)
        .args(command_line_arguments)
        .output()
}
