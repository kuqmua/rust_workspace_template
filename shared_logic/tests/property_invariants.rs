#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    use shared_logic::{
        ArithmeticOperation, CalculationRequest, deserialize_calculation_request_from_wire_format,
        evaluate_calculation_request, serialize_calculation_request_to_wire_format,
    };
    use test_helpers as _;
    use thiserror as _;
    use trybuild as _;

    proptest! {
        #[test]
        fn wire_format_round_trip_preserves_request(
            left_operand in -1_000_000i64..=1_000_000i64,
            right_operand in -1_000_000i64..=1_000_000i64,
            arithmetic_operation in prop_oneof![
                Just(ArithmeticOperation::Addition),
                Just(ArithmeticOperation::Subtraction),
                Just(ArithmeticOperation::Multiplication),
                Just(ArithmeticOperation::Division),
            ],
        ) {
            let sanitized_right_operand =
                if arithmetic_operation == ArithmeticOperation::Division && right_operand == 0 {
                    1
                } else {
                    right_operand
                };

            let calculation_request =
                CalculationRequest::new(arithmetic_operation, left_operand, sanitized_right_operand);
            let wire_format = serialize_calculation_request_to_wire_format(&calculation_request);
            let deserialized_calculation_request =
                deserialize_calculation_request_from_wire_format(&wire_format).expect("1a2b3c4d");

            prop_assert_eq!(deserialized_calculation_request, calculation_request);
        }

        #[test]
        fn addition_commutativity_holds_for_safe_operand_range(
            left_operand in -1_000_000_000i64..=1_000_000_000i64,
            right_operand in -1_000_000_000i64..=1_000_000_000i64,
        ) {
            let left_then_right_request =
                CalculationRequest::new(ArithmeticOperation::Addition, left_operand, right_operand);
            let right_then_left_request =
                CalculationRequest::new(ArithmeticOperation::Addition, right_operand, left_operand);

            let left_then_right_result =
                evaluate_calculation_request(&left_then_right_request).expect("5e6f7a8b");
            let right_then_left_result =
                evaluate_calculation_request(&right_then_left_request).expect("9c0d1e2f");

            prop_assert_eq!(left_then_right_result, right_then_left_result);
        }

        #[test]
        fn subtraction_is_inverse_of_addition_for_safe_operand_range(
            left_operand in -1_000_000_000i64..=1_000_000_000i64,
            right_operand in -1_000_000_000i64..=1_000_000_000i64,
        ) {
            let addition_request =
                CalculationRequest::new(ArithmeticOperation::Addition, left_operand, right_operand);
            let addition_result = evaluate_calculation_request(&addition_request).expect("6d8a1f2c");
            let subtraction_request = CalculationRequest::new(
                ArithmeticOperation::Subtraction,
                addition_result.value(),
                right_operand,
            );
            let subtraction_result =
                evaluate_calculation_request(&subtraction_request).expect("3e7c1a9d");

            prop_assert_eq!(subtraction_result.value(), left_operand);
        }
    }
}
