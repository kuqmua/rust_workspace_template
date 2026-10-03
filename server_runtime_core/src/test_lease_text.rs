#[test]
fn test_lease_text_boundaries_and_error_precedence_are_shared_by_id_and_key() {
    let maximum = crate::lease_text_maximum_bytes::LEASE_TEXT_MAXIMUM_BYTES;
    [
        (
            constants_str::EMPTY.to_owned(),
            Err(crate::lease_text_error::LeaseTextError::Empty),
        ),
        (constants_str::X.to_owned(), Ok(())),
        (constants_str::SPACE.to_owned(), Ok(())),
        (
            constants_str::TEST_TEXT_WITH_NUL.to_owned(),
            Err(crate::lease_text_error::LeaseTextError::ContainsNul),
        ),
        (constants_str::X.repeat(maximum), Ok(())),
        (
            '\u{e9}'
                .to_string()
                .repeat(maximum.div_euclid(constants_usize::TWO)),
            Ok(()),
        ),
        (
            '\u{e9}'
                .to_string()
                .repeat(maximum.div_euclid(constants_usize::TWO) + constants_usize::ONE),
            Err(crate::lease_text_error::LeaseTextError::TooLong),
        ),
        (
            constants_str::X.repeat(maximum + constants_usize::ONE),
            Err(crate::lease_text_error::LeaseTextError::TooLong),
        ),
        (
            '\0'.to_string().repeat(maximum + constants_usize::ONE),
            Err(crate::lease_text_error::LeaseTextError::TooLong),
        ),
    ]
    .into_iter()
    .fold((), |(), (value, expected)| {
        assert_eq!(
            crate::validate_lease_text::validate_lease_text(
                crate::lease_text_ref::LeaseTextRef::from(value.as_str())
            ),
            expected,
        );
        assert_eq!(
            crate::lease_id::LeaseId::try_from(value.clone())
                .map(|lease_id| lease_id.as_ref() == value),
            expected.map(|()| true),
        );
        assert_eq!(
            crate::lease_key::LeaseKey::try_from(value.clone())
                .map(|lease_key| lease_key.as_ref() == value),
            expected.map(|()| true),
        );
    });
}
