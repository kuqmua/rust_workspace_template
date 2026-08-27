pub fn validate_migration_idempotency<SchemaSnapshot>(
    first_run: &SchemaSnapshot,
    second_run: &SchemaSnapshot,
) -> Result<(), crate::domain_types::DataInvariantViolation>
where
    SchemaSnapshot: PartialEq,
{
    if first_run == second_run {
        Ok(())
    } else {
        Err(crate::domain_types::DataInvariantViolation::MigrationSecondRunChangedSchema)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn migration_and_bulk_snapshots_must_remain_unchanged() {
        assert_eq!(
            super::validate_migration_idempotency(&[1u8, 2u8], &[1u8, 2u8]),
            Ok(())
        );
        assert_eq!(
            crate::domain_types::validate_bulk_atomicity(
                &[1u8, 2u8],
                crate::domain_types::BulkMutationOutcome::Failed,
                &[1u8, 3u8],
            ),
            Err(crate::domain_types::DataInvariantViolation::BulkFailureChangedState)
        );
    }
}
