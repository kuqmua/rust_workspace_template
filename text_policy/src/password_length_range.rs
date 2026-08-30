#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PasswordLengthRange {
    pub(super) minimum: crate::password_length::PasswordLength,
    pub(super) maximum: crate::password_length::PasswordLength,
}
impl PasswordLengthRange {
    #[must_use]
    pub const fn from_prevalidated(
        minimum: crate::password_length::PasswordLength,
        maximum: crate::password_length::PasswordLength,
    ) -> Self {
        Self { minimum, maximum }
    }
}
impl
    TryFrom<(
        crate::password_length::PasswordLength,
        crate::password_length::PasswordLength,
    )> for PasswordLengthRange
{
    type Error = crate::password_length_range_error::PasswordLengthRangeError;
    fn try_from(
        value: (
            crate::password_length::PasswordLength,
            crate::password_length::PasswordLength,
        ),
    ) -> Result<Self, Self::Error> {
        if value.1.0 < value.0.0 {
            Err(crate::password_length_range_error::PasswordLengthRangeError::Invalid)
        } else {
            Ok(Self {
                minimum: value.0,
                maximum: value.1,
            })
        }
    }
}
