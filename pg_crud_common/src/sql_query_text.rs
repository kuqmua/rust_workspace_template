#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::IntoInnerFrom,
    generate_accessor::Getters,
)]
#[getters(get_mut)]
pub(crate) struct SqlQueryText(String);

impl
    From<crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError>
    for SqlQueryText
{
    fn from(
        value: crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError,
    ) -> Self {
        Self(value.to_string())
    }
}

impl TryFrom<String> for SqlQueryText {
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
            Ok(Self(value))
        }
    }
}
