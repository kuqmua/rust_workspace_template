#[cfg(test)]
mod tests {
    fn bounded_secret(value: &str) -> crate::bounded_secret_text::BoundedSecretText {
        crate::bounded_secret_text::BoundedSecretText::try_from(value.to_owned())
            .expect("2c20f43d secret invariant must hold")
    }

    #[test]
    fn test_secrets_are_redacted_validated_and_compared() {
        let expected = bounded_secret(constants_str::TEST_SECRET_TEXT);
        let equal = bounded_secret(constants_str::TEST_SECRET_TEXT);
        let different = bounded_secret(constants_str::TEST_DIFFERENT_SECRET_TEXT);
        assert_eq!(
            crate::secret_texts_match::secret_texts_match((&expected).into(), (&equal).into()),
            crate::secret_text_match::SecretTextMatch::Equal,
        );
        assert_eq!(
            crate::secret_texts_match::secret_texts_match((&expected).into(), (&different).into()),
            crate::secret_text_match::SecretTextMatch::Different,
        );
        assert_eq!(format!("{expected:?}"), constants_str::REDACTED_ALT_3);
        assert!(matches!(
            crate::secret_text_ref::SecretTextRef::try_from(constants_str::TEST_SECRET_TEXT),
            Ok(_value)
        ));
        assert_eq!(
            crate::bounded_secret_text::BoundedSecretText::try_from(String::from(
                constants_str::TEST_REPEATED_SECRET
            )),
            Err(crate::bounded_secret_text_error::BoundedSecretTextError::RepeatedByte),
        );
    }
}
