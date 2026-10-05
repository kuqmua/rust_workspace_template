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

    #[test]
    fn test_borrowed_and_owned_secret_text_formatting_preserves_redaction() {
        assert!(
            [
                constants_str::TEST_SECRET_TEXT,
                constants_str::TEST_DIFFERENT_SECRET_TEXT
            ]
            .into_iter()
            .all(|value| {
                let owned = bounded_secret(value);
                let borrowed = crate::secret_text_ref::SecretTextRef::from(&owned);
                format!("{owned}") == constants_str::REDACTED_ALT_3
                    && format!("{owned:?}") == constants_str::REDACTED_ALT_3
                    && format!("{borrowed:?}") == constants_str::REDACTED_ALT_3
                    && format!("{borrowed:#?}") == constants_str::REDACTED_ALT_3
            })
        );
    }

    #[test]
    fn test_secret_text_comparison_distinguishes_zero_byte_suffixes_in_both_directions() {
        let expected = bounded_secret(constants_str::TEST_SECRET_TEXT);
        let extended = bounded_secret(&format!("{}{}", constants_str::TEST_SECRET_TEXT, '\0'));
        assert_eq!(
            crate::secret_texts_match::secret_texts_match((&expected).into(), (&extended).into()),
            crate::secret_text_match::SecretTextMatch::Different,
        );
        assert_eq!(
            crate::secret_texts_match::secret_texts_match((&extended).into(), (&expected).into()),
            crate::secret_text_match::SecretTextMatch::Different,
        );
        assert_eq!(
            crate::secret_texts_match::secret_texts_match((&extended).into(), (&extended).into()),
            crate::secret_text_match::SecretTextMatch::Equal,
        );
    }

    #[test]
    fn test_maximum_length_secret_text_comparison_checks_the_final_byte() {
        let prefix = constants_str::X.repeat(constants_usize::VALUE_8_192 - 1usize);
        let expected = bounded_secret(&format!("{prefix}{}", '1'));
        let different = bounded_secret(&format!("{prefix}{}", '2'));
        assert_eq!(
            crate::secret_texts_match::secret_texts_match((&expected).into(), (&different).into()),
            crate::secret_text_match::SecretTextMatch::Different,
        );
        assert_eq!(
            crate::secret_texts_match::secret_texts_match((&different).into(), (&expected).into()),
            crate::secret_text_match::SecretTextMatch::Different,
        );
        assert_eq!(
            crate::secret_texts_match::secret_texts_match((&expected).into(), (&expected).into()),
            crate::secret_text_match::SecretTextMatch::Equal,
        );
    }
}
