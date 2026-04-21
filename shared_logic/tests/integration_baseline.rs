use std::fs;

use proptest as _;
use shared_logic::{
    ArithmeticOperation, CalculationReportFormat, build_calculation_request_from_text_parts,
    deserialize_calculation_request_from_wire_format, evaluate_calculation_request,
    render_calculation_report, render_calculation_report_with_format,
    serialize_calculation_request_to_wire_format,
};
use test_helpers::{
    build_calculation_report_golden_file_path, build_invalid_wire_format_with_extra_parts,
    build_standard_division_operands,
};
use thiserror as _;
use trybuild as _;

#[cfg(test)]
mod integration_tests {
    use super::{
        ArithmeticOperation, CalculationReportFormat, build_calculation_report_golden_file_path,
        build_calculation_request_from_text_parts, build_invalid_wire_format_with_extra_parts,
        build_standard_division_operands, deserialize_calculation_request_from_wire_format,
        evaluate_calculation_request, fs, render_calculation_report,
        render_calculation_report_with_format, serialize_calculation_request_to_wire_format,
    };

    const fn build_calculation_request(
        left_operand: i64,
        right_operand: i64,
        arithmetic_operation: ArithmeticOperation,
    ) -> shared_logic::CalculationRequest {
        shared_logic::CalculationRequest::new(arithmetic_operation, left_operand, right_operand)
    }

    #[test]
    fn evaluates_public_api_contract() {
        let calculation_request =
            build_calculation_request(14, 6, ArithmeticOperation::Subtraction);
        let calculation_result =
            evaluate_calculation_request(&calculation_request).expect("4d8f2a1c");
        assert_eq!(calculation_result.value(), 8);
    }

    #[test]
    fn request_getters_preserve_constructor_values() {
        let calculation_request = build_calculation_request(17, 9, ArithmeticOperation::Addition);

        assert_eq!(calculation_request.arithmetic_operation(), ArithmeticOperation::Addition);
        assert_eq!(calculation_request.left_operand(), 17);
        assert_eq!(calculation_request.right_operand(), 9);
    }

    #[test]
    fn evaluates_deterministically_for_same_input() {
        let (left_operand, right_operand) = build_standard_division_operands();
        let calculation_request =
            build_calculation_request(left_operand, right_operand, ArithmeticOperation::Division);

        let first_result = evaluate_calculation_request(&calculation_request).expect("3b7e1d9a");
        let second_result = evaluate_calculation_request(&calculation_request).expect("8c1f6a2d");

        assert_eq!(first_result, second_result);
    }

    #[test]
    fn matches_golden_report_snapshot() {
        let calculation_request =
            build_calculation_request(10, 4, ArithmeticOperation::Multiplication);
        let rendered_report = render_calculation_report(&calculation_request).expect("7d2a9c1e");

        let golden_file_path =
            build_calculation_report_golden_file_path(env!("CARGO_MANIFEST_DIR"));

        let expected_report = fs::read_to_string(golden_file_path).expect("4a1e8d7c");
        assert_eq!(rendered_report, expected_report.trim_end());
    }

    #[test]
    fn preserves_value_on_round_trip() {
        let calculation_request =
            build_calculation_request(-12, 9, ArithmeticOperation::Multiplication);
        let wire_format = serialize_calculation_request_to_wire_format(&calculation_request);
        let deserialized_calculation_request =
            deserialize_calculation_request_from_wire_format(&wire_format).expect("6e2b9c4d");
        assert_eq!(deserialized_calculation_request, calculation_request);
    }

    #[test]
    fn preserves_value_on_round_trip_for_each_arithmetic_operation() {
        let deterministic_round_trip_cases = [
            build_calculation_request(11, 7, ArithmeticOperation::Addition),
            build_calculation_request(11, 7, ArithmeticOperation::Subtraction),
            build_calculation_request(11, 7, ArithmeticOperation::Multiplication),
            build_calculation_request(21, 7, ArithmeticOperation::Division),
        ];

        deterministic_round_trip_cases
            .iter()
            .try_for_each(|calculation_request| {
                let wire_format = serialize_calculation_request_to_wire_format(calculation_request);
                let deserialized_calculation_request =
                    deserialize_calculation_request_from_wire_format(&wire_format)?;
                assert_eq!(&deserialized_calculation_request, calculation_request);
                Ok::<(), shared_logic::CalculationError>(())
            })
            .expect("7e1d3a9c");
    }

    #[test]
    fn renders_report_deterministically_for_same_input() {
        let calculation_request = build_calculation_request(5, 8, ArithmeticOperation::Addition);

        let first_report = render_calculation_report(&calculation_request).expect("1a9d6f3b");
        let second_report = render_calculation_report(&calculation_request).expect("2e4c8b1f");

        assert_eq!(first_report, second_report);
    }

    #[test]
    fn renders_json_report_for_machine_consumers() {
        let calculation_request = build_calculation_request(5, 8, ArithmeticOperation::Addition);

        let json_report = render_calculation_report_with_format(
            &calculation_request,
            CalculationReportFormat::Json,
        )
        .expect("8d1f4e2a");

        assert_eq!(json_report, "{\"operation\":\"+\",\"left\":5,\"right\":8,\"result\":13}");
    }

    #[test]
    fn parses_text_input_for_client_usage() {
        let calculation_request =
            build_calculation_request_from_text_parts("11", "-", "7").expect("9a3e1c7f");

        let calculation_result =
            evaluate_calculation_request(&calculation_request).expect("1b8d6f2a");
        assert_eq!(calculation_result.value(), 4);
    }

    #[test]
    fn rejects_malformed_wire_format_with_extra_parts() {
        let calculation_error = deserialize_calculation_request_from_wire_format(
            build_invalid_wire_format_with_extra_parts(),
        )
        .expect_err("3f9d1a7c");

        assert_eq!(calculation_error, shared_logic::CalculationError::MalformedWireFormat {
            provided_wire_format: build_invalid_wire_format_with_extra_parts().to_owned(),
        });
    }

    #[test]
    fn report_format_parsing_has_stable_contract() {
        let text_report_format = "text".parse::<CalculationReportFormat>().expect("2d9c7a1e");
        let json_report_format = "json".parse::<CalculationReportFormat>().expect("6a1e4d9c");
        let unknown_report_format_error = "yaml"
            .parse::<CalculationReportFormat>()
            .expect_err("8f2c1a7d");

        assert_eq!(text_report_format, CalculationReportFormat::Text);
        assert_eq!(json_report_format, CalculationReportFormat::Json);
        assert_eq!(
            unknown_report_format_error,
            shared_logic::CalculationError::UnknownReportFormat {
                provided_format: "yaml".to_owned(),
            }
        );
    }

    #[test]
    fn default_report_renderer_matches_explicit_text_format() {
        let calculation_request = build_calculation_request(15, 4, ArithmeticOperation::Division);
        let default_rendered_report =
            render_calculation_report(&calculation_request).expect("4c2d9a1f");
        let explicitly_rendered_text_report = render_calculation_report_with_format(
            &calculation_request,
            CalculationReportFormat::Text,
        )
        .expect("1a7d6c3e");

        assert_eq!(default_rendered_report, explicitly_rendered_text_report);
    }

    #[test]
    fn returns_overflow_error_for_addition_contract() {
        let calculation_request =
            build_calculation_request(i64::MAX, 1, ArithmeticOperation::Addition);

        let calculation_error =
            evaluate_calculation_request(&calculation_request).expect_err("1f9d4a7c");
        assert_eq!(calculation_error, shared_logic::CalculationError::Overflow);
    }

    #[test]
    fn returns_overflow_error_for_division_contract() {
        let calculation_request =
            build_calculation_request(i64::MIN, -1, ArithmeticOperation::Division);

        let calculation_error =
            evaluate_calculation_request(&calculation_request).expect_err("5d2b8e1a");
        assert_eq!(calculation_error, shared_logic::CalculationError::Overflow);
    }

    #[test]
    fn rejects_unknown_operation_with_stable_error_payload() {
        let calculation_error =
            build_calculation_request_from_text_parts("10", "%", "3").expect_err("3d7a1c9e");

        assert_eq!(calculation_error, shared_logic::CalculationError::UnknownOperation {
            provided_operation: "%".to_owned(),
        });
    }

    #[test]
    fn rejects_invalid_left_operand_with_stable_error_payload() {
        let calculation_error =
            build_calculation_request_from_text_parts("invalid", "+", "3").expect_err("2c7e1d9a");

        assert_eq!(calculation_error, shared_logic::CalculationError::InvalidIntegerValue {
            provided_value: "invalid".to_owned(),
        });
    }

    #[test]
    fn rejects_invalid_right_operand_from_wire_format_with_stable_error_payload() {
        let calculation_error =
            deserialize_calculation_request_from_wire_format("10|+|invalid").expect_err("7f1d3a8c");

        assert_eq!(calculation_error, shared_logic::CalculationError::InvalidIntegerValue {
            provided_value: "invalid".to_owned(),
        });
    }

    #[test]
    fn arithmetic_operation_display_symbols_are_stable() {
        assert_eq!(ArithmeticOperation::Addition.to_string(), "+");
        assert_eq!(ArithmeticOperation::Subtraction.to_string(), "-");
        assert_eq!(ArithmeticOperation::Multiplication.to_string(), "*");
        assert_eq!(ArithmeticOperation::Division.to_string(), "/");
    }

    #[test]
    fn division_by_zero_error_message_contract_is_stable() {
        let calculation_request = build_calculation_request(10, 0, ArithmeticOperation::Division);
        let calculation_error =
            evaluate_calculation_request(&calculation_request).expect_err("2a1e8c7d");

        assert_eq!(calculation_error.to_string(), "division by zero is not allowed");
    }

    #[test]
    fn unknown_operation_error_message_contract_is_stable() {
        let calculation_error =
            build_calculation_request_from_text_parts("10", "%", "3").expect_err("4d8a1e7c");

        assert_eq!(calculation_error.to_string(), "unknown arithmetic operation: %");
    }

    #[test]
    fn malformed_wire_format_error_message_contract_for_missing_right_part_is_stable() {
        let calculation_error =
            deserialize_calculation_request_from_wire_format("10|+").expect_err("9f2b1c7d");

        assert_eq!(
            calculation_error.to_string(),
            "wire format must contain exactly 3 parts separated by '|': 10|+"
        );
    }
}
