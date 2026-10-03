#[cfg(test)]
mod tests {
    fn bounded_secret(str: &str) -> crate::bounded_secret_text::BoundedSecretText {
        crate::bounded_secret_text::BoundedSecretText::try_from(str.to_owned())
            .expect(constants_str::DIAGNOSTIC_2C20F43D)
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
    #[test]
    fn test_secret_text_boundaries_and_validation_precedence_match_borrowed_and_owned_values() {
        let mixed = |length| {
            format!(
                "{}{}",
                constants_str::X.repeat(length),
                constants_str::VALUE_1
            )
        };
        [
            (
                constants_str::EMPTY.to_owned(),
                Err(crate::bounded_secret_text_error::BoundedSecretTextError::InvalidLength),
            ),
            (
                mixed(14usize),
                Err(crate::bounded_secret_text_error::BoundedSecretTextError::InvalidLength),
            ),
            (mixed(15usize), Ok(())),
            (mixed(8_191usize), Ok(())),
            (
                mixed(8_192usize),
                Err(crate::bounded_secret_text_error::BoundedSecretTextError::InvalidLength),
            ),
            (
                constants_str::SPACE.repeat(15usize),
                Err(crate::bounded_secret_text_error::BoundedSecretTextError::InvalidLength),
            ),
            (
                constants_str::SPACE.repeat(16usize),
                Err(
                    crate::bounded_secret_text_error::BoundedSecretTextError::SurroundingWhitespace,
                ),
            ),
            (
                format!(
                    "{}{}",
                    constants_str::SPACE,
                    constants_str::TEST_SECRET_TEXT
                ),
                Err(
                    crate::bounded_secret_text_error::BoundedSecretTextError::SurroundingWhitespace,
                ),
            ),
            (
                format!(
                    "{}{}",
                    constants_str::TEST_SECRET_TEXT,
                    constants_str::SPACE
                ),
                Err(
                    crate::bounded_secret_text_error::BoundedSecretTextError::SurroundingWhitespace,
                ),
            ),
            (
                constants_str::X.repeat(16usize),
                Err(crate::bounded_secret_text_error::BoundedSecretTextError::RepeatedByte),
            ),
        ]
        .into_iter()
        .fold((), |(), (value, expected)| {
            assert_eq!(
                crate::secret_text_ref::SecretTextRef::try_from(value.as_str()).map(|_secret| ()),
                expected
            );
            assert_eq!(
                crate::bounded_secret_text::BoundedSecretText::try_from(value).map(|_secret| ()),
                expected
            );
        });
    }
}
