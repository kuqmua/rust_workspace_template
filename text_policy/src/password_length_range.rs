#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::domain_types::{PasswordLength, PasswordLengthRangeError};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PasswordLengthRange {
    pub(super) minimum: PasswordLength,
    pub(super) maximum: PasswordLength,
}
impl PasswordLengthRange {
    #[must_use]
    pub const fn from_prevalidated(minimum: PasswordLength, maximum: PasswordLength) -> Self {
        Self { minimum, maximum }
    }
}
impl TryFrom<(PasswordLength, PasswordLength)> for PasswordLengthRange {
    type Error = PasswordLengthRangeError;
    fn try_from(value: (PasswordLength, PasswordLength)) -> Result<Self, Self::Error> {
        if value.1.0 < value.0.0 {
            Err(PasswordLengthRangeError)
        } else {
            Ok(Self {
                minimum: value.0,
                maximum: value.1,
            })
        }
    }
}
