pub use crate::bounded_secret_text::BoundedSecretText;
pub use crate::bounded_secret_text_error::BoundedSecretTextError;
pub use crate::secret_text_match::SecretTextMatch;
use crate::secret_text_minimum_bytes::SECRET_TEXT_MINIMUM_BYTES;
pub use crate::secret_text_ref::SecretTextRef;
pub use crate::secret_texts_match::secret_texts_match;

#[cfg(test)]
mod tests {
    fn bounded_secret(value: &str) -> super::BoundedSecretText {
        super::BoundedSecretText::try_from(value.to_owned())
            .expect("2c20f43d secret invariant must hold")
    }

    #[test]
    fn secrets_are_redacted_validated_and_compared() {
        let expected = bounded_secret(constants_str::TEST_SECRET_TEXT);
        let equal = bounded_secret(constants_str::TEST_SECRET_TEXT);
        let different = bounded_secret(constants_str::TEST_DIFFERENT_SECRET_TEXT);
        assert_eq!(
            super::secret_texts_match((&expected).into(), (&equal).into()),
            super::SecretTextMatch::Equal,
        );
        assert_eq!(
            super::secret_texts_match((&expected).into(), (&different).into()),
            super::SecretTextMatch::Different,
        );
        assert_eq!(format!("{expected:?}"), constants_str::REDACTED_ALT_3);
        assert!(matches!(
            super::SecretTextRef::try_from(constants_str::TEST_SECRET_TEXT),
            Ok(_value)
        ));
        assert_eq!(
            super::BoundedSecretText::try_from(String::from(constants_str::TEST_REPEATED_SECRET)),
            Err(super::BoundedSecretTextError::RepeatedByte),
        );
    }
}
