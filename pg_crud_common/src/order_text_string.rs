#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
#[derive(
    Debug, Clone, PartialEq, Eq, optimal_memory_layout::OptimalMemoryLayout, newtype::Display,
)]
pub(crate) struct OrderTextString(String);

impl From<crate::domain_types::PgCrudStringWrapperTryFromStringError> for OrderTextString {
    fn from(value: crate::domain_types::PgCrudStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}

impl TryFrom<String> for OrderTextString {
    type Error = crate::domain_types::PgCrudStringWrapperTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::domain_types::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: crate::domain_types::PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
