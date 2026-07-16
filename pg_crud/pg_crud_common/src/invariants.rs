#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BulkMutationOutcome {
    Failed,
    Succeeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PaginationTotal(usize);
impl From<usize> for PaginationTotal {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum DataInvariantViolation {
    #[error("{}", str_constants::BULK_FAILURE_CHANGED_STATE)]
    BulkFailureChangedState,
    #[error("{}", str_constants::BULK_MUTATION_MUST_FAIL)]
    BulkMutationMustFail,
    #[error("{}", str_constants::MIGRATION_SECOND_RUN_CHANGED_SCHEMA)]
    MigrationSecondRunChangedSchema,
    #[error("{}", str_constants::PAGINATION_ITEMS_OVERLAP)]
    PaginationItemsOverlap,
    #[error("{}", str_constants::PAGINATION_TOTAL_CHANGED)]
    PaginationTotalChanged,
}

pub fn validate_migration_idempotency<SchemaSnapshot>(
    first_run: &SchemaSnapshot,
    second_run: &SchemaSnapshot,
) -> Result<(), DataInvariantViolation>
where
    SchemaSnapshot: PartialEq,
{
    if first_run == second_run {
        Ok(())
    } else {
        Err(DataInvariantViolation::MigrationSecondRunChangedSchema)
    }
}

pub fn validate_bulk_atomicity<StateSnapshot>(
    before: &StateSnapshot,
    outcome: BulkMutationOutcome,
    after: &StateSnapshot,
) -> Result<(), DataInvariantViolation>
where
    StateSnapshot: PartialEq,
{
    if outcome != BulkMutationOutcome::Failed {
        return Err(DataInvariantViolation::BulkMutationMustFail);
    }
    if before == after {
        Ok(())
    } else {
        Err(DataInvariantViolation::BulkFailureChangedState)
    }
}

pub fn validate_pagination_invariants<Identifier>(
    first_page: &[Identifier],
    first_total: PaginationTotal,
    second_page: &[Identifier],
    second_total: PaginationTotal,
) -> Result<(), DataInvariantViolation>
where
    Identifier: PartialEq,
{
    if first_total != second_total {
        return Err(DataInvariantViolation::PaginationTotalChanged);
    }
    if first_page
        .iter()
        .any(|identifier| second_page.contains(identifier))
    {
        Err(DataInvariantViolation::PaginationItemsOverlap)
    } else {
        Ok(())
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
            super::validate_bulk_atomicity(
                &[1u8, 2u8],
                super::BulkMutationOutcome::Failed,
                &[1u8, 3u8],
            ),
            Err(super::DataInvariantViolation::BulkFailureChangedState)
        );
    }

    #[test]
    fn pagination_rejects_overlap_and_total_changes() {
        assert_eq!(
            super::validate_pagination_invariants(
                &[1u8, 2u8],
                4usize.into(),
                &[2u8, 3u8],
                4usize.into(),
            ),
            Err(super::DataInvariantViolation::PaginationItemsOverlap)
        );
        assert_eq!(
            super::validate_pagination_invariants(&[1u8], 2usize.into(), &[2u8], 3usize.into(),),
            Err(super::DataInvariantViolation::PaginationTotalChanged)
        );
    }
}
