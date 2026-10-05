#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_into_inner::IntoInner,
)]
#[getters(get_mut)]
pub(super) struct PgScopedForeignKeyClauseText(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN },
        false,
    >,
);

impl TryFrom<String> for PgScopedForeignKeyClauseText {
    type Error =
        crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    actual_length,
                    maximum_length,
                } => Self::Error::TooLong {
                    len: actual_length.get(),
                    max: maximum_length.get(),
                },
                bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    actual_length,
                    minimum_length,
                } => Self::Error::TooLong {
                    len: actual_length.get(),
                    max: minimum_length.get(),
                },
            })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_scoped_foreign_key_text_preserves_exact_byte_limit_and_rejects_overflow() {
        let maximum = crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN;
        assert!([String::new(), constants_str::X.repeat(maximum)]
            .into_iter()
            .all(|value| {
                let length = value.len();
                crate::pg_scoped_foreign_key_clause_text::PgScopedForeignKeyClauseText::try_from(value)
                    .is_ok_and(|text| {
                        let inner = text.into_inner();
                        inner.as_str().len() == length
                            && inner.as_str().bytes().all(|byte| Some(&byte) == constants_str::X.as_bytes().first())
                    })
            }));
        assert!(matches!(
            crate::pg_scoped_foreign_key_clause_text::PgScopedForeignKeyClauseText::try_from(
                constants_str::X.repeat(maximum + 1),
            ),
            Err(crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong { len, max })
                if len == maximum + 1 && max == maximum
        ));
    }
}
