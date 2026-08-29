#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    generate_accessor::Getters,
)]
pub(crate) struct SqlIdentifierListText(String);

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
            Ok(Self(value))
        }
    }
}
