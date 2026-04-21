use core::{
    fmt::{Display, Formatter, Result as FormattingResult},
    str::FromStr as _,
};
use std::{
    env,
    error::Error,
    process::{ExitCode, Termination},
};

use shared_logic::{
    CalculationError, CalculationReportFormat, build_calculation_request_from_text_parts,
    deserialize_calculation_request_from_wire_format, render_calculation_report_with_format,
};
#[cfg(test)]
use test_helpers as _;

const EXIT_CODE_FAILURE: u8 = 2;
const ARGUMENT_FLAG_HELP_LONG: &str = "--help";
const ARGUMENT_FLAG_HELP_SHORT: &str = "-h";
const ARGUMENT_FLAG_WIRE_FORMAT: &str = "--wire-format";
const ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT: &str = "CALCULATION_REPORT_FORMAT";
const MESSAGE_USAGE_HEADER: &str = "Usage:";
const MESSAGE_USAGE_POSITIONAL: &str = "  server <left_operand> <operation> <right_operand>";
const MESSAGE_USAGE_WIRE_FORMAT: &str = "  server --wire-format <left|operation|right>";
const MESSAGE_USAGE_HELP: &str = "  server --help";
const MESSAGE_USAGE_ENVIRONMENT: &str =
    "Environment: CALCULATION_REPORT_FORMAT=text|json (default: text)";
const MESSAGE_INVALID_ARGUMENTS_PREFIX: &str = "invalid arguments:";

#[derive(Debug, Eq, PartialEq)]
enum StartupOutput {
    CalculationReport { report: String },
    HelpText,
}

#[derive(Debug, Eq, PartialEq)]
enum StartupError {
    Calculation(CalculationError),
    InvalidArguments { details: String },
}

impl Display for StartupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormattingResult {
        match self {
            Self::Calculation(calculation_error) => write!(f, "{calculation_error}"),
            Self::InvalidArguments { details } => {
                write!(f, "{MESSAGE_INVALID_ARGUMENTS_PREFIX} {details}")
            }
        }
    }
}

impl Error for StartupError {}

impl From<CalculationError> for StartupError {
    fn from(calculation_error: CalculationError) -> Self {
        Self::Calculation(calculation_error)
    }
}

fn main() -> impl Termination {
    let command_line_arguments: Vec<String> = env::args().skip(1).collect();
    let startup_output_result = match command_line_arguments.as_slice() {
        [] => Ok(StartupOutput::HelpText),
        [first_argument]
            if first_argument == ARGUMENT_FLAG_HELP_LONG
                || first_argument == ARGUMENT_FLAG_HELP_SHORT =>
        {
            Ok(StartupOutput::HelpText)
        }
        _ => {
            let calculation_report_format_result = {
                let environment_value_result =
                    env::var(ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT);
                match environment_value_result {
                    Ok(environment_value) => CalculationReportFormat::from_str(&environment_value)
                        .map_err(StartupError::from),
                    Err(env::VarError::NotPresent) => Ok(CalculationReportFormat::Text),
                    Err(env::VarError::NotUnicode(_non_unicode_environment_value)) => {
                        Err(StartupError::InvalidArguments {
                            details: format!(
                                "environment variable \
                                 {ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT} contains \
                                 non-unicode data"
                            ),
                        })
                    }
                }
            };

            calculation_report_format_result.and_then(|calculation_report_format| {
                match command_line_arguments.as_slice() {
                    [wire_format_flag, wire_format_value]
                        if wire_format_flag == ARGUMENT_FLAG_WIRE_FORMAT =>
                    {
                        deserialize_calculation_request_from_wire_format(wire_format_value)
                            .and_then(|calculation_request| {
                                render_calculation_report_with_format(
                                    &calculation_request,
                                    calculation_report_format,
                                )
                            })
                            .map(|report| StartupOutput::CalculationReport { report })
                            .map_err(StartupError::from)
                    }
                    [
                        left_operand_text,
                        arithmetic_operation_text,
                        right_operand_text,
                    ] => build_calculation_request_from_text_parts(
                        left_operand_text,
                        arithmetic_operation_text,
                        right_operand_text,
                    )
                    .and_then(|calculation_request| {
                        render_calculation_report_with_format(
                            &calculation_request,
                            calculation_report_format,
                        )
                    })
                    .map(|report| StartupOutput::CalculationReport { report })
                    .map_err(StartupError::from),
                    _ => Err(StartupError::InvalidArguments {
                        details: format!(
                            "expected no args, '--wire-format <value>', or exactly 3 positional \
                             args, received {}",
                            command_line_arguments.len()
                        ),
                    }),
                }
            })
        }
    };

    match startup_output_result {
        Ok(StartupOutput::HelpText) => {
            println!("{}", build_usage_message());
            ExitCode::SUCCESS
        }
        Ok(StartupOutput::CalculationReport { report }) => {
            println!("{report}");
            ExitCode::SUCCESS
        }
        Err(startup_error) => {
            eprintln!("{startup_error}");
            eprintln!("{}", build_usage_message());
            ExitCode::from(EXIT_CODE_FAILURE)
        }
    }
}

fn build_usage_message() -> String {
    [
        MESSAGE_USAGE_HEADER,
        MESSAGE_USAGE_POSITIONAL,
        MESSAGE_USAGE_WIRE_FORMAT,
        MESSAGE_USAGE_HELP,
        MESSAGE_USAGE_ENVIRONMENT,
    ]
    .join("\n")
}

#[cfg(test)]
mod unit_tests {
    use super::build_usage_message;

    #[test]
    fn usage_message_contains_all_documented_forms() {
        let usage_message = build_usage_message();

        assert!(usage_message.contains("server <left_operand> <operation> <right_operand>"));
        assert!(usage_message.contains("server --wire-format <left|operation|right>"));
        assert!(usage_message.contains("server --help"));
        assert!(usage_message.contains("CALCULATION_REPORT_FORMAT=text|json"));
    }

    #[test]
    fn usage_message_has_stable_exact_contract() {
        let usage_message = build_usage_message();

        assert_eq!(
            usage_message,
            "Usage:\n  server <left_operand> <operation> <right_operand>\n  server --wire-format \
             <left|operation|right>\n  server --help\nEnvironment: \
             CALCULATION_REPORT_FORMAT=text|json (default: text)"
        );
    }
}
