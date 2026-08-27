pub const fn validate_operation_budget(
    actual: crate::domain_types::OperationCount,
    budget: crate::domain_types::OperationBudget,
) -> Result<(), crate::domain_types::OperationBudgetExceeded> {
    if actual.get() <= budget.get() {
        Ok(())
    } else {
        Err(crate::domain_types::OperationBudgetExceeded::new(
            actual, budget,
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn operation_count_must_not_exceed_budget() {
        assert_eq!(
            super::validate_operation_budget(10usize.into(), 10usize.into()),
            Ok(())
        );
        assert!(super::validate_operation_budget(11usize.into(), 10usize.into()).is_err());
    }
}
