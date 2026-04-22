#[cfg(test)]
mod integration_tests {
    use std::path::PathBuf;

    use test_helpers::{
        build_calculation_report_golden_file_path, build_invalid_wire_format_with_extra_parts,
        build_standard_division_operands, build_standard_operand_text_triplet,
        build_standard_positional_and_wire_format_operation_cases,
    };

    #[test]
    fn golden_file_path_builder_uses_expected_workspace_layout() {
        let manifest_directory = "/workspace/shared_logic";
        let golden_file_path = build_calculation_report_golden_file_path(manifest_directory);

        let expected_path = PathBuf::from("/workspace/shared_logic")
            .join("tests")
            .join("golden")
            .join("calculation_report.txt");

        assert_eq!(golden_file_path, expected_path);
    }

    #[test]
    fn standard_division_operands_fixture_is_stable() {
        let (left_operand, right_operand) = build_standard_division_operands();
        assert_eq!(left_operand, 27);
        assert_eq!(right_operand, 3);
    }

    #[test]
    fn standard_operand_text_triplet_fixture_is_stable() {
        let (left_operand_text, arithmetic_operation_text, right_operand_text) =
            build_standard_operand_text_triplet();

        assert_eq!(left_operand_text, "27");
        assert_eq!(arithmetic_operation_text, "/");
        assert_eq!(right_operand_text, "3");
    }

    #[test]
    fn invalid_wire_format_fixture_is_stable() {
        let invalid_wire_format = build_invalid_wire_format_with_extra_parts();
        assert_eq!(invalid_wire_format, "1|+|2|extra");
    }

    #[test]
    fn standard_positional_and_wire_format_operation_cases_fixture_is_stable() {
        let standard_cases = build_standard_positional_and_wire_format_operation_cases();

        assert_eq!(standard_cases, [
            ("11", "+", "7", "11|+|7"),
            ("11", "-", "7", "11|-|7"),
            ("11", "*", "7", "11|*|7"),
            ("21", "/", "7", "21|/|7"),
        ]);
    }
}
