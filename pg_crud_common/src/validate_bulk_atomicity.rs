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

#[cfg(test)]
mod tests {
    #[test]
    fn test_failed_bulk_mutation_requires_unchanged_state() {
        let before = [1u8, 2u8];
        assert_eq!(
            crate::validate_bulk_atomicity::validate_bulk_atomicity(
                &before,
                crate::bulk_mutation_outcome::BulkMutationOutcome::Failed,
                &[1u8, 2u8]
            ),
            Ok(())
        );
        assert_eq!(
            crate::validate_bulk_atomicity::validate_bulk_atomicity(
                &before,
                crate::bulk_mutation_outcome::BulkMutationOutcome::Failed,
                &[2u8, 1u8]
            ),
            Err(crate::data_invariant_violation::DataInvariantViolation::BulkFailureChangedState)
        );
    }

    #[test]
    fn test_successful_bulk_mutation_is_rejected_before_state_comparison() {
        let before = [1u8];
        assert_eq!(
            crate::validate_bulk_atomicity::validate_bulk_atomicity(
                &before,
                crate::bulk_mutation_outcome::BulkMutationOutcome::Succeeded,
                &before
            ),
            Err(crate::data_invariant_violation::DataInvariantViolation::BulkMutationMustFail)
        );
        assert_eq!(
            crate::validate_bulk_atomicity::validate_bulk_atomicity(
                &before,
                crate::bulk_mutation_outcome::BulkMutationOutcome::Succeeded,
                &[2u8]
            ),
            Err(crate::data_invariant_violation::DataInvariantViolation::BulkMutationMustFail)
        );
    }
}
