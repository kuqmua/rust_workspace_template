#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub struct SecretTextRef<'value_lt>(pub(super) &'value_lt str);

impl<'value_lt> TryFrom<&'value_lt str> for SecretTextRef<'value_lt> {
    type Error = super::BoundedSecretTextError;

    fn try_from(value: &'value_lt str) -> Result<Self, Self::Error> {
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

impl<'value_lt> From<&'value_lt super::BoundedSecretText> for SecretTextRef<'value_lt> {
    fn from(value: &'value_lt super::BoundedSecretText) -> Self {
        Self(value.0.as_str())
    }
}

impl std::fmt::Debug for SecretTextRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::REDACTED_ALT_3)
    }
}
