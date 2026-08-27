#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Eq, PartialEq, newtype::DisplayConst,
)]
#[display_const(constants_str::REDACTED_ALT_3)]
pub struct BoundedSecretText(pub(super) String);

impl TryFrom<String> for BoundedSecretText {
    type Error = super::BoundedSecretTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < super::SECRET_TEXT_MINIMUM_BYTES
            || value.len() > constants_usize::VALUE_8_192
        {
            return Err(super::BoundedSecretTextError::InvalidLength);
        }
        if value.trim().len() != value.len() {
            return Err(super::BoundedSecretTextError::SurroundingWhitespace);
        }
        if value
            .as_bytes()
            .first()
            .is_some_and(|first| value.as_bytes().iter().all(|byte| byte == first))
        {
            return Err(super::BoundedSecretTextError::RepeatedByte);
        }
        Ok(Self(value))
    }
}

impl std::fmt::Debug for BoundedSecretText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::REDACTED_ALT_3)
    }
}
