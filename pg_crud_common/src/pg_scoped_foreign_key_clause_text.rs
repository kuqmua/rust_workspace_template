#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(super) struct PgScopedForeignKeyClauseText(pub(super) String);

impl TryFrom<String> for PgScopedForeignKeyClauseText {
    type Error = crate::domain_types::PgCrudStringWrapperTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::domain_types::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            Err(
                crate::domain_types::PgCrudStringWrapperTryFromStringError::TooLong {
                    len: value.len(),
                    max: crate::domain_types::PG_CRUD_STRING_WRAPPER_MAX_LEN,
                },
            )
        } else {
            Ok(Self(value))
        }
    }
}
