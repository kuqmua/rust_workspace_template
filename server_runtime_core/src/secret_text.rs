const SECRET_TEXT_MAXIMUM_BYTES: usize = 8192usize;
const SECRET_TEXT_MINIMUM_BYTES: usize = 16usize;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum BoundedSecretTextError {
    #[error("secret text length is outside the allowed range")]
    InvalidLength,
    #[error("secret text repeats one byte")]
    RepeatedByte,
    #[error("secret text contains surrounding whitespace")]
    SurroundingWhitespace,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Eq, PartialEq, newtype::DisplayConst,
)]
#[display_const(str_constants::REDACTED_ALT_3)]
pub struct BoundedSecretText(String);
impl TryFrom<String> for BoundedSecretText {
    type Error = BoundedSecretTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < SECRET_TEXT_MINIMUM_BYTES || value.len() > SECRET_TEXT_MAXIMUM_BYTES {
            return Err(BoundedSecretTextError::InvalidLength);
        }
        if value.trim().len() != value.len() {
            return Err(BoundedSecretTextError::SurroundingWhitespace);
        }
        if value
            .as_bytes()
            .first()
            .is_some_and(|first| value.as_bytes().iter().all(|byte| byte == first))
        {
            return Err(BoundedSecretTextError::RepeatedByte);
        }
        Ok(Self(value))
    }
}
impl std::fmt::Debug for BoundedSecretText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::REDACTED_ALT_3)
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub struct SecretTextRef<'value_lt>(&'value_lt str);
impl<'value_lt> TryFrom<&'value_lt str> for SecretTextRef<'value_lt> {
    type Error = BoundedSecretTextError;
    fn try_from(value: &'value_lt str) -> Result<Self, Self::Error> {
        if value.len() < SECRET_TEXT_MINIMUM_BYTES || value.len() > SECRET_TEXT_MAXIMUM_BYTES {
            return Err(BoundedSecretTextError::InvalidLength);
        }
        if value.trim().len() != value.len() {
            return Err(BoundedSecretTextError::SurroundingWhitespace);
        }
        if value
            .as_bytes()
            .first()
            .is_some_and(|first| value.as_bytes().iter().all(|byte| byte == first))
        {
            return Err(BoundedSecretTextError::RepeatedByte);
        }
        Ok(Self(value))
    }
}
impl<'value_lt> From<&'value_lt BoundedSecretText> for SecretTextRef<'value_lt> {
    fn from(value: &'value_lt BoundedSecretText) -> Self {
        Self(value.0.as_str())
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecretTextMatch {
    Different,
    Equal,
}
impl std::fmt::Debug for SecretTextRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::REDACTED_ALT_3)
    }
}

#[must_use]
pub fn secret_texts_match(
    expected: SecretTextRef<'_>,
    provided: SecretTextRef<'_>,
) -> SecretTextMatch {
    let expected_bytes = expected.0.as_bytes();
    let provided_bytes = provided.0.as_bytes();
    let length_difference = expected_bytes.len() ^ provided_bytes.len();
    let difference =
        (0usize..SECRET_TEXT_MAXIMUM_BYTES).fold(length_difference, |accumulated, index| {
            let expected_byte = expected_bytes.get(index).copied().unwrap_or_default();
            let provided_byte = provided_bytes.get(index).copied().unwrap_or_default();
            accumulated | usize::from(expected_byte ^ provided_byte)
        });
    if difference == 0usize {
        SecretTextMatch::Equal
    } else {
        SecretTextMatch::Different
    }
}

#[cfg(test)]
mod tests {
    fn secret(value: &str) -> super::BoundedSecretText {
        super::BoundedSecretText::try_from(value.to_owned())
            .expect("2c20f43d secret invariant must hold")
    }

    #[test]
    fn secrets_are_redacted_validated_and_compared() {
        let expected = secret(str_constants::TEST_SECRET_TEXT);
        let equal = secret(str_constants::TEST_SECRET_TEXT);
        let different = secret(str_constants::TEST_DIFFERENT_SECRET_TEXT);
        assert_eq!(
            super::secret_texts_match((&expected).into(), (&equal).into()),
            super::SecretTextMatch::Equal,
        );
        assert_eq!(
            super::secret_texts_match((&expected).into(), (&different).into()),
            super::SecretTextMatch::Different,
        );
        assert_eq!(format!("{expected:?}"), str_constants::REDACTED_ALT_3);
        assert!(matches!(
            super::SecretTextRef::try_from(str_constants::TEST_SECRET_TEXT),
            Ok(_value)
        ));
        assert_eq!(
            super::BoundedSecretText::try_from(String::from(str_constants::TEST_REPEATED_SECRET)),
            Err(super::BoundedSecretTextError::RepeatedByte),
        );
    }
}
