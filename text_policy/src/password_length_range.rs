#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PasswordLengthRange {
    minimum: crate::password_length::PasswordLength,
    maximum: crate::password_length::PasswordLength,
}
impl PasswordLengthRange {
    #[must_use]
    pub const fn from_prevalidated(
        minimum: crate::password_length::PasswordLength,
        maximum: crate::password_length::PasswordLength,
    ) -> Self {
        Self { minimum, maximum }
    }

    #[must_use]
    pub const fn into_parts(
        self,
    ) -> (
        crate::password_length::PasswordLength,
        crate::password_length::PasswordLength,
    ) {
        (self.minimum, self.maximum)
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
        if usize::from(value.1) < usize::from(value.0) {
            Err(crate::password_length_range_error::PasswordLengthRangeError::Invalid)
        } else {
            Ok(Self {
                minimum: value.0,
                maximum: value.1,
            })
        }
    }
}
