pub use crate::bounded_text_policy_error::BoundedTextPolicyError;
pub use crate::fixed_length_ascii_hex_text::FixedLengthAsciiHexText;
pub use crate::fixed_length_ascii_hex_text_error::FixedLengthAsciiHexTextError;
pub use crate::non_empty_trimmed_text::NonEmptyTrimmedText;
pub use crate::password_length::PasswordLength;
pub use crate::password_length_range::PasswordLengthRange;
pub use crate::password_length_range_error::PasswordLengthRangeError;
pub use crate::password_policy_violation::PasswordPolicyViolation;
pub use crate::password_text_ref::PasswordTextRef;
pub use crate::required_nul_free_bounded_text::RequiredNulFreeBoundedText;
pub(crate) use crate::url_safe_token_part_maximum_bytes::URL_SAFE_TOKEN_PART_MAXIMUM_BYTES;
pub use crate::url_safe_token_part_maximum_bytes::UrlSafeTokenPartMaximumBytes;
pub use crate::url_safe_token_part_ref::UrlSafeTokenPartRef;
pub use crate::url_safe_token_part_text::UrlSafeTokenPartText;
pub use crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError;
pub use crate::validate_password_policy::validate_password_policy;
pub use crate::validate_url_safe_token_part::validate_url_safe_token_part;

#[cfg(test)]
mod tests {
    #[test]
    fn required_bounded_text_rejects_nul() {
        assert_eq!(
            super::RequiredNulFreeBoundedText::try_from(
                constants_str::TEST_TEXT_WITH_NUL.to_owned()
            ),
            Err(super::BoundedTextPolicyError::ContainsNul)
        );
    }

    #[test]
    fn fixed_hex_requires_lowercase_and_exact_length() {
        let _value = super::FixedLengthAsciiHexText::try_from(
            constants_str::TEST_GIT_COMMIT_HASH.to_owned(),
        )
        .expect("fdb4f77c fixed_hex_requires_lowercase_and_exact_length invariant must hold");
        assert_eq!(
            super::FixedLengthAsciiHexText::try_from(
                constants_str::TEST_UPPERCASE_GIT_COMMIT_HASH.to_owned()
            ),
            Err(super::FixedLengthAsciiHexTextError::InvalidSymbol)
        );
    }

    #[test]
    fn url_safe_token_policy_is_table_driven() {
        assert_eq!(
            [
                constants_str::ABC_ALT_3,
                constants_str::TEST_URL_TOKEN_WITH_SEPARATOR,
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
                super::UrlSafeTokenPartRef::from(constants_str::ABC_ALT_3),
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
                constants_str::TEST_STRONG_PASSWORD,
                constants_str::PASSWORD,
                constants_str::TEST_PASSWORD_WITH_WHITESPACE,
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
        let secret = constants_str::NEVER_PRINT_THIS_VALUE;
        assert!(!format!("{:?}", super::PasswordTextRef::from(secret)).contains(secret));
    }
}
