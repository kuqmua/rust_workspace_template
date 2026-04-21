use std::{
    ffi::OsStr,
    io::Error as IoError,
    path::PathBuf,
    process::{Command, Output},
    str::{Utf8Error, from_utf8},
};

const ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT: &str = "CALCULATION_REPORT_FORMAT";

#[must_use]
pub fn build_calculation_report_golden_file_path(manifest_directory: &str) -> PathBuf {
    PathBuf::from(manifest_directory)
        .join("tests")
        .join("golden")
        .join("calculation_report.txt")
}

#[must_use]
pub const fn build_standard_division_operands() -> (i64, i64) {
    (27, 3)
}

#[must_use]
pub const fn build_standard_operand_text_triplet() -> (&'static str, &'static str, &'static str) {
    ("27", "/", "3")
}

#[must_use]
pub const fn build_invalid_wire_format_with_extra_parts() -> &'static str {
    "1|+|2|extra"
}

pub fn run_server_command(
    server_binary_path: &str,
    command_line_arguments: &[&str],
) -> Result<Output, IoError> {
    run_binary_command(server_binary_path, command_line_arguments)
}

pub fn run_server_command_with_report_format(
    server_binary_path: &str,
    command_line_arguments: &[&str],
    report_format: &OsStr,
) -> Result<Output, IoError> {
    run_binary_command_with_environment_variable(
        server_binary_path,
        command_line_arguments,
        ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT,
        report_format,
    )
}

pub fn stdout_as_utf8(output: &Output) -> Result<&str, Utf8Error> {
    from_utf8(&output.stdout)
}

pub fn stderr_as_utf8(output: &Output) -> Result<&str, Utf8Error> {
    from_utf8(&output.stderr)
}

pub fn run_binary_command(
    binary_path: &str,
    command_line_arguments: &[&str],
) -> Result<Output, IoError> {
    Command::new(binary_path)
        .args(command_line_arguments)
        .output()
}

pub fn run_binary_command_with_environment_variable(
    binary_path: &str,
    command_line_arguments: &[&str],
    environment_variable_name: &str,
    environment_variable_value: &OsStr,
) -> Result<Output, IoError> {
    Command::new(binary_path)
        .env(environment_variable_name, environment_variable_value)
        .args(command_line_arguments)
        .output()
}
