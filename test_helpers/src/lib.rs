#![forbid(unsafe_code)]

use std::{
    ffi::OsStr,
    io::Error as IoError,
    process::{Command, Output},
    str::{Utf8Error, from_utf8},
};

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
