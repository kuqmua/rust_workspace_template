#![forbid(unsafe_code)]

use core::{
    fmt::{Display, Formatter, Result as FormattingResult},
    str::FromStr as _,
};
use std::{
    env,
    error::Error,
    process::{ExitCode, Termination},
};

use shared_logic::{CalculationError, CalculationReportFormat};

const ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT: &str = "CALCULATION_REPORT_FORMAT";
const EXIT_CODE_FAILURE: u8 = 2;
const MESSAGE_NON_UNICODE_REPORT_FORMAT_ENVIRONMENT_VARIABLE: &str =
    "environment variable CALCULATION_REPORT_FORMAT contains non-unicode data";
const MESSAGE_INVALID_CONFIGURATION_PREFIX: &str = "invalid configuration:";
const STARTUP_TEXT_MESSAGE: &str = "server started";
const STARTUP_JSON_MESSAGE: &str = "{\"status\":\"started\"}";

#[derive(Debug, Eq, PartialEq)]
enum StartupOutput {
    Json,
    Text,
}

impl Display for StartupOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormattingResult {
        match self {
            Self::Json => write!(f, "{STARTUP_JSON_MESSAGE}"),
            Self::Text => write!(f, "{STARTUP_TEXT_MESSAGE}"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum StartupError {
    Configuration(CalculationError),
    InvalidConfiguration { details: String },
}

impl Display for StartupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormattingResult {
        match self {
            Self::Configuration(configuration_error) => write!(f, "{configuration_error}"),
            Self::InvalidConfiguration { details } => {
                write!(f, "{MESSAGE_INVALID_CONFIGURATION_PREFIX} {details}")
            }
        }
    }
}

impl Error for StartupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Configuration(configuration_error) => Some(configuration_error),
            Self::InvalidConfiguration { .. } => None,
        }
    }
}

impl From<CalculationError> for StartupError {
    fn from(configuration_error: CalculationError) -> Self {
        Self::Configuration(configuration_error)
    }
}

fn main() -> impl Termination {
    let startup_output_result = env::var_os(ENVIRONMENT_VARIABLE_CALCULATION_REPORT_FORMAT).map_or(
        Ok(StartupOutput::Text),
        |environment_value_os| {
            let environment_value_text_result =
                environment_value_os
                    .into_string()
                    .map_err(|_non_unicode_environment_value| StartupError::InvalidConfiguration {
                        details: MESSAGE_NON_UNICODE_REPORT_FORMAT_ENVIRONMENT_VARIABLE.to_owned(),
                    });
            environment_value_text_result
                .and_then(|environment_value_text_value| {
                    CalculationReportFormat::from_str(&environment_value_text_value)
                        .map_err(StartupError::from)
                })
                .map(|calculation_report_format| match calculation_report_format {
                    CalculationReportFormat::Json => StartupOutput::Json,
                    CalculationReportFormat::Text => StartupOutput::Text,
                })
        },
    );

    match startup_output_result {
        Ok(startup_output) => {
            println!("{startup_output}");
            ExitCode::SUCCESS
        }
        Err(startup_error) => {
            eprintln!("{startup_error}");
            ExitCode::from(EXIT_CODE_FAILURE)
        }
    }
}
