pub fn validate_migration_idempotency<SchemaSnapshot>(
    first_run: &SchemaSnapshot,
    second_run: &SchemaSnapshot,
) -> Result<(), crate::data_invariant_violation::DataInvariantViolation>
where
    SchemaSnapshot: PartialEq,
{
    if first_run == second_run {
        Ok(())
    } else {
        Err(crate::data_invariant_violation::DataInvariantViolation::MigrationSecondRunChangedSchema)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn migration_and_bulk_snapshots_must_remain_unchanged() {
        assert_eq!(
            crate::validate_migration_idempotency::validate_migration_idempotency(
                &[1u8, 2u8],
                &[1u8, 2u8]
            ),
            Ok(())
        );
        assert_eq!(
            crate::validate_bulk_atomicity::validate_bulk_atomicity(
                &[1u8, 2u8],
                crate::bulk_mutation_outcome::BulkMutationOutcome::Failed,
                &[1u8, 3u8],
            ),
            Err(crate::data_invariant_violation::DataInvariantViolation::BulkFailureChangedState)
        );
    }
}
