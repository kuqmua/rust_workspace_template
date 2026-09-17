#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
)]
pub(crate) struct SqlIdentifierListText(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN },
        false,
    >,
);

impl TryFrom<String> for SqlIdentifierListText {
    type Error =
        crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            Err(
                crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong {
                    len: value.len(),
                    max: crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN,
                },
            )
        } else {
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
}
