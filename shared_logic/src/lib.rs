#![forbid(unsafe_code)]

use core::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};
use std::num::ParseIntError;

#[cfg(test)]
use proptest as _;
use thiserror::Error;
#[cfg(test)]
use trybuild as _;

const ARITHMETIC_SYMBOL_ADDITION: &str = "+";
const ARITHMETIC_SYMBOL_DIVISION: &str = "/";
const ARITHMETIC_SYMBOL_MULTIPLICATION: &str = "*";
const ARITHMETIC_SYMBOL_SUBTRACTION: &str = "-";
const CALCULATION_REPORT_FORMAT_JSON: &str = "json";
const CALCULATION_REPORT_FORMAT_TEXT: &str = "text";
const WIRE_FORMAT_DELIMITER: char = '|';

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "arithmetic operation type must be handled by the caller"]
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
#[must_use = "calculation report format type must be handled by the caller"]
pub enum CalculationReportFormat {
    Json,
    Text,
}

impl Display for CalculationReportFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let format_name = match self {
            Self::Json => CALCULATION_REPORT_FORMAT_JSON,
            Self::Text => CALCULATION_REPORT_FORMAT_TEXT,
        };
        write!(f, "{format_name}")
    }
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
#[must_use = "calculation request value must be handled by the caller"]
pub struct CalculationRequest {
    arithmetic_operation: ArithmeticOperation,
    left_operand: i64,
    right_operand: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "calculation result value must be handled by the caller"]
pub struct CalculationResult {
    value: i64,
}

impl CalculationRequest {
    #[must_use = "arithmetic operation value must be handled by the caller"]
    pub const fn arithmetic_operation(&self) -> ArithmeticOperation {
        self.arithmetic_operation
    }

    #[must_use = "left operand value must be handled by the caller"]
    pub const fn left_operand(&self) -> i64 {
        self.left_operand
    }

    #[must_use = "calculation request constructor result must be handled by the caller"]
    pub const fn new(
        arithmetic_operation: ArithmeticOperation,
        left_operand: i64,
        right_operand: i64,
    ) -> Self {
        Self {
            arithmetic_operation,
            left_operand,
            right_operand,
        }
    }

    #[must_use = "right operand value must be handled by the caller"]
    pub const fn right_operand(&self) -> i64 {
        self.right_operand
    }
}

impl CalculationResult {
    #[must_use = "calculation result constructor value must be handled by the caller"]
    pub const fn new(value: i64) -> Self {
        Self { value }
    }

    #[must_use = "calculation result value must be handled by the caller"]
    pub const fn value(&self) -> i64 {
        self.value
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CalculationError {
    #[error("division by zero is not allowed")]
    DivisionByZero,
    #[error("invalid integer value: {provided_value}")]
    InvalidIntegerValue {
        provided_value: String,
        #[source]
        source_error: ParseIntError,
    },
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
/// let request = CalculationRequest::new(ArithmeticOperation::Division, 9, 3);
///
/// let result = evaluate_calculation_request(&request).expect("e7d0a1c2");
/// assert_eq!(result.value(), 3);
/// ```
///
/// ```compile_fail
/// use shared_logic::{
///     ArithmeticOperation, CalculationRequest, evaluate_calculation_request,
/// };
///
/// let request = CalculationRequest::new(ArithmeticOperation::Addition, 1, 2);
/// let _invalid_operation = CalculationRequest::new(true, 1, 2);
/// let _ = evaluate_calculation_request(&request);
/// ```
#[must_use = "calculation evaluation result must be handled by the caller"]
pub fn evaluate_calculation_request(
    calculation_request: &CalculationRequest,
) -> Result<CalculationResult, CalculationError> {
    let value = match calculation_request.arithmetic_operation() {
        ArithmeticOperation::Addition => calculation_request
            .left_operand()
            .checked_add(calculation_request.right_operand())
            .ok_or(CalculationError::Overflow)?,
        ArithmeticOperation::Division => {
            if calculation_request.right_operand() == 0 {
                return Err(CalculationError::DivisionByZero);
            }
            calculation_request
                .left_operand()
                .checked_div(calculation_request.right_operand())
                .ok_or(CalculationError::Overflow)?
        }
        ArithmeticOperation::Multiplication => calculation_request
            .left_operand()
            .checked_mul(calculation_request.right_operand())
            .ok_or(CalculationError::Overflow)?,
        ArithmeticOperation::Subtraction => calculation_request
            .left_operand()
            .checked_sub(calculation_request.right_operand())
            .ok_or(CalculationError::Overflow)?,
    };

    Ok(CalculationResult::new(value))
}

#[must_use = "request parsing result must be handled by the caller"]
pub fn build_calculation_request_from_text_parts(
    left_operand_text: &str,
    arithmetic_operation_text: &str,
    right_operand_text: &str,
) -> Result<CalculationRequest, CalculationError> {
    let left_operand = parse_integer_operand(left_operand_text)?;
    let right_operand = parse_integer_operand(right_operand_text)?;
    let arithmetic_operation = ArithmeticOperation::from_str(arithmetic_operation_text)?;

    Ok(CalculationRequest::new(arithmetic_operation, left_operand, right_operand))
}

#[must_use = "wire-format serialization result must be handled by the caller"]
pub fn serialize_calculation_request_to_wire_format(
    calculation_request: &CalculationRequest,
) -> String {
    format!(
        "{}{}{}{}{}",
        calculation_request.left_operand(),
        WIRE_FORMAT_DELIMITER,
        calculation_request.arithmetic_operation(),
        WIRE_FORMAT_DELIMITER,
        calculation_request.right_operand()
    )
}

#[must_use = "wire-format deserialization result must be handled by the caller"]
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

#[must_use = "report rendering result must be handled by the caller"]
pub fn render_calculation_report(
    calculation_request: &CalculationRequest,
) -> Result<String, CalculationError> {
    render_calculation_report_with_format(calculation_request, CalculationReportFormat::Text)
}

#[must_use = "report rendering result must be handled by the caller"]
pub fn render_calculation_report_with_format(
    calculation_request: &CalculationRequest,
    calculation_report_format: CalculationReportFormat,
) -> Result<String, CalculationError> {
    let calculation_result = evaluate_calculation_request(calculation_request)?;
    let rendered_report = match calculation_report_format {
        CalculationReportFormat::Json => format!(
            "{{\"operation\":\"{}\",\"left\":{},\"right\":{},\"result\":{}}}",
            calculation_request.arithmetic_operation(),
            calculation_request.left_operand(),
            calculation_request.right_operand(),
            calculation_result.value()
        ),
        CalculationReportFormat::Text => format!(
            "operation={} left={} right={} result={}",
            calculation_request.arithmetic_operation(),
            calculation_request.left_operand(),
            calculation_request.right_operand(),
            calculation_result.value()
        ),
    };

    Ok(rendered_report)
}

fn build_malformed_wire_format_error(wire_format: &str) -> CalculationError {
    CalculationError::MalformedWireFormat {
        provided_wire_format: wire_format.to_owned(),
    }
}

fn parse_integer_operand(operand_text: &str) -> Result<i64, CalculationError> {
    operand_text.parse::<i64>().map_err(|parse_integer_error| {
        CalculationError::InvalidIntegerValue {
            provided_value: operand_text.to_owned(),
            source_error: parse_integer_error,
        }
    })
}

#[cfg(test)]
mod unit_tests {
    use core::str::FromStr as _;
    use std::error::Error as _;

    use test_helpers::{build_standard_division_operands, build_standard_operand_text_triplet};

    use super::{
        ArithmeticOperation, CalculationError, CalculationReportFormat, CalculationRequest,
        WIRE_FORMAT_DELIMITER, build_calculation_request_from_text_parts,
        deserialize_calculation_request_from_wire_format, evaluate_calculation_request,
        render_calculation_report_with_format, serialize_calculation_request_to_wire_format,
    };
    use crate::render_calculation_report;

    #[test]
    fn evaluates_addition() {
        let calculation_request = CalculationRequest::new(ArithmeticOperation::Addition, 8, 13);

        let calculation_result =
            evaluate_calculation_request(&calculation_request).expect("8d3b6f1a");
        assert_eq!(calculation_result.value(), 21);
    }

    #[test]
    fn returns_division_by_zero_error() {
        let (left_operand, right_operand) = build_standard_division_operands();
        let calculation_request = CalculationRequest::new(
            ArithmeticOperation::Division,
            left_operand,
            right_operand - right_operand,
        );

        let calculation_error =
            evaluate_calculation_request(&calculation_request).expect_err("1f9a2c4e");
        assert_eq!(calculation_error, CalculationError::DivisionByZero);
    }

    #[test]
    fn returns_overflow_error() {
        let calculation_request =
            CalculationRequest::new(ArithmeticOperation::Addition, i64::MAX, 1);

        let calculation_error =
            evaluate_calculation_request(&calculation_request).expect_err("5b1d7e9c");
        assert_eq!(calculation_error, CalculationError::Overflow);
    }

    #[test]
    fn returns_overflow_error_for_addition_underflow_boundary() {
        let calculation_request =
            CalculationRequest::new(ArithmeticOperation::Addition, i64::MIN, -1);

        let calculation_error =
            evaluate_calculation_request(&calculation_request).expect_err("9f2b7c1d");
        assert_eq!(calculation_error, CalculationError::Overflow);
    }

    #[test]
    fn returns_overflow_error_for_subtraction_underflow_boundary() {
        let calculation_request =
            CalculationRequest::new(ArithmeticOperation::Subtraction, i64::MIN, 1);

        let calculation_error =
            evaluate_calculation_request(&calculation_request).expect_err("3c8a1e7f");
        assert_eq!(calculation_error, CalculationError::Overflow);
    }

    #[test]
    fn returns_overflow_error_for_multiplication_boundary() {
        let calculation_request =
            CalculationRequest::new(ArithmeticOperation::Multiplication, i64::MIN, -1);

        let calculation_error =
            evaluate_calculation_request(&calculation_request).expect_err("7d1e4a9b");
        assert_eq!(calculation_error, CalculationError::Overflow);
    }

    #[test]
    fn returns_overflow_error_for_division_boundary() {
        let calculation_request =
            CalculationRequest::new(ArithmeticOperation::Division, i64::MIN, -1);

        let calculation_error =
            evaluate_calculation_request(&calculation_request).expect_err("4a9d2c7e");
        assert_eq!(calculation_error, CalculationError::Overflow);
    }

    #[test]
    fn serializes_and_deserializes_wire_format() {
        let calculation_request =
            CalculationRequest::new(ArithmeticOperation::Multiplication, -7, 5);

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
    fn unknown_operation_error_preserves_exact_input_payload_including_whitespace() {
        let provided_operation = "  +  ";
        let calculation_error =
            ArithmeticOperation::from_str(provided_operation).expect_err("9b4d1e7c");

        assert_eq!(calculation_error, CalculationError::UnknownOperation {
            provided_operation: provided_operation.to_owned(),
        });
    }

    #[test]
    fn parses_text_report_format() {
        let calculation_report_format =
            CalculationReportFormat::from_str("text").expect("3d7a1e9b");

        assert_eq!(calculation_report_format, CalculationReportFormat::Text);
    }

    #[test]
    fn parses_json_report_format() {
        let calculation_report_format =
            CalculationReportFormat::from_str("json").expect("5f2c8e1a");

        assert_eq!(calculation_report_format, CalculationReportFormat::Json);
    }

    #[test]
    fn report_format_display_and_parse_round_trip_is_stable() {
        let calculation_report_formats =
            [CalculationReportFormat::Text, CalculationReportFormat::Json];

        calculation_report_formats
            .iter()
            .try_for_each(|calculation_report_format| {
                let serialized_report_format = calculation_report_format.to_string();
                let parsed_report_format =
                    CalculationReportFormat::from_str(&serialized_report_format)?;
                assert_eq!(parsed_report_format, *calculation_report_format);
                Ok::<(), CalculationError>(())
            })
            .expect("f7a91c3d");
    }

    #[test]
    fn arithmetic_operation_display_and_parse_round_trip_is_stable() {
        let arithmetic_operations = [
            ArithmeticOperation::Addition,
            ArithmeticOperation::Subtraction,
            ArithmeticOperation::Multiplication,
            ArithmeticOperation::Division,
        ];

        arithmetic_operations
            .iter()
            .try_for_each(|arithmetic_operation| {
                let serialized_arithmetic_operation = arithmetic_operation.to_string();
                let parsed_arithmetic_operation =
                    ArithmeticOperation::from_str(&serialized_arithmetic_operation)?;
                assert_eq!(parsed_arithmetic_operation, *arithmetic_operation);
                Ok::<(), CalculationError>(())
            })
            .expect("1d7a9e4c");
    }

    #[test]
    fn returns_unknown_report_format_error_for_invalid_value() {
        let calculation_error = CalculationReportFormat::from_str("yaml").expect_err("2b7d4e1a");

        assert_eq!(calculation_error, CalculationError::UnknownReportFormat {
            provided_format: "yaml".to_owned(),
        });
    }

    #[test]
    fn unknown_report_format_error_preserves_exact_input_payload_including_whitespace() {
        let provided_report_format = " json ";
        let calculation_error =
            CalculationReportFormat::from_str(provided_report_format).expect_err("5c1a7d9e");

        assert_eq!(calculation_error, CalculationError::UnknownReportFormat {
            provided_format: provided_report_format.to_owned(),
        });
    }

    #[test]
    fn renders_json_report_when_requested() {
        let calculation_request =
            CalculationRequest::new(ArithmeticOperation::Multiplication, 4, 6);

        let rendered_report = render_calculation_report_with_format(
            &calculation_request,
            CalculationReportFormat::Json,
        )
        .expect("7c2e4a1d");

        assert_eq!(rendered_report, "{\"operation\":\"*\",\"left\":4,\"right\":6,\"result\":24}");
    }

    #[test]
    fn renders_text_report_with_stable_exact_contract() {
        let calculation_request = CalculationRequest::new(ArithmeticOperation::Subtraction, 40, 15);

        let rendered_report = render_calculation_report_with_format(
            &calculation_request,
            CalculationReportFormat::Text,
        )
        .expect("1f4a9c7e");

        assert_eq!(rendered_report, "operation=- left=40 right=15 result=25");
    }

    #[test]
    fn returns_malformed_wire_format_error_for_missing_parts() {
        let calculation_error =
            deserialize_calculation_request_from_wire_format("1|+").expect_err("7c2d8a1f");

        assert_eq!(calculation_error, CalculationError::MalformedWireFormat {
            provided_wire_format: "1|+".to_owned(),
        });
    }

    #[test]
    fn returns_invalid_integer_value_for_empty_left_operand_in_wire_format() {
        let calculation_error =
            deserialize_calculation_request_from_wire_format("|+|2").expect_err("6a3d9e1f");

        assert!(matches!(
            calculation_error,
            CalculationError::InvalidIntegerValue {
                provided_value,
                ..
            } if provided_value.is_empty()
        ));
    }

    #[test]
    fn invalid_integer_value_error_preserves_exact_input_payload_including_whitespace() {
        let calculation_error =
            build_calculation_request_from_text_parts(" 42", "+", "1").expect_err("8e4b1a7d");

        assert!(matches!(
            calculation_error,
            CalculationError::InvalidIntegerValue {
                provided_value,
                ..
            } if provided_value == " 42"
        ));
    }

    #[test]
    fn preserves_source_error_for_invalid_integer_value() {
        let calculation_error =
            build_calculation_request_from_text_parts("invalid", "+", "2").expect_err("2a6d8e1c");

        let source_error_text = calculation_error
            .source()
            .map(ToString::to_string)
            .expect("5d1a7c9e");

        assert_eq!(source_error_text, "invalid digit found in string");
    }

    #[test]
    fn preserves_source_error_for_invalid_right_integer_value_in_text_parts() {
        let calculation_error =
            build_calculation_request_from_text_parts("2", "+", "invalid").expect_err("c8a71d4f");

        let source_error_text = calculation_error
            .source()
            .map(ToString::to_string)
            .expect("7f3d8c1a");

        assert_eq!(source_error_text, "invalid digit found in string");
    }

    #[test]
    fn preserves_source_error_for_invalid_left_integer_value_in_wire_format() {
        let calculation_error =
            deserialize_calculation_request_from_wire_format("invalid|+|2").expect_err("d4b1e9a3");

        let source_error_text = calculation_error
            .source()
            .map(ToString::to_string)
            .expect("5e9a2c7d");

        assert_eq!(source_error_text, "invalid digit found in string");
    }

    #[test]
    fn evaluates_all_operations_for_deterministic_fixture_set() {
        let deterministic_fixture_set = [
            (9, 4, ArithmeticOperation::Addition, 13),
            (9, 4, ArithmeticOperation::Subtraction, 5),
            (9, 4, ArithmeticOperation::Multiplication, 36),
            (9, 3, ArithmeticOperation::Division, 3),
        ];

        deterministic_fixture_set
            .iter()
            .try_for_each(|(left_operand, right_operand, arithmetic_operation, expected_result)| {
                let calculation_request =
                    CalculationRequest::new(*arithmetic_operation, *left_operand, *right_operand);
                let calculation_result = evaluate_calculation_request(&calculation_request)?;
                assert_eq!(calculation_result.value(), *expected_result);
                Ok::<(), CalculationError>(())
            })
            .expect("3d8a1f6e");
    }

    #[test]
    fn verifies_commutativity_property_for_addition() {
        (-20i64..=20i64)
            .flat_map(|left_operand| {
                (-20i64..=20i64).map(move |right_operand| (left_operand, right_operand))
            })
            .try_for_each(|(left_operand, right_operand)| {
                let left_then_right_result =
                    evaluate_calculation_request(&CalculationRequest::new(
                        ArithmeticOperation::Addition,
                        left_operand,
                        right_operand,
                    ))?;

                let right_then_left_result =
                    evaluate_calculation_request(&CalculationRequest::new(
                        ArithmeticOperation::Addition,
                        right_operand,
                        left_operand,
                    ))?;

                assert_eq!(left_then_right_result, right_then_left_result);
                Ok::<(), CalculationError>(())
            })
            .expect("7a2e5d1b");
    }

    #[test]
    fn verifies_anti_commutativity_property_for_subtraction() {
        (-200i64..=200i64)
            .flat_map(|left_operand| {
                (-200i64..=200i64).map(move |right_operand| (left_operand, right_operand))
            })
            .try_for_each(|(left_operand, right_operand)| {
                let left_minus_right_result =
                    evaluate_calculation_request(&CalculationRequest::new(
                        ArithmeticOperation::Subtraction,
                        left_operand,
                        right_operand,
                    ))?;
                let right_minus_left_result =
                    evaluate_calculation_request(&CalculationRequest::new(
                        ArithmeticOperation::Subtraction,
                        right_operand,
                        left_operand,
                    ))?;
                let negated_right_minus_left = right_minus_left_result
                    .value()
                    .checked_neg()
                    .ok_or(CalculationError::Overflow)?;

                assert_eq!(left_minus_right_result.value(), negated_right_minus_left);
                Ok::<(), CalculationError>(())
            })
            .expect("f1a7c3e9");
    }

    #[test]
    fn rendered_reports_are_consistent_with_evaluated_result_contract() {
        let deterministic_fixture_set = [
            CalculationRequest::new(ArithmeticOperation::Addition, 12, 8),
            CalculationRequest::new(ArithmeticOperation::Subtraction, -3, 11),
            CalculationRequest::new(ArithmeticOperation::Multiplication, -7, 6),
            CalculationRequest::new(ArithmeticOperation::Division, 81, 9),
        ];

        deterministic_fixture_set
            .iter()
            .try_for_each(|calculation_request| {
                let calculation_result = evaluate_calculation_request(calculation_request)?;
                let text_report = render_calculation_report(calculation_request)?;
                let json_report = render_calculation_report_with_format(
                    calculation_request,
                    CalculationReportFormat::Json,
                )?;

                let expected_text_report = format!(
                    "operation={} left={} right={} result={}",
                    calculation_request.arithmetic_operation(),
                    calculation_request.left_operand(),
                    calculation_request.right_operand(),
                    calculation_result.value()
                );
                let expected_json_report = format!(
                    "{{\"operation\":\"{}\",\"left\":{},\"right\":{},\"result\":{}}}",
                    calculation_request.arithmetic_operation(),
                    calculation_request.left_operand(),
                    calculation_request.right_operand(),
                    calculation_result.value()
                );

                assert_eq!(text_report, expected_text_report);
                assert_eq!(json_report, expected_json_report);
                Ok::<(), CalculationError>(())
            })
            .expect("0c9e4a7d");
    }

    #[test]
    fn non_parse_calculation_errors_do_not_expose_source_chain() {
        let non_parse_calculation_errors = [
            CalculationError::DivisionByZero,
            CalculationError::Overflow,
            CalculationError::MalformedWireFormat {
                provided_wire_format: "1|+|2|extra".to_owned(),
            },
            CalculationError::UnknownOperation {
                provided_operation: "%".to_owned(),
            },
            CalculationError::UnknownReportFormat {
                provided_format: "yaml".to_owned(),
            },
        ];

        let all_errors_have_empty_source_chain = non_parse_calculation_errors
            .iter()
            .all(|calculation_error| calculation_error.source().is_none());
        assert!(all_errors_have_empty_source_chain);
    }

    #[test]
    fn serialized_wire_format_always_contains_exactly_two_delimiters() {
        let deterministic_fixture_set = [
            CalculationRequest::new(ArithmeticOperation::Addition, 10, 5),
            CalculationRequest::new(ArithmeticOperation::Subtraction, -10, 5),
            CalculationRequest::new(ArithmeticOperation::Multiplication, 0, 999),
            CalculationRequest::new(ArithmeticOperation::Division, -27, 3),
        ];

        let all_serialized_wire_formats_have_exactly_two_delimiters = deterministic_fixture_set
            .iter()
            .map(serialize_calculation_request_to_wire_format)
            .all(|serialized_wire_format| {
                let wire_format_delimiter_count = serialized_wire_format
                    .chars()
                    .filter(|symbol| *symbol == WIRE_FORMAT_DELIMITER)
                    .count();
                wire_format_delimiter_count == 2usize
            });
        assert!(all_serialized_wire_formats_have_exactly_two_delimiters);
    }

    #[test]
    fn wire_format_round_trip_preserves_i64_extreme_values_for_supported_operations() {
        let deterministic_extreme_cases = [
            (
                CalculationRequest::new(ArithmeticOperation::Addition, i64::MIN, i64::MAX),
                "-9223372036854775808|+|9223372036854775807",
            ),
            (
                CalculationRequest::new(ArithmeticOperation::Subtraction, i64::MAX, i64::MIN),
                "9223372036854775807|-|-9223372036854775808",
            ),
            (
                CalculationRequest::new(ArithmeticOperation::Multiplication, i64::MIN, 1),
                "-9223372036854775808|*|1",
            ),
            (
                CalculationRequest::new(ArithmeticOperation::Division, i64::MIN, 1),
                "-9223372036854775808|/|1",
            ),
        ];

        let round_trip_result = deterministic_extreme_cases.iter().try_for_each(
            |(calculation_request, expected_wire_format)| {
                let serialized_wire_format =
                    serialize_calculation_request_to_wire_format(calculation_request);
                assert_eq!(serialized_wire_format, *expected_wire_format);
                assert_eq!(
                    deserialize_calculation_request_from_wire_format(&serialized_wire_format),
                    Ok(*calculation_request)
                );
                Ok::<(), CalculationError>(())
            },
        );
        assert_eq!(round_trip_result, Ok(()));
    }
}
