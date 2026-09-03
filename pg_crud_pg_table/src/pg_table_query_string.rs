#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_display::Display,
)]
pub struct PgTableQueryString(String);

impl From<crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError> for PgTableQueryString {
    fn from(pg_table_string_wrapper_try_from_string_error: crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError) -> Self {
        Self(pg_table_string_wrapper_try_from_string_error.to_string())
    }
}

impl TryFrom<String> for PgTableQueryString {
    type Error = crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() > crate::pg_tbl_string_wrapper_max_len::PG_TBL_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: string.len(),
                max: crate::pg_tbl_string_wrapper_max_len::PG_TBL_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(string))
    }
}
