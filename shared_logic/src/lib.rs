use core::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[cfg(test)]
use test_helpers as _;
use thiserror::Error;

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
            Self::Addition => "+",
            Self::Division => "/",
            Self::Multiplication => "*",
            Self::Subtraction => "-",
        };
        write!(f, "{symbol}")
    }
}

impl FromStr for ArithmeticOperation {
    type Err = CalculationError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source {
            "+" => Ok(Self::Addition),
            "/" => Ok(Self::Division),
            "*" => Ok(Self::Multiplication),
            "-" => Ok(Self::Subtraction),
            _ => Err(CalculationError::UnknownOperation {
                provided_operation: source.to_owned(),
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

#[must_use]
pub fn serialize_calculation_request_to_wire_format(
    calculation_request: &CalculationRequest,
) -> String {
    format!(
        "{}|{}|{}",
        calculation_request.left_operand,
        calculation_request.arithmetic_operation,
        calculation_request.right_operand
    )
}

pub fn deserialize_calculation_request_from_wire_format(
    wire_format: &str,
) -> Result<CalculationRequest, CalculationError> {
    let mut parts = wire_format.split('|');
    let left_operand_part = parts
        .next()
        .ok_or_else(|| CalculationError::MalformedWireFormat {
            provided_wire_format: wire_format.to_owned(),
        })?;
    let operation_part = parts
        .next()
        .ok_or_else(|| CalculationError::MalformedWireFormat {
            provided_wire_format: wire_format.to_owned(),
        })?;
    let right_operand_part = parts
        .next()
        .ok_or_else(|| CalculationError::MalformedWireFormat {
            provided_wire_format: wire_format.to_owned(),
        })?;

    if parts.next().is_some() {
        return Err(CalculationError::MalformedWireFormat {
            provided_wire_format: wire_format.to_owned(),
        });
    }

    let left_operand = left_operand_part
        .parse::<i64>()
        .map_err(|_parse_integer_error| CalculationError::InvalidIntegerValue {
            provided_value: left_operand_part.to_owned(),
        })?;
    let right_operand = right_operand_part
        .parse::<i64>()
        .map_err(|_parse_integer_error| CalculationError::InvalidIntegerValue {
            provided_value: right_operand_part.to_owned(),
        })?;
    let arithmetic_operation = ArithmeticOperation::from_str(operation_part)?;

    Ok(CalculationRequest {
        arithmetic_operation,
        left_operand,
        right_operand,
    })
}

pub fn render_calculation_report(
    calculation_request: &CalculationRequest,
) -> Result<String, CalculationError> {
    let calculation_result = evaluate_calculation_request(calculation_request)?;
    Ok(format!(
        "operation={} left={} right={} result={}",
        calculation_request.arithmetic_operation,
        calculation_request.left_operand,
        calculation_request.right_operand,
        calculation_result.value
    ))
}

#[cfg(test)]
mod unit_tests {
    use super::{
        ArithmeticOperation, CalculationError, CalculationRequest,
        deserialize_calculation_request_from_wire_format, evaluate_calculation_request,
        serialize_calculation_request_to_wire_format,
    };

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
        let calculation_request = CalculationRequest {
            arithmetic_operation: ArithmeticOperation::Division,
            left_operand: 21,
            right_operand: 0,
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
