#[test]
fn test_transport_idempotency_key_enforces_nonempty_byte_length_boundaries() {
    assert!(matches!(
        crate::transport_idempotency_key::TransportIdempotencyKey::try_from(String::new()),
        Err(
            crate::transport_idempotency_key::TransportIdempotencyKeyTryFromStringError::TooShort {
                len: 0,
                min: 1
            }
        )
    ));
    assert!([1usize, 255usize].into_iter().all(|length| {
        let value = constants_str::X.repeat(length);
        crate::transport_idempotency_key::TransportIdempotencyKey::try_from(value.clone())
            .is_ok_and(|key| key.as_ref() == value)
    }));
    assert!(matches!(
        crate::transport_idempotency_key::TransportIdempotencyKey::try_from(
            constants_str::X.repeat(256usize)
        ),
        Err(
            crate::transport_idempotency_key::TransportIdempotencyKeyTryFromStringError::TooLong {
                len: 256,
                max: 255
            }
        )
    ));
}

#[test]
fn test_transport_if_match_enforces_nonempty_byte_length_boundaries() {
    assert!(matches!(
        crate::transport_if_match::TransportIfMatch::try_from(String::new()),
        Err(
            crate::transport_if_match::TransportIfMatchTryFromStringError::TooShort {
                len: 0,
                min: 1
            }
        )
    ));
    assert!([1usize, 20usize].into_iter().all(|length| {
        let value = constants_str::X.repeat(length);
        crate::transport_if_match::TransportIfMatch::try_from(value.clone())
            .is_ok_and(|header| header.as_ref() == value)
    }));
    assert!(matches!(
        crate::transport_if_match::TransportIfMatch::try_from(constants_str::X.repeat(21usize)),
        Err(
            crate::transport_if_match::TransportIfMatchTryFromStringError::TooLong {
                len: 21,
                max: 20
            }
        )
    ));
    let value = char::from_u32(0x00e9).map(|character| character.to_string().repeat(11usize));
    assert!(value.is_some_and(|text| matches!(
        crate::transport_if_match::TransportIfMatch::try_from(text),
        Err(
            crate::transport_if_match::TransportIfMatchTryFromStringError::TooLong {
                len: 22,
                max: 20
            }
        )
    )));
}
