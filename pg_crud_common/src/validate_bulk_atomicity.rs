pub fn validate_bulk_atomicity<StateSnapshot>(
    before: &StateSnapshot,
    outcome: crate::domain_types::BulkMutationOutcome,
    after: &StateSnapshot,
) -> Result<(), crate::domain_types::DataInvariantViolation>
where
    StateSnapshot: PartialEq,
{
    if outcome != crate::domain_types::BulkMutationOutcome::Failed {
        return Err(crate::domain_types::DataInvariantViolation::BulkMutationMustFail);
    }
    if before == after {
        Ok(())
    } else {
        Err(crate::domain_types::DataInvariantViolation::BulkFailureChangedState)
    }
}
