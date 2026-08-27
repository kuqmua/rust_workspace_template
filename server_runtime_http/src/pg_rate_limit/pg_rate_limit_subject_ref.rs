#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRateLimitSubjectRef<'value_lt>(pub(super) &'value_lt str);

impl<'value_lt> TryFrom<&'value_lt str> for PgRateLimitSubjectRef<'value_lt> {
    type Error = super::PgRateLimitValidationError;

    fn try_from(value: &'value_lt str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(super::PgRateLimitValidationError::EmptyKeyPart)
        } else if value.len() > super::PG_RATE_LIMIT_KEY_PART_MAX_LEN {
            Err(super::PgRateLimitValidationError::KeyPartTooLong)
        } else {
            Ok(Self(value))
        }
    }
}
