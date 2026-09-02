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
        assert!(
            crate::validate_operation_budget::validate_operation_budget(
                11usize.into(),
                10usize.into()
            )
            .is_err()
        );
    }
}
