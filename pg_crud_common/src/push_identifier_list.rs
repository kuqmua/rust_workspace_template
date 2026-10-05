pub(super) fn push_identifier_list(
    pg_scoped_foreign_key_clause_text: &mut crate::pg_scoped_foreign_key_clause_text::PgScopedForeignKeyClauseText,
    columns: &[crate::sql_identifier::SqlIdentifier],
) -> Result<(), bounded_types::bounded_string_error::BoundedStringError> {
    columns.iter().enumerate().try_for_each(|(index, column)| {
        if index != constants_usize::ZERO {
            pg_scoped_foreign_key_clause_text
                .get_inner_mut()
                .try_push_str(constants_str::TEXT_ALT_6)?;
        }
        pg_scoped_foreign_key_clause_text
            .get_inner_mut()
            .try_push_str(column.as_ref())
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_identifier_list_overflow_preserves_completed_appends() {
        let maximum_length = crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN;
        [
            (1usize, maximum_length.saturating_add(2usize), maximum_length, constants_str::EMPTY),
            (4usize, maximum_length.saturating_add(1usize), maximum_length.saturating_sub(1usize), constants_str::TEXT_ALT_6),
        ].into_iter().fold((), |(), (remaining_capacity, failed_length, observed_length, suffix)| {
            let prefix_length = maximum_length.saturating_sub(remaining_capacity);
            let clause_result = crate::pg_scoped_foreign_key_clause_text::PgScopedForeignKeyClauseText::try_from(constants_str::X.repeat(prefix_length));
            assert!(clause_result.is_ok());
            let Ok(mut clause) = clause_result else { return; };
            let columns_result = [constants_str::X, constants_str::AB].into_iter().map(|text| crate::sql_identifier::SqlIdentifier::try_from(text.to_owned())).collect::<Result<Vec<_>, _>>();
            assert!(columns_result.is_ok());
            let Ok(columns) = columns_result else { return; };
            let result = super::push_identifier_list(&mut clause, &columns);
            assert_eq!(result, Err(bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                actual_length: bounded_types::bounded_len::BoundedLen::from(failed_length),
                maximum_length: bounded_types::bounded_len::BoundedLen::from(maximum_length),
            }));
            assert_eq!(clause.get_inner().len().get(), observed_length);
            let expected_suffix = [constants_str::X, suffix].concat();
            assert!(clause.get_inner().as_str().ends_with(expected_suffix.as_str()));
            assert!(clause.get_inner().as_str().strip_suffix(suffix).is_some_and(|prefix| prefix.chars().all(|character| character == 'x')));
        });
    }
}
