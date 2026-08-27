#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, newtype::DerefTarget, newtype::Display,
)]
pub struct PgTableQueryString(pub(super) String);

impl From<super::PgTableStringWrapperTryFromStringError> for PgTableQueryString {
    fn from(value: super::PgTableStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}

impl TryFrom<String> for PgTableQueryString {
    type Error = super::PgTableStringWrapperTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > super::PG_TBL_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: super::PG_TBL_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
