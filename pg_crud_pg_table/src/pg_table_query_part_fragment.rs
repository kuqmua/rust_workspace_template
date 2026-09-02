#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::Display,
)]
pub struct PgTableQueryPartFragment(String);
impl From<crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError> for PgTableQueryPartFragment {
    fn from(pg_table_string_wrapper_try_from_string_error: crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError) -> Self {
        Self(pg_table_string_wrapper_try_from_string_error.to_string())
    }
}
impl TryFrom<String> for PgTableQueryPartFragment {
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
