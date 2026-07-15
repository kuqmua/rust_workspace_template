#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "password range fields retain semantic minimum-then-maximum order"
)]
#[derive(Clone, Copy, Debug)]
pub struct PasswordTextRef<'value_lt>(&'value_lt str);
impl<'value_lt> From<&'value_lt str> for PasswordTextRef<'value_lt> {
    fn from(value: &'value_lt str) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PasswordLength(usize);
impl From<usize> for PasswordLength {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PasswordLengthRange {
    minimum: PasswordLength,
    maximum: PasswordLength,
}
impl PasswordLengthRange {
    #[must_use]
    pub const fn from_prevalidated(minimum: PasswordLength, maximum: PasswordLength) -> Self {
        Self { minimum, maximum }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("password maximum length must not be less than minimum length")]
pub struct PasswordLengthRangeError;
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PasswordPolicyViolation {
    #[error("password must not contain whitespace")]
    ContainsWhitespace,
    #[error("password must contain a digit")]
    MissingDigit,
    #[error("password must contain a lowercase letter")]
    MissingLowercase,
    #[error("password must contain a special character")]
    MissingSpecial,
    #[error("password must contain an uppercase letter")]
    MissingUppercase,
    #[error("password is too long")]
    TooLong,
    #[error("password is too short")]
    TooShort,
}
pub fn validate_password_policy(
    password: PasswordTextRef<'_>,
    range: PasswordLengthRange,
) -> Result<(), PasswordPolicyViolation> {
    if password.0.len() < range.minimum.0 {
        return Err(PasswordPolicyViolation::TooShort);
    }
    if password.0.len() > range.maximum.0 {
        return Err(PasswordPolicyViolation::TooLong);
    }
    if password.0.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return Err(PasswordPolicyViolation::ContainsWhitespace);
    }
    if !password.0.bytes().any(|byte| byte.is_ascii_digit()) {
        return Err(PasswordPolicyViolation::MissingDigit);
    }
    if !password.0.bytes().any(|byte| byte.is_ascii_lowercase()) {
        return Err(PasswordPolicyViolation::MissingLowercase);
    }
    if !password.0.bytes().any(|byte| byte.is_ascii_uppercase()) {
        return Err(PasswordPolicyViolation::MissingUppercase);
    }
    if !password.0.bytes().any(|byte| byte.is_ascii_punctuation()) {
        return Err(PasswordPolicyViolation::MissingSpecial);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn password_policy_accepts_complete_password() {
        let range = super::PasswordLengthRange::try_from((
            super::PasswordLength::from(12usize),
            super::PasswordLength::from(128usize),
        ))
        .expect("6bea80c7");
        assert_eq!(
            super::validate_password_policy(
                super::PasswordTextRef::from(str_constants::TEST_STRONG_PASSWORD),
                range
            ),
            Ok(())
        );
    }
}
