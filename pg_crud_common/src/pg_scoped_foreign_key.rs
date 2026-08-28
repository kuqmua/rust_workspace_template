#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct PgScopedForeignKey {
    pub(super) local_columns: crate::domain_types::PgSqlIdentifiers,
    pub(super) referenced_columns: crate::domain_types::PgSqlIdentifiers,
    pub(super) referenced_table: crate::domain_types::SqlQualifiedIdentifier,
    pub(super) on_delete: crate::domain_types::PgScopedForeignKeyOnDelete,
}

impl PgScopedForeignKey {
    pub fn new(
        local_columns: crate::domain_types::PgSqlIdentifiers,
        referenced_table: crate::domain_types::SqlQualifiedIdentifier,
        referenced_columns: crate::domain_types::PgSqlIdentifiers,
        on_delete: crate::domain_types::PgScopedForeignKeyOnDelete,
    ) -> Result<Self, crate::domain_types::PgScopedForeignKeyError> {
        if local_columns.0.len() != referenced_columns.0.len() {
            return Err(crate::domain_types::PgScopedForeignKeyError::ColumnCountMismatch);
        }
        if !(crate::minimum_scoped_foreign_key_columns::MINIMUM_SCOPED_FOREIGN_KEY_COLUMNS
            ..=crate::maximum_scoped_foreign_key_columns::MAXIMUM_SCOPED_FOREIGN_KEY_COLUMNS)
            .contains(&local_columns.0.len())
        {
            return Err(crate::domain_types::PgScopedForeignKeyError::InvalidColumnCount);
        }
        if crate::contains_duplicate_identifier::contains_duplicate_identifier(
            local_columns.0.as_slice(),
        ) == crate::pg_duplicate_identifier_presence::PgDuplicateIdentifierPresence::Present
            || crate::contains_duplicate_identifier::contains_duplicate_identifier(
                referenced_columns.0.as_slice(),
            ) == crate::pg_duplicate_identifier_presence::PgDuplicateIdentifierPresence::Present
        {
            return Err(crate::domain_types::PgScopedForeignKeyError::DuplicateColumn);
        }
        Ok(Self {
            local_columns,
            referenced_columns,
            referenced_table,
            on_delete,
        })
    }
}
