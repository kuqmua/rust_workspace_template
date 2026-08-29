pub const fn validate_operation_budget(
    actual: crate::operation_count::OperationCount,
    budget: crate::operation_budget::OperationBudget,
) -> Result<(), crate::operation_budget_exceeded::OperationBudgetExceeded> {
    if actual.get() <= budget.get() {
        Ok(())
    } else {
        Err(crate::operation_budget_exceeded::OperationBudgetExceeded::new(actual, budget))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn operation_count_must_not_exceed_budget() {
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
