#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct OperationBudget(usize);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct OperationCount(usize);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("operation count exceeds the deterministic budget")]
pub struct OperationBudgetExceeded {
    actual: OperationCount,
    budget: OperationBudget,
}

pub const fn validate_operation_budget(
    actual: OperationCount,
    budget: OperationBudget,
) -> Result<(), OperationBudgetExceeded> {
    if actual.0 <= budget.0 {
        Ok(())
    } else {
        Err(OperationBudgetExceeded { actual, budget })
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
        assert!(matches!(
            super::validate_operation_budget(11usize.into(), 10usize.into()),
            Err(super::OperationBudgetExceeded { .. })
        ));
    }
}
