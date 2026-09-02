pub(super) fn push_identifier_list(
    pg_scoped_foreign_key_clause_text: &mut crate::pg_scoped_foreign_key_clause_text::PgScopedForeignKeyClauseText,
    columns: &[crate::sql_identifier::SqlIdentifier],
) {
    columns.iter().enumerate().for_each(|(index, column)| {
        if index != constants_usize::ZERO {
            pg_scoped_foreign_key_clause_text
                .get_inner_mut()
                .push_str(constants_str::TEXT_ALT_6);
        }
        pg_scoped_foreign_key_clause_text
            .get_inner_mut()
            .push_str(column.as_ref());
    });
}
