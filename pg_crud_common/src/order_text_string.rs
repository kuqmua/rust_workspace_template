#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_display::Display,
)]
pub(crate) struct OrderTextString(String);

impl
    From<crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError>
    for OrderTextString
{
    fn from(
        pg_crud_string_wrapper_try_from_string_error: crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError,
    ) -> Self {
        Self(pg_crud_string_wrapper_try_from_string_error.to_string())
    }
}

impl TryFrom<String> for OrderTextString {
    type Error =
        crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() > crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: string.len(),
                max: crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(string))
    }
}
