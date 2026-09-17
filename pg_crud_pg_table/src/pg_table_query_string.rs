#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_display::Display,
)]
pub struct PgTableQueryString(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::pg_tbl_string_wrapper_max_len::PG_TBL_STRING_WRAPPER_MAX_LEN },
        false,
    >,
);

impl From<crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError> for PgTableQueryString {
    fn from(value: crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError) -> Self {
        bounded_types::try_from_bounded_error_text::try_from_bounded_error_text(value)
    }
}

impl TryFrom<String> for PgTableQueryString {
    type Error = crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::pg_tbl_string_wrapper_max_len::PG_TBL_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: crate::pg_tbl_string_wrapper_max_len::PG_TBL_STRING_WRAPPER_MAX_LEN,
            });
        }
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    actual_length,
                    maximum_length,
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    actual_length,
                    minimum_length: maximum_length,
                } => Self::Error::TooLong {
                    len: actual_length.get(),
                    max: maximum_length.get(),
                },
            })
    }
}
