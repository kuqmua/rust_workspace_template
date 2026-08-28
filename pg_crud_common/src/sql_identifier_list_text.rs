#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
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
