use core::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use thiserror::Error;

const ARITHMETIC_SYMBOL_ADDITION: &str = "+";
const ARITHMETIC_SYMBOL_DIVISION: &str = "/";
const ARITHMETIC_SYMBOL_MULTIPLICATION: &str = "*";
const ARITHMETIC_SYMBOL_SUBTRACTION: &str = "-";
const CALCULATION_REPORT_FORMAT_JSON: &str = "json";
const CALCULATION_REPORT_FORMAT_TEXT: &str = "text";
const WIRE_FORMAT_DELIMITER: char = '|';

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArithmeticOperation {
    Addition,
    Division,
    Multiplication,
    Subtraction,
}

impl Display for ArithmeticOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::Addition => ARITHMETIC_SYMBOL_ADDITION,
            Self::Division => ARITHMETIC_SYMBOL_DIVISION,
            Self::Multiplication => ARITHMETIC_SYMBOL_MULTIPLICATION,
            Self::Subtraction => ARITHMETIC_SYMBOL_SUBTRACTION,
        };
        write!(f, "{symbol}")
    }
}

impl FromStr for ArithmeticOperation {
    type Err = CalculationError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source {
            ARITHMETIC_SYMBOL_ADDITION => Ok(Self::Addition),
            ARITHMETIC_SYMBOL_DIVISION => Ok(Self::Division),
            ARITHMETIC_SYMBOL_MULTIPLICATION => Ok(Self::Multiplication),
            ARITHMETIC_SYMBOL_SUBTRACTION => Ok(Self::Subtraction),
            _ => Err(CalculationError::UnknownOperation {
                provided_operation: source.to_owned(),
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CalculationReportFormat {
    Json,
    Text,
}

impl FromStr for CalculationReportFormat {
    type Err = CalculationError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source {
            CALCULATION_REPORT_FORMAT_JSON => Ok(Self::Json),
            CALCULATION_REPORT_FORMAT_TEXT => Ok(Self::Text),
            _ => Err(CalculationError::UnknownReportFormat {
                provided_format: source.to_owned(),
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalculationRequest {
    pub arithmetic_operation: ArithmeticOperation,
    pub left_operand: i64,
    pub right_operand: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalculationResult {
    pub value: i64,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CalculationError {
    #[error("division by zero is not allowed")]
    DivisionByZero,
    #[error("invalid integer value: {provided_value}")]
    InvalidIntegerValue { provided_value: String },
    #[error("wire format must contain exactly 3 parts separated by '|': {provided_wire_format}")]
    MalformedWireFormat { provided_wire_format: String },
    #[error("arithmetic overflow while evaluating operation")]
    Overflow,
    #[error("unknown arithmetic operation: {provided_operation}")]
    UnknownOperation { provided_operation: String },
    #[error("unknown calculation report format: {provided_format}; expected 'text' or 'json'")]
    UnknownReportFormat { provided_format: String },
}

/// Evaluate a request using checked arithmetic.
///
/// # Examples
///
/// ```
/// use shared_logic::{ArithmeticOperation, CalculationRequest, evaluate_calculation_request};
///
/// let request = CalculationRequest {
///     arithmetic_operation: ArithmeticOperation::Division,
///     left_operand: 9,
///     right_operand: 3,
/// };
///
/// let result = evaluate_calculation_request(&request).expect("e7d0a1c2");
/// assert_eq!(result.value, 3);
/// ```
///
/// ```compile_fail
/// use shared_logic::{
///     CalculationRequest, evaluate_calculation_request,
/// };
///
/// let request = CalculationRequest {
///     arithmetic_operation: true,
///     left_operand: 1,
///     right_operand: 2,
/// };
/// let _ = evaluate_calculation_request(&request);
/// ```
pub fn evaluate_calculation_request(
    calculation_request: &CalculationRequest,
) -> Result<CalculationResult, CalculationError> {
    let value = match calculation_request.arithmetic_operation {
        ArithmeticOperation::Addition => calculation_request
            .left_operand
            .checked_add(calculation_request.right_operand)
            .ok_or(CalculationError::Overflow)?,
        ArithmeticOperation::Division => {
            if calculation_request.right_operand == 0 {
                return Err(CalculationError::DivisionByZero);
            }
            calculation_request
                .left_operand
                .checked_div(calculation_request.right_operand)
                .ok_or(CalculationError::Overflow)?
        }
        ArithmeticOperation::Multiplication => calculation_request
            .left_operand
            .checked_mul(calculation_request.right_operand)
            .ok_or(CalculationError::Overflow)?,
        ArithmeticOperation::Subtraction => calculation_request
            .left_operand
            .checked_sub(calculation_request.right_operand)
            .ok_or(CalculationError::Overflow)?,
    };

    Ok(CalculationResult { value })
}

pub fn build_calculation_request_from_text_parts(
    left_operand_text: &str,
    arithmetic_operation_text: &str,
    right_operand_text: &str,
) -> Result<CalculationRequest, CalculationError> {
    let left_operand = left_operand_text
        .parse::<i64>()
        .map_err(|_parse_integer_error| CalculationError::InvalidIntegerValue {
            provided_value: left_operand_text.to_owned(),
        })?;
    let right_operand = right_operand_text
        .parse::<i64>()
        .map_err(|_parse_integer_error| CalculationError::InvalidIntegerValue {
            provided_value: right_operand_text.to_owned(),
        })?;
    let arithmetic_operation = ArithmeticOperation::from_str(arithmetic_operation_text)?;

    Ok(CalculationRequest {
        arithmetic_operation,
        left_operand,
        right_operand,
    })
}

#[must_use]
pub fn serialize_calculation_request_to_wire_format(
    calculation_request: &CalculationRequest,
) -> String {
    format!(
        "{}{}{}{}{}",
        calculation_request.left_operand,
        WIRE_FORMAT_DELIMITER,
        calculation_request.arithmetic_operation,
        WIRE_FORMAT_DELIMITER,
        calculation_request.right_operand
    )
}

pub fn deserialize_calculation_request_from_wire_format(
    wire_format: &str,
) -> Result<CalculationRequest, CalculationError> {
    let mut wire_format_parts = wire_format.split(WIRE_FORMAT_DELIMITER);
    let left_operand_part = wire_format_parts
        .next()
        .ok_or_else(|| build_malformed_wire_format_error(wire_format))?;
    let operation_part = wire_format_parts
        .next()
        .ok_or_else(|| build_malformed_wire_format_error(wire_format))?;
    let right_operand_part = wire_format_parts
        .next()
        .ok_or_else(|| build_malformed_wire_format_error(wire_format))?;

    if wire_format_parts.next().is_some() {
        return Err(build_malformed_wire_format_error(wire_format));
    }

    let calculation_request = build_calculation_request_from_text_parts(
        left_operand_part,
        operation_part,
        right_operand_part,
    )?;
    Ok(calculation_request)
}

pub fn render_calculation_report(
    calculation_request: &CalculationRequest,
) -> Result<String, CalculationError> {
    render_calculation_report_with_format(calculation_request, CalculationReportFormat::Text)
}

pub fn render_calculation_report_with_format(
    calculation_request: &CalculationRequest,
    calculation_report_format: CalculationReportFormat,
) -> Result<String, CalculationError> {
    let calculation_result = evaluate_calculation_request(calculation_request)?;
    let rendered_report = match calculation_report_format {
        CalculationReportFormat::Json => format!(
            "{{\"operation\":\"{}\",\"left\":{},\"right\":{},\"result\":{}}}",
            calculation_request.arithmetic_operation,
            calculation_request.left_operand,
            calculation_request.right_operand,
            calculation_result.value
        ),
        CalculationReportFormat::Text => format!(
            "operation={} left={} right={} result={}",
            calculation_request.arithmetic_operation,
            calculation_request.left_operand,
            calculation_request.right_operand,
            calculation_result.value
        ),
    };

    Ok(rendered_report)
}

fn build_malformed_wire_format_error(wire_format: &str) -> CalculationError {
    CalculationError::MalformedWireFormat {
        provided_wire_format: wire_format.to_owned(),
    }
}

#[cfg(test)]
mod unit_tests {
    use core::str::FromStr as _;

    use test_helpers::{build_standard_division_operands, build_standard_operand_text_triplet};

    use super::{
        ArithmeticOperation, CalculationError, CalculationReportFormat, CalculationRequest,
        build_calculation_request_from_text_parts,
        deserialize_calculation_request_from_wire_format, evaluate_calculation_request,
        render_calculation_report_with_format, serialize_calculation_request_to_wire_format,
    };
    use crate::render_calculation_report;

    #[test]
    fn evaluates_addition() {
        let calculation_request = CalculationRequest {
            arithmetic_operation: ArithmeticOperation::Addition,
            left_operand: 8,
            right_operand: 13,
        };

        let calculation_result =
            evaluate_calculation_request(&calculation_request).expect("8d3b6f1a");
        assert_eq!(calculation_result.value, 21);
    }

    #[test]
    fn returns_division_by_zero_error() {
        let (left_operand, right_operand) = build_standard_division_operands();
        let calculation_request = CalculationRequest {
            arithmetic_operation: ArithmeticOperation::Division,
            left_operand,
            right_operand: right_operand - right_operand,
        };

        let calculation_error =
            evaluate_calculation_request(&calculation_request).expect_err("1f9a2c4e");
        assert_eq!(calculation_error, CalculationError::DivisionByZero);
    }

    #[test]
    fn returns_overflow_error() {
        let calculation_request = CalculationRequest {
            arithmetic_operation: ArithmeticOperation::Addition,
            left_operand: i64::MAX,
            right_operand: 1,
        };

        let calculation_error =
            evaluate_calculation_request(&calculation_request).expect_err("5b1d7e9c");
        assert_eq!(calculation_error, CalculationError::Overflow);
    }

    #[test]
    fn serializes_and_deserializes_wire_format() {
        let calculation_request = CalculationRequest {
            arithmetic_operation: ArithmeticOperation::Multiplication,
            left_operand: -7,
            right_operand: 5,
        };

        let wire_format = serialize_calculation_request_to_wire_format(&calculation_request);
        let deserialized_calculation_request =
            deserialize_calculation_request_from_wire_format(&wire_format).expect("c4a9d2f8");

        assert_eq!(deserialized_calculation_request, calculation_request);
    }

    #[test]
    fn builds_calculation_request_from_text_parts() {
        let (left_operand_text, arithmetic_operation_text, right_operand_text) =
            build_standard_operand_text_triplet();

        let calculation_request = build_calculation_request_from_text_parts(
            left_operand_text,
            arithmetic_operation_text,
            right_operand_text,
        )
        .expect("2f8d4c1a");
        let rendered_report = render_calculation_report(&calculation_request).expect("3a9d7e1c");

        assert_eq!(rendered_report, "operation=/ left=27 right=3 result=9");
    }

    #[test]
    fn returns_malformed_wire_format_error_for_too_many_parts() {
        let calculation_error =
            deserialize_calculation_request_from_wire_format("1|+|2|extra").expect_err("8a1c4d7e");

        assert_eq!(calculation_error, CalculationError::MalformedWireFormat {
            provided_wire_format: "1|+|2|extra".to_owned(),
        });
    }

    #[test]
    fn preserves_malformed_wire_format_error_message_contract() {
        let calculation_error =
            deserialize_calculation_request_from_wire_format("1|+|2|extra").expect_err("4be9d2a1");

        assert_eq!(
            calculation_error.to_string(),
            "wire format must contain exactly 3 parts separated by '|': 1|+|2|extra"
        );
    }

    #[test]
    fn returns_unknown_operation_error_for_invalid_symbol() {
        let calculation_error = ArithmeticOperation::from_str("^").expect_err("6e2b9f1d");

        assert_eq!(calculation_error, CalculationError::UnknownOperation {
            provided_operation: "^".to_owned(),
        });
    }

    #[test]
    fn parses_text_report_format() {
        let calculation_report_format =
            CalculationReportFormat::from_str("text").expect("3d7a1e9c");

        assert_eq!(calculation_report_format, CalculationReportFormat::Text);
    }

    #[test]
    fn parses_json_report_format() {
        let calculation_report_format =
            CalculationReportFormat::from_str("json").expect("5f2c8e1a");

        assert_eq!(calculation_report_format, CalculationReportFormat::Json);
    }

    #[test]
    fn returns_unknown_report_format_error_for_invalid_value() {
        let calculation_error = CalculationReportFormat::from_str("yaml").expect_err("2b7d4e1a");

        assert_eq!(calculation_error, CalculationError::UnknownReportFormat {
            provided_format: "yaml".to_owned(),
        });
    }

    #[test]
    fn renders_json_report_when_requested() {
        let calculation_request = CalculationRequest {
            arithmetic_operation: ArithmeticOperation::Multiplication,
            left_operand: 4,
            right_operand: 6,
        };

        let rendered_report = render_calculation_report_with_format(
            &calculation_request,
            CalculationReportFormat::Json,
        )
        .expect("7c2e4a1d");

        assert_eq!(rendered_report, "{\"operation\":\"*\",\"left\":4,\"right\":6,\"result\":24}");
    }

    #[test]
    fn verifies_commutativity_property_for_addition() {
        for left_operand in -20i64..=20i64 {
            for right_operand in -20i64..=20i64 {
                let left_then_right_result = evaluate_calculation_request(&CalculationRequest {
                    arithmetic_operation: ArithmeticOperation::Addition,
                    left_operand,
                    right_operand,
                })
                .expect("7a2e5d1b");

                let right_then_left_result = evaluate_calculation_request(&CalculationRequest {
                    arithmetic_operation: ArithmeticOperation::Addition,
                    left_operand: right_operand,
                    right_operand: left_operand,
                })
                .expect("9f3c1b6e");

                assert_eq!(left_then_right_result, right_then_left_result);
            }
        }
    }
}
