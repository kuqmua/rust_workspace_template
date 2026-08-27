pub(super) fn contains_duplicate_identifier(
    columns: &[crate::domain_types::SqlIdentifier],
) -> crate::domain_types::pg_duplicate_identifier_presence::PgDuplicateIdentifierPresence {
    if columns
        .iter()
        .enumerate()
        .any(|(index, column)| columns.iter().take(index).any(|seen| seen == column))
    {
        crate::domain_types::pg_duplicate_identifier_presence::PgDuplicateIdentifierPresence::Present
    } else {
        crate::domain_types::pg_duplicate_identifier_presence::PgDuplicateIdentifierPresence::Absent
    }
}
