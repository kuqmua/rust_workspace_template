#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "pg scoped foreign key keeps declaration order aligned with generated layout or processing flow"
)]
pub struct PgScopedForeignKey {
    local_columns: crate::pg_sql_identifiers::PgSqlIdentifiers,
    referenced_columns: crate::pg_sql_identifiers::PgSqlIdentifiers,
    referenced_table: crate::sql_qualified_identifier::SqlQualifiedIdentifier,
    on_delete: crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete,
}

impl PgScopedForeignKey {
    pub fn new(
        local_columns: crate::pg_sql_identifiers::PgSqlIdentifiers,
        sql_qualified_identifier: crate::sql_qualified_identifier::SqlQualifiedIdentifier,
        referenced_columns: crate::pg_sql_identifiers::PgSqlIdentifiers,
        pg_scoped_foreign_key_on_delete: crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete,
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
            referenced_table: sql_qualified_identifier,
            on_delete: pg_scoped_foreign_key_on_delete,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_foreign_key_column_bounds_and_error_precedence() {
        let verify = || -> Result<bool, crate::sql_identifier_error::SqlIdentifierError> {
            let columns = |count, duplicate| {
                (0..count)
                    .map(|index| {
                        crate::sql_identifier::SqlIdentifier::try_from(format!(
                            "{}{}",
                            constants_str::X,
                            if duplicate { 0usize } else { index }
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(crate::pg_sql_identifiers::PgSqlIdentifiers::from)
            };
            let table = crate::sql_qualified_identifier::SqlQualifiedIdentifier::new(
                crate::sql_identifier::SqlIdentifier::try_from(constants_str::X.to_owned())?,
                crate::sql_identifier::SqlIdentifier::try_from(constants_str::TABLE.to_owned())?,
            );
            [
                (0usize, 0usize, false, false, Some(crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError::InvalidColumnCount)),
                (0usize, 1usize, false, false, Some(crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError::ColumnCountMismatch)),
                (1usize, 0usize, false, false, Some(crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError::ColumnCountMismatch)),
                (1usize, 1usize, false, false, Some(crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError::InvalidColumnCount)),
                (2usize, 2usize, false, false, None),
                (2usize, 2usize, true, false, Some(crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError::DuplicateColumn)),
                (2usize, 2usize, false, true, Some(crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError::DuplicateColumn)),
                (2usize, 2usize, true, true, Some(crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError::DuplicateColumn)),
                (16usize, 16usize, false, false, None),
                (17usize, 17usize, false, false, Some(crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError::InvalidColumnCount)),
                (17usize, 16usize, false, false, Some(crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError::ColumnCountMismatch)),
            ]
            .into_iter()
            .try_fold(true, |valid, (local_count, referenced_count, duplicate_local, duplicate_referenced, expected_error)| {
                let local_columns = columns(local_count, duplicate_local)?;
                let referenced_columns = columns(referenced_count, duplicate_referenced)?;
                let actual = crate::pg_scoped_foreign_key::PgScopedForeignKey::new(
                    local_columns.clone(),
                    table.clone(),
                    referenced_columns.clone(),
                    crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete::Restrict,
                );
                Ok::<_, crate::sql_identifier_error::SqlIdentifierError>(valid && expected_error.map_or_else(
                    || actual.as_ref().is_ok_and(|foreign_key| {
                        foreign_key.get_local_columns() == &local_columns
                            && foreign_key.get_referenced_columns() == &referenced_columns
                            && foreign_key.get_referenced_table() == &table
                            && *foreign_key.get_on_delete() == crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete::Restrict
                    }),
                    |error| actual == Err(error),
                ))
            })
        };
        assert!(verify().is_ok_and(|valid| valid));
    }
}
