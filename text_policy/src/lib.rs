#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "validators stay adjacent to their domain wrappers and ranges retain minimum-then-maximum order"
)]

const TEXT_POLICY_MAXIMUM_BYTES: usize = 1_048_576usize;
const URL_SAFE_TOKEN_PART_MAXIMUM_BYTES: usize = 4096usize;

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum BoundedTextPolicyError {
    #[error("text contains a NUL character")]
    ContainsNul,
    #[error("text must not be empty")]
    Empty,
    #[error("text exceeds its maximum byte length")]
    TooLong,
}

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
pub struct RequiredNulFreeBoundedText(String);
impl TryFrom<String> for RequiredNulFreeBoundedText {
    type Error = BoundedTextPolicyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > TEXT_POLICY_MAXIMUM_BYTES {
            return Err(Self::Error::TooLong);
        }
        if value.is_empty() {
            Err(Self::Error::Empty)
        } else if value.contains('\0') {
            Err(Self::Error::ContainsNul)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
pub struct NonEmptyTrimmedText(String);
impl TryFrom<String> for NonEmptyTrimmedText {
    type Error = BoundedTextPolicyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > TEXT_POLICY_MAXIMUM_BYTES {
            return Err(Self::Error::TooLong);
        }
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(Self::Error::Empty)
        } else if trimmed.contains('\0') {
            Err(Self::Error::ContainsNul)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum FixedLengthAsciiHexTextError {
    #[error("hexadecimal text has an unexpected length")]
    InvalidLength,
    #[error("hexadecimal text must contain only lowercase ASCII hexadecimal digits")]
    InvalidSymbol,
}
#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr, newtype::IntoInner)]
pub struct FixedLengthAsciiHexText(String);
impl TryFrom<String> for FixedLengthAsciiHexText {
    type Error = FixedLengthAsciiHexTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 40usize {
            Err(Self::Error::InvalidLength)
        } else if !value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            Err(Self::Error::InvalidSymbol)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct UrlSafeTokenPartMaximumBytes(usize);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct UrlSafeTokenPartRef<'value_lt>(&'value_lt str);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum UrlSafeTokenPartTextError {
    #[error("URL-safe token part must not be empty")]
    Empty,
    #[error("URL-safe token part contains a forbidden symbol")]
    InvalidSymbol,
    #[error("URL-safe token part is too long")]
    TooLong,
}

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
pub struct UrlSafeTokenPartText(String);
impl TryFrom<String> for UrlSafeTokenPartText {
    type Error = UrlSafeTokenPartTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > URL_SAFE_TOKEN_PART_MAXIMUM_BYTES {
            return Err(Self::Error::TooLong);
        }
        validate_url_safe_token_part(
            UrlSafeTokenPartRef::from(value.as_str()),
            UrlSafeTokenPartMaximumBytes::from(URL_SAFE_TOKEN_PART_MAXIMUM_BYTES),
        )?;
        Ok(Self(value))
    }
}

pub fn validate_url_safe_token_part(
    value: UrlSafeTokenPartRef<'_>,
    maximum_bytes: UrlSafeTokenPartMaximumBytes,
) -> Result<(), UrlSafeTokenPartTextError> {
    if value.0.len() > maximum_bytes.0 {
        return Err(UrlSafeTokenPartTextError::TooLong);
    }
    if value.0.is_empty() {
        Err(UrlSafeTokenPartTextError::Empty)
    } else if value
        .0
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        Ok(())
    } else {
        Err(UrlSafeTokenPartTextError::InvalidSymbol)
    }
}

#[derive(optml::Optml, Clone, Copy, newtype::FromInner)]
pub struct PasswordTextRef<'value_lt>(&'value_lt str);
impl std::fmt::Debug for PasswordTextRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct PasswordLength(usize);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
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
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
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
    fn required_bounded_text_rejects_nul() {
        assert_eq!(
            super::RequiredNulFreeBoundedText::try_from(
                str_constants::TEST_TEXT_WITH_NUL.to_owned()
            ),
            Err(super::BoundedTextPolicyError::ContainsNul)
        );
    }

    #[test]
    fn fixed_hex_requires_lowercase_and_exact_length() {
        let _value = super::FixedLengthAsciiHexText::try_from(
            str_constants::TEST_GIT_COMMIT_HASH.to_owned(),
        )
        .expect("fdb4f77c");
        assert_eq!(
            super::FixedLengthAsciiHexText::try_from(
                str_constants::TEST_UPPERCASE_GIT_COMMIT_HASH.to_owned()
            ),
            Err(super::FixedLengthAsciiHexTextError::InvalidSymbol)
        );
    }

    #[test]
    fn url_safe_token_policy_is_table_driven() {
        assert_eq!(
            [
                str_constants::ABC_ALT_3,
                str_constants::TEST_URL_TOKEN_WITH_SEPARATOR,
                "",
            ]
            .map(|value| {
                super::validate_url_safe_token_part(
                    super::UrlSafeTokenPartRef::from(value),
                    super::UrlSafeTokenPartMaximumBytes::from(128usize),
                )
            }),
            [
                Ok(()),
                Err(super::UrlSafeTokenPartTextError::InvalidSymbol),
                Err(super::UrlSafeTokenPartTextError::Empty),
            ]
        );
        assert_eq!(
            super::validate_url_safe_token_part(
                super::UrlSafeTokenPartRef::from(str_constants::ABC_ALT_3),
                super::UrlSafeTokenPartMaximumBytes::from(2usize),
            ),
            Err(super::UrlSafeTokenPartTextError::TooLong)
        );
    }

    #[test]
    fn password_policy_is_table_driven() {
        let range = super::PasswordLengthRange::from_prevalidated(
            super::PasswordLength::from(12usize),
            super::PasswordLength::from(128usize),
        );
        assert_eq!(
            [
                str_constants::TEST_STRONG_PASSWORD,
                str_constants::PASSWORD,
                str_constants::TEST_PASSWORD_WITH_WHITESPACE,
            ]
            .map(|value| {
                super::validate_password_policy(super::PasswordTextRef::from(value), range)
            }),
            [
                Ok(()),
                Err(super::PasswordPolicyViolation::TooShort),
                Err(super::PasswordPolicyViolation::ContainsWhitespace),
            ]
        );
        let secret = str_constants::NEVER_PRINT_THIS_VALUE;
        assert!(!format!("{:?}", super::PasswordTextRef::from(secret)).contains(secret));
    }
}
