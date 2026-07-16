#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OperationBudget(usize);
impl From<usize> for OperationBudget {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OperationCount(usize);
impl From<usize> for OperationCount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
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
