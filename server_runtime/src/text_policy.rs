const TEXT_POLICY_MAXIMUM_BYTES: usize = 1_048_576usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum BoundedTextPolicyError {
    #[error("text contains a NUL character")]
    ContainsNul,
    #[error("text must not be empty")]
    Empty,
    #[error("text exceeds its maximum byte length")]
    TooLong,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequiredNulFreeBoundedText(String);
impl AsRef<str> for RequiredNulFreeBoundedText {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonEmptyTrimmedText(String);
impl AsRef<str> for NonEmptyTrimmedText {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum FixedLengthAsciiHexTextError {
    #[error("hexadecimal text has an unexpected length")]
    InvalidLength,
    #[error("hexadecimal text must contain only lowercase ASCII hexadecimal digits")]
    InvalidSymbol,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedLengthAsciiHexText(String);
impl AsRef<str> for FixedLengthAsciiHexText {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum UrlSafeTokenPartTextError {
    #[error("URL-safe token part must not be empty")]
    Empty,
    #[error("URL-safe token part contains a forbidden symbol")]
    InvalidSymbol,
    #[error("URL-safe token part is too long")]
    TooLong,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UrlSafeTokenPartText(String);
impl AsRef<str> for UrlSafeTokenPartText {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl TryFrom<String> for UrlSafeTokenPartText {
    type Error = UrlSafeTokenPartTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 4096usize {
            return Err(Self::Error::TooLong);
        }
        if value.is_empty() {
            Err(Self::Error::Empty)
        } else if value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            Ok(Self(value))
        } else {
            Err(Self::Error::InvalidSymbol)
        }
    }
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
        .expect("a5301b6c");
        assert_eq!(
            super::FixedLengthAsciiHexText::try_from(
                str_constants::TEST_UPPERCASE_GIT_COMMIT_HASH.to_owned()
            ),
            Err(super::FixedLengthAsciiHexTextError::InvalidSymbol)
        );
    }

    #[test]
    fn url_safe_token_rejects_separator() {
        assert_eq!(
            super::UrlSafeTokenPartText::try_from(
                str_constants::TEST_URL_TOKEN_WITH_SEPARATOR.to_owned()
            ),
            Err(super::UrlSafeTokenPartTextError::InvalidSymbol)
        );
    }
}
