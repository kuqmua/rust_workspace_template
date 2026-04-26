#![forbid(unsafe_code)]

use core::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};
use std::num::ParseIntError;

use thiserror::Error;

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
