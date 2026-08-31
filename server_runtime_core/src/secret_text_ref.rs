#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub struct SecretTextRef<'value_lt>(&'value_lt str);

impl<'value_lt> SecretTextRef<'value_lt> {
    pub(super) const fn as_str(self) -> &'value_lt str {
        self.0
    }
}

impl<'value_lt> TryFrom<&'value_lt str> for SecretTextRef<'value_lt> {
    type Error = crate::bounded_secret_text_error::BoundedSecretTextError;

    fn try_from(value: &'value_lt str) -> Result<Self, Self::Error> {
        if value.len() < crate::secret_text_minimum_bytes::SECRET_TEXT_MINIMUM_BYTES
            || value.len() > constants_usize::VALUE_8_192
        {
            return Err(crate::bounded_secret_text_error::BoundedSecretTextError::InvalidLength);
        }
        if value.trim().len() != value.len() {
            return Err(
                crate::bounded_secret_text_error::BoundedSecretTextError::SurroundingWhitespace,
            );
        }
        if value
            .as_bytes()
            .first()
            .is_some_and(|first| value.as_bytes().iter().all(|byte| byte == first))
        {
            return Err(crate::bounded_secret_text_error::BoundedSecretTextError::RepeatedByte);
        }
        Ok(Self(value))
    }
}

impl<'value_lt> From<&'value_lt crate::bounded_secret_text::BoundedSecretText>
    for SecretTextRef<'value_lt>
{
    fn from(value: &'value_lt crate::bounded_secret_text::BoundedSecretText) -> Self {
        Self(value.as_str())
    }
}

impl std::fmt::Debug for SecretTextRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::catalog::REDACTED_ALT_3)
    }
}
