#[cfg(test)]
mod compile_fail_tests {
    use proptest as _;
    use shared_logic as _;
    use test_helpers as _;
    use thiserror as _;

    #[test]
    fn arithmetic_operation_type_contract_rejects_boolean_argument() {
        let test_cases = trybuild::TestCases::new();
        test_cases.compile_fail(
            "tests/ui/arithmetic_operation_type_contract_rejects_boolean_argument.rs",
        );
    }
}
