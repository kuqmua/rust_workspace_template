#[derive(
    generate_accessor::Getters,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct PgScopedForeignKey {
    local_columns: crate::pg_sql_identifiers::PgSqlIdentifiers,
    referenced_columns: crate::pg_sql_identifiers::PgSqlIdentifiers,
    referenced_table: crate::sql_qualified_identifier::SqlQualifiedIdentifier,
    on_delete: crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete,
}

impl PgScopedForeignKey {
    pub fn new(
        local_columns: crate::pg_sql_identifiers::PgSqlIdentifiers,
        referenced_table: crate::sql_qualified_identifier::SqlQualifiedIdentifier,
        referenced_columns: crate::pg_sql_identifiers::PgSqlIdentifiers,
        on_delete: crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete,
    ) -> Result<Self, crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError> {
        if local_columns.get_inner().len() != referenced_columns.get_inner().len() {
            return Err(
                crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError::ColumnCountMismatch,
            );
        }
        if !(crate::minimum_scoped_foreign_key_columns::MINIMUM_SCOPED_FOREIGN_KEY_COLUMNS
            ..=crate::maximum_scoped_foreign_key_columns::MAXIMUM_SCOPED_FOREIGN_KEY_COLUMNS)
            .contains(&local_columns.get_inner().len())
        {
            return Err(
                crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError::InvalidColumnCount,
            );
        }
        if crate::contains_duplicate_identifier::contains_duplicate_identifier(
            local_columns.get_inner().as_slice(),
        ) == crate::pg_duplicate_identifier_presence::PgDuplicateIdentifierPresence::Present
            || crate::contains_duplicate_identifier::contains_duplicate_identifier(
                referenced_columns.get_inner().as_slice(),
            ) == crate::pg_duplicate_identifier_presence::PgDuplicateIdentifierPresence::Present
        {
            return Err(
                crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError::DuplicateColumn,
            );
        }
        Ok(Self {
            local_columns,
            referenced_columns,
            referenced_table,
            on_delete,
        })
    }
}
