pub const fn validate_operation_budget(
    operation_count: crate::operation_count::OperationCount,
    operation_budget: crate::operation_budget::OperationBudget,
) -> Result<(), crate::operation_budget_exceeded::OperationBudgetExceeded> {
    if operation_count.get() <= operation_budget.get() {
        Ok(())
    } else {
        Err(
            crate::operation_budget_exceeded::OperationBudgetExceeded::Exceeded {
                actual: operation_count,
                budget: operation_budget,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_operation_count_must_not_exceed_budget() {
        assert_eq!(
            crate::validate_operation_budget::validate_operation_budget(
                10usize.into(),
                10usize.into()
            ),
            Ok(())
        );
        assert!([0usize, 1usize, 9usize].into_iter().all(|operation_count| {
            crate::validate_operation_budget::validate_operation_budget(
                crate::operation_count::OperationCount::from(operation_count),
                crate::operation_budget::OperationBudget::from(10usize),
            ) == Ok(())
        }));
        assert_eq!(
            crate::validate_operation_budget::validate_operation_budget(
                crate::operation_count::OperationCount::from(11usize),
                crate::operation_budget::OperationBudget::from(10usize),
            ),
            Err(
                crate::operation_budget_exceeded::OperationBudgetExceeded::Exceeded {
                    actual: crate::operation_count::OperationCount::from(11usize),
                    budget: crate::operation_budget::OperationBudget::from(10usize),
                }
            )
        );
    }
    #[test]
    fn test_zero_budget_and_exceeded_payload() {
        assert_eq!(
            crate::validate_operation_budget::validate_operation_budget(
                0usize.into(),
                0usize.into()
            ),
            Ok(())
        );
        assert_eq!(
            crate::validate_operation_budget::validate_operation_budget(
                1usize.into(),
                0usize.into()
            ),
            Err(
                crate::operation_budget_exceeded::OperationBudgetExceeded::Exceeded {
                    actual: 1usize.into(),
                    budget: 0usize.into()
                }
            )
        );
    }
}
