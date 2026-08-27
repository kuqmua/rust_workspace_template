#[path = "secret_text/bounded_secret_text.rs"]
mod bounded_secret_text;
#[path = "secret_text/bounded_secret_text_error.rs"]
mod bounded_secret_text_error;
#[path = "secret_text/secret_text_match.rs"]
mod secret_text_match;
#[path = "secret_text/secret_text_minimum_bytes.rs"]
mod secret_text_minimum_bytes;
#[path = "secret_text/secret_text_ref.rs"]
mod secret_text_ref;
#[path = "secret_text/secret_texts_match.rs"]
mod secret_texts_match;

pub use bounded_secret_text::BoundedSecretText;
pub use bounded_secret_text_error::BoundedSecretTextError;
pub use secret_text_match::SecretTextMatch;
use secret_text_minimum_bytes::SECRET_TEXT_MINIMUM_BYTES;
pub use secret_text_ref::SecretTextRef;
pub use secret_texts_match::secret_texts_match;

#[cfg(test)]
mod tests {
    fn secret(value: &str) -> super::BoundedSecretText {
        super::BoundedSecretText::try_from(value.to_owned())
            .expect("2c20f43d secret invariant must hold")
    }

    #[test]
    fn secrets_are_redacted_validated_and_compared() {
        let expected = secret(constants_str::TEST_SECRET_TEXT);
        let equal = secret(constants_str::TEST_SECRET_TEXT);
        let different = secret(constants_str::TEST_DIFFERENT_SECRET_TEXT);
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
