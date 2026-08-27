#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
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
        if !(crate::domain_types::minimum_scoped_foreign_key_columns::MINIMUM_SCOPED_FOREIGN_KEY_COLUMNS
            ..=crate::domain_types::maximum_scoped_foreign_key_columns::MAXIMUM_SCOPED_FOREIGN_KEY_COLUMNS)
            .contains(&local_columns.0.len())
        {
            return Err(crate::domain_types::PgScopedForeignKeyError::InvalidColumnCount);
        }
        if crate::domain_types::contains_duplicate_identifier::contains_duplicate_identifier(
            local_columns.0.as_slice(),
        ) == crate::domain_types::pg_duplicate_identifier_presence::PgDuplicateIdentifierPresence::Present
            || crate::domain_types::contains_duplicate_identifier::contains_duplicate_identifier(
                referenced_columns.0.as_slice(),
            ) == crate::domain_types::pg_duplicate_identifier_presence::PgDuplicateIdentifierPresence::Present
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
