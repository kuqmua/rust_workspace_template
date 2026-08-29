#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(super) struct PgScopedForeignKeyClauseText(pub(super) String);

impl TryFrom<String> for PgScopedForeignKeyClauseText {
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
