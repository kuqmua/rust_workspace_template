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

#[test]
fn test_lease_stale_timeout_rejects_zero_and_preserves_positive_duration_boundaries() {
    assert!(matches!(
        crate::lease_stale_timeout_duration::LeaseStaleTimeoutDuration::try_from(
            std::time::Duration::ZERO
        ),
        Err(crate::std_lease_stale_timeout_error::StdLeaseStaleTimeoutError::Zero)
    ));
    assert!(
        [
            std::time::Duration::from_nanos(1u64),
            std::time::Duration::new(1u64, 123_456_789u32),
            std::time::Duration::new(u64::MAX, 0u32),
            std::time::Duration::MAX,
        ]
        .into_iter()
        .all(|duration| {
            crate::lease_stale_timeout_duration::LeaseStaleTimeoutDuration::try_from(duration)
                .is_ok_and(|timeout| *timeout == duration)
        })
    );
}
