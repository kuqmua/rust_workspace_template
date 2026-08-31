pub(super) fn push_identifier_list(
    output: &mut crate::pg_scoped_foreign_key_clause_text::PgScopedForeignKeyClauseText,
    columns: &[crate::sql_identifier::SqlIdentifier],
) {
    columns.iter().enumerate().for_each(|(index, column)| {
        if index != constants_usize::ZERO {
            output.0.push_str(constants_str::TEXT_ALT_6);
        }
        output.0.push_str(column.as_ref());
    });
}
