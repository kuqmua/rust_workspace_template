const MAXIMUM_SCOPED_FOREIGN_KEY_COLUMNS: usize = 16usize;
const MINIMUM_SCOPED_FOREIGN_KEY_COLUMNS: usize = 2usize;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgScopedForeignKeyOnDelete {
    Cascade,
    Restrict,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum PgScopedForeignKeyError {
    #[error("{}", constants_str::PG_SCOPED_FOREIGN_KEY_COLUMN_COUNT_MISMATCH)]
    ColumnCountMismatch,
    #[error("{}", constants_str::PG_SCOPED_FOREIGN_KEY_DUPLICATE_COLUMN)]
    DuplicateColumn,
    #[error("{}", constants_str::PG_SCOPED_FOREIGN_KEY_INVALID_COLUMN_COUNT)]
    InvalidColumnCount,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::FromInner,
)]
pub struct PgSqlIdentifiers(Vec<crate::domain_types::SqlIdentifier>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub struct PgScopedForeignKey {
    local_columns: PgSqlIdentifiers,
    referenced_columns: PgSqlIdentifiers,
    referenced_table: crate::domain_types::SqlQualifiedIdentifier,
    on_delete: PgScopedForeignKeyOnDelete,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
enum PgDuplicateIdentifierPresence {
    Absent,
    Present,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
struct PgScopedForeignKeyClauseText(String);

impl TryFrom<String> for PgScopedForeignKeyClauseText {
    type Error = crate::domain_types::PgCrudStringWrapperTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::domain_types::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            Err(
                crate::domain_types::PgCrudStringWrapperTryFromStringError::TooLong {
                    len: value.len(),
                    max: crate::domain_types::PG_CRUD_STRING_WRAPPER_MAX_LEN,
                },
            )
        } else {
            Ok(Self(value))
        }
    }
}

impl PgScopedForeignKey {
    pub fn new(
        local_columns: PgSqlIdentifiers,
        referenced_table: crate::domain_types::SqlQualifiedIdentifier,
        referenced_columns: PgSqlIdentifiers,
        on_delete: PgScopedForeignKeyOnDelete,
    ) -> Result<Self, PgScopedForeignKeyError> {
        if local_columns.0.len() != referenced_columns.0.len() {
            return Err(PgScopedForeignKeyError::ColumnCountMismatch);
        }
        if !(MINIMUM_SCOPED_FOREIGN_KEY_COLUMNS..=MAXIMUM_SCOPED_FOREIGN_KEY_COLUMNS)
            .contains(&local_columns.0.len())
        {
            return Err(PgScopedForeignKeyError::InvalidColumnCount);
        }
        if contains_duplicate_identifier(local_columns.0.as_slice())
            == PgDuplicateIdentifierPresence::Present
            || contains_duplicate_identifier(referenced_columns.0.as_slice())
                == PgDuplicateIdentifierPresence::Present
        {
            return Err(PgScopedForeignKeyError::DuplicateColumn);
        }
        Ok(Self {
            local_columns,
            referenced_columns,
            referenced_table,
            on_delete,
        })
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    newtype::FromInner,
)]
pub struct PgCounterValue(u64);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgCounterReconciliation {
    ActualAhead(PgCounterValue),
    InSync,
    TrackedAhead(PgCounterValue),
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd,
)]
pub struct PgOperationalLimit(u64);

impl TryFrom<u64> for PgOperationalLimit {
    type Error = PgOperationalLimitError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == constants_u64::ZERO {
            Err(PgOperationalLimitError::ZeroLimit)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgOperationalLimitUpdateAuthority {
    MigrationDefault,
    Operator,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum PgOperationalLimitError {
    #[error("{}", constants_str::PG_OPERATIONAL_LIMIT_BELOW_CURRENT_USAGE)]
    BelowCurrentUsage,
    #[error("{}", constants_str::PG_OPERATIONAL_LIMIT_MUST_BE_GREATER_THAN_ZERO)]
    ZeroLimit,
}

pub fn build_pg_scoped_foreign_key_clause(
    foreign_key: &PgScopedForeignKey,
) -> Result<
    crate::domain_types::QueryPartFragment,
    crate::domain_types::PgCrudStringWrapperTryFromStringError,
> {
    let mut clause =
        PgScopedForeignKeyClauseText::try_from(String::from(constants_str::FOREIGN_KEY_OPENING))?;
    push_identifier_list(&mut clause, foreign_key.local_columns.0.as_slice());
    clause.0.push_str(constants_str::REFERENCES);
    clause
        .0
        .push_str(foreign_key.referenced_table.to_string().as_str());
    clause.0.push('(');
    push_identifier_list(&mut clause, foreign_key.referenced_columns.0.as_slice());
    clause.0.push(')');
    clause.0.push_str(match foreign_key.on_delete {
        PgScopedForeignKeyOnDelete::Cascade => constants_str::ON_DELETE_CASCADE,
        PgScopedForeignKeyOnDelete::Restrict => constants_str::ON_DELETE_RESTRICT,
    });
    crate::domain_types::QueryPartFragment::try_from(clause.0)
}

fn contains_duplicate_identifier(
    columns: &[crate::domain_types::SqlIdentifier],
) -> PgDuplicateIdentifierPresence {
    if columns
        .iter()
        .enumerate()
        .any(|(index, column)| columns.iter().take(index).any(|seen| seen == column))
    {
        PgDuplicateIdentifierPresence::Present
    } else {
        PgDuplicateIdentifierPresence::Absent
    }
}

fn push_identifier_list(
    output: &mut PgScopedForeignKeyClauseText,
    columns: &[crate::domain_types::SqlIdentifier],
) {
    columns.iter().enumerate().for_each(|(index, column)| {
        if index != constants_usize::ZERO {
            output.0.push_str(constants_str::TEXT_ALT_6);
        }
        output.0.push_str(column.as_ref());
    });
}

#[must_use]
pub fn reconcile_pg_counter(
    tracked: PgCounterValue,
    actual: PgCounterValue,
) -> PgCounterReconciliation {
    match actual.0.cmp(&tracked.0) {
        std::cmp::Ordering::Greater => PgCounterReconciliation::ActualAhead(PgCounterValue::from(
            actual.0.saturating_sub(tracked.0),
        )),
        std::cmp::Ordering::Less => PgCounterReconciliation::TrackedAhead(PgCounterValue::from(
            tracked.0.saturating_sub(actual.0),
        )),
        std::cmp::Ordering::Equal => PgCounterReconciliation::InSync,
    }
}

pub fn resolve_pg_operational_limit_update(
    current: PgOperationalLimit,
    requested: PgOperationalLimit,
    current_usage: PgCounterValue,
    authority: PgOperationalLimitUpdateAuthority,
) -> Result<PgOperationalLimit, PgOperationalLimitError> {
    match authority {
        PgOperationalLimitUpdateAuthority::MigrationDefault => Ok(current.max(requested)),
        PgOperationalLimitUpdateAuthority::Operator if requested.0 < current_usage.0 => {
            Err(PgOperationalLimitError::BelowCurrentUsage)
        }
        PgOperationalLimitUpdateAuthority::Operator => Ok(requested),
    }
}

#[cfg(test)]
mod tests {
    fn identifier(value: &str) -> crate::domain_types::SqlIdentifier {
        crate::domain_types::SqlIdentifier::try_from(value.to_owned())
            .expect("2ec15e48 identifier invariant must hold")
    }

    fn limit(value: u64) -> super::PgOperationalLimit {
        super::PgOperationalLimit::try_from(value).expect("2710e8b4 limit invariant must hold")
    }

    #[test]
    fn counter_reconciliation_reports_direction_and_distance() {
        assert_eq!(
            super::reconcile_pg_counter(7u64.into(), 10u64.into()),
            super::PgCounterReconciliation::ActualAhead(3u64.into())
        );
        assert_eq!(
            super::reconcile_pg_counter(12u64.into(), 10u64.into()),
            super::PgCounterReconciliation::TrackedAhead(2u64.into())
        );
        assert_eq!(
            super::reconcile_pg_counter(10u64.into(), 10u64.into()),
            super::PgCounterReconciliation::InSync
        );
    }

    #[test]
    fn migration_defaults_only_raise_limits_and_operator_cannot_cross_usage() {
        assert_eq!(
            super::resolve_pg_operational_limit_update(
                limit(100u64),
                limit(50u64),
                80u64.into(),
                super::PgOperationalLimitUpdateAuthority::MigrationDefault,
            ),
            Ok(limit(100u64))
        );
        assert_eq!(
            super::resolve_pg_operational_limit_update(
                limit(100u64),
                limit(50u64),
                80u64.into(),
                super::PgOperationalLimitUpdateAuthority::Operator,
            ),
            Err(super::PgOperationalLimitError::BelowCurrentUsage)
        );
    }

    #[test]
    fn scoped_foreign_key_uses_validated_composite_columns() {
        let foreign_key = super::PgScopedForeignKey::new(
            vec![
                identifier(constants_str::PG_TEST_FEATURE_ID),
                identifier(constants_str::PG_TEST_LAYER_ID),
            ]
            .into(),
            crate::domain_types::SqlQualifiedIdentifier::new(
                identifier(constants_str::PUBLIC),
                identifier(constants_str::PG_TEST_FEATURES),
            ),
            vec![
                identifier(constants_str::SQL_NAMES_ID),
                identifier(constants_str::PG_TEST_LAYER_ID),
            ]
            .into(),
            super::PgScopedForeignKeyOnDelete::Cascade,
        )
        .expect("21fc516e scoped_foreign_key_uses_validated_composite_columns invariant must hold");
        assert_eq!(
            super::build_pg_scoped_foreign_key_clause(&foreign_key)
                .expect("594452b0 scoped_foreign_key_uses_validated_composite_columns invariant must hold")
                .into_inner(),
            constants_str::TEST_SCOPED_FOREIGN_KEY_CLAUSE
        );
    }
}
