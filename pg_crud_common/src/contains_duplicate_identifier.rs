pub(super) fn contains_duplicate_identifier(
    columns: &[crate::sql_identifier::SqlIdentifier],
) -> crate::pg_duplicate_identifier_presence::PgDuplicateIdentifierPresence {
    if columns
        .iter()
        .enumerate()
        .any(|(index, column)| columns.iter().take(index).any(|seen| seen == column))
    {
        crate::pg_duplicate_identifier_presence::PgDuplicateIdentifierPresence::Present
    } else {
        crate::pg_duplicate_identifier_presence::PgDuplicateIdentifierPresence::Absent
    }
}
