pub fn validate_bulk_atomicity<StateSnapshot>(
    before: &StateSnapshot,
    bulk_mutation_outcome: crate::bulk_mutation_outcome::BulkMutationOutcome,
    after: &StateSnapshot,
) -> Result<(), crate::data_invariant_violation::DataInvariantViolation>
where
    StateSnapshot: PartialEq,
{
    if bulk_mutation_outcome != crate::bulk_mutation_outcome::BulkMutationOutcome::Failed {
        return Err(crate::data_invariant_violation::DataInvariantViolation::BulkMutationMustFail);
    }
    if before == after {
        Ok(())
    } else {
        Err(crate::data_invariant_violation::DataInvariantViolation::BulkFailureChangedState)
    }
}
