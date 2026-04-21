use std::fs;

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
        shared_logic::CalculationRequest {
            arithmetic_operation,
            left_operand,
            right_operand,
        }
    }

    #[test]
    fn evaluates_public_api_contract() {
        let calculation_request =
            build_calculation_request(14, 6, ArithmeticOperation::Subtraction);
        let calculation_result =
            evaluate_calculation_request(&calculation_request).expect("4d8f2a1c");
        assert_eq!(calculation_result.value, 8);
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
        assert_eq!(calculation_result.value, 4);
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
}
