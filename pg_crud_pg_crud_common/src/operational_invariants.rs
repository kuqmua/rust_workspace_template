const MAXIMUM_SCOPED_FOREIGN_KEY_COLUMNS: usize = 16usize;
const MINIMUM_SCOPED_FOREIGN_KEY_COLUMNS: usize = 2usize;

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgScopedForeignKeyOnDelete {
    Cascade,
    Restrict,
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PgScopedForeignKeyError {
    #[error("{}", str_constants::PG_SCOPED_FOREIGN_KEY_COLUMN_COUNT_MISMATCH)]
    ColumnCountMismatch,
    #[error("{}", str_constants::PG_SCOPED_FOREIGN_KEY_DUPLICATE_COLUMN)]
    DuplicateColumn,
    #[error("{}", str_constants::PG_SCOPED_FOREIGN_KEY_INVALID_COLUMN_COUNT)]
    InvalidColumnCount,
}
#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct PgSqlIdentifiers(Vec<crate::SqlIdentifier>);

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optml takes precedence over alphabetical field order
pub struct PgScopedForeignKey {
    local_columns: PgSqlIdentifiers,
    referenced_columns: PgSqlIdentifiers,
    referenced_table: crate::SqlQualifiedIdentifier,
    on_delete: PgScopedForeignKeyOnDelete,
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
enum PgDuplicateIdentifierPresence {
    Absent,
    Present,
}

#[derive(optml::Optml, Debug)]
struct PgScopedForeignKeyClauseText(String);

impl TryFrom<String> for PgScopedForeignKeyClauseText {
    type Error = crate::PgCrudStringWrapperTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            Err(crate::PgCrudStringWrapperTryFromStringError::TooLong {
                len: value.len(),
                max: crate::PG_CRUD_STRING_WRAPPER_MAX_LEN,
            })
        } else {
            Ok(Self(value))
        }
    }
}

impl PgScopedForeignKey {
    pub fn new(
        local_columns: PgSqlIdentifiers,
        referenced_table: crate::SqlQualifiedIdentifier,
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

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, newtype::FromInner)]
pub struct PgCounterValue(u64);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgCounterReconciliation {
    ActualAhead(PgCounterValue),
    InSync,
    TrackedAhead(PgCounterValue),
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PgOperationalLimit(u64);

impl TryFrom<u64> for PgOperationalLimit {
    type Error = PgOperationalLimitError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0u64 {
            Err(PgOperationalLimitError::ZeroLimit)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgOperationalLimitUpdateAuthority {
    MigrationDefault,
    Operator,
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PgOperationalLimitError {
    #[error("{}", str_constants::PG_OPERATIONAL_LIMIT_BELOW_CURRENT_USAGE)]
    BelowCurrentUsage,
    #[error("{}", str_constants::PG_OPERATIONAL_LIMIT_MUST_BE_GREATER_THAN_ZERO)]
    ZeroLimit,
}

pub fn build_pg_scoped_foreign_key_clause(
    foreign_key: &PgScopedForeignKey,
) -> Result<crate::QueryPartFragment, crate::PgCrudStringWrapperTryFromStringError> {
    let mut clause =
        PgScopedForeignKeyClauseText::try_from(String::from(str_constants::FOREIGN_KEY_OPENING))?;
    push_identifier_list(&mut clause, foreign_key.local_columns.0.as_slice());
    clause.0.push_str(str_constants::REFERENCES);
    clause
        .0
        .push_str(foreign_key.referenced_table.to_string().as_str());
    clause.0.push('(');
    push_identifier_list(&mut clause, foreign_key.referenced_columns.0.as_slice());
    clause.0.push(')');
    clause.0.push_str(match foreign_key.on_delete {
        PgScopedForeignKeyOnDelete::Cascade => str_constants::ON_DELETE_CASCADE,
        PgScopedForeignKeyOnDelete::Restrict => str_constants::ON_DELETE_RESTRICT,
    });
    crate::QueryPartFragment::try_from(clause.0)
}

fn contains_duplicate_identifier(
    columns: &[crate::SqlIdentifier],
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
    columns: &[crate::SqlIdentifier],
) {
    columns.iter().enumerate().for_each(|(index, column)| {
        if index != 0usize {
            output.0.push_str(str_constants::TEXT_ALT_6);
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
    fn identifier(value: &str) -> crate::SqlIdentifier {
        crate::SqlIdentifier::try_from(value.to_owned()).expect("2ec15e48")
    }

    fn limit(value: u64) -> super::PgOperationalLimit {
        super::PgOperationalLimit::try_from(value).expect("2710e8b4")
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
                identifier(str_constants::PG_TEST_FEATURE_ID),
                identifier(str_constants::PG_TEST_LAYER_ID),
            ]
            .into(),
            crate::SqlQualifiedIdentifier::new(
                identifier(str_constants::PUBLIC),
                identifier(str_constants::PG_TEST_FEATURES),
            ),
            vec![
                identifier(str_constants::SQL_NAMES_ID),
                identifier(str_constants::PG_TEST_LAYER_ID),
            ]
            .into(),
            super::PgScopedForeignKeyOnDelete::Cascade,
        )
        .expect("21fc516e");
        assert_eq!(
            super::build_pg_scoped_foreign_key_clause(&foreign_key)
                .expect("594452b0")
                .into_inner(),
            str_constants::TEST_SCOPED_FOREIGN_KEY_CLAUSE
        );
    }
}
