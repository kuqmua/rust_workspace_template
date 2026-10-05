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

#[test]
fn test_transport_request_header_updates_preserve_payload_and_other_headers() {
    let body_result = crate::transport_body::TransportBody::try_from(Vec::from([1u8, 2u8]));
    let path_result =
        crate::transport_path::TransportPath::try_from(constants_str::SLASH.to_owned());
    let key_result = crate::transport_idempotency_key::TransportIdempotencyKey::try_from(
        constants_str::X.to_owned(),
    );
    let match_result =
        crate::transport_if_match::TransportIfMatch::try_from(constants_str::VALUE_1.to_owned());
    assert!(
        body_result.is_ok() && path_result.is_ok() && key_result.is_ok() && match_result.is_ok()
    );
    let (
        Ok(transport_body),
        Ok(transport_path),
        Ok(transport_idempotency_key),
        Ok(transport_if_match),
    ) = (body_result, path_result, key_result, match_result)
    else {
        return;
    };
    let route = crate::route_contract::RouteContract::new(
        crate::authentication_requirement::AuthenticationRequirement::Public,
        crate::route_method::RouteMethod::Get,
        crate::mutation_kind::MutationKind::ReadOnly,
        crate::contract_str::ContractStr::from(constants_str::SLASH),
        crate::success_status::SuccessStatus::Code200,
    );
    let request =
        crate::transport_request::TransportRequest::new(transport_body, transport_path, route);
    assert!(request.idempotency_key().is_none());
    assert!(request.if_match().is_none());
    let keyed = request.with_idempotency_key(transport_idempotency_key);
    assert!(
        keyed
            .idempotency_key()
            .is_some_and(|key| key.as_ref() == constants_str::X)
    );
    assert!(keyed.if_match().is_none());
    let matched = keyed.with_if_match(transport_if_match);
    assert!(
        matched
            .idempotency_key()
            .is_some_and(|key| key.as_ref() == constants_str::X)
    );
    assert!(
        matched
            .if_match()
            .is_some_and(|value| value.as_ref() == constants_str::VALUE_1)
    );
    let replacement_key_result =
        crate::transport_idempotency_key::TransportIdempotencyKey::try_from(
            constants_str::VALUE_1.to_owned(),
        );
    let replacement_match_result =
        crate::transport_if_match::TransportIfMatch::try_from(constants_str::X.to_owned());
    assert!(replacement_key_result.is_ok() && replacement_match_result.is_ok());
    let (Ok(replacement_key), Ok(replacement_match)) =
        (replacement_key_result, replacement_match_result)
    else {
        return;
    };
    let replaced_key = matched.with_idempotency_key(replacement_key);
    assert!(
        replaced_key
            .idempotency_key()
            .is_some_and(|key| key.as_ref() == constants_str::VALUE_1)
    );
    assert!(
        replaced_key
            .if_match()
            .is_some_and(|value| value.as_ref() == constants_str::VALUE_1)
    );
    let replaced_headers = replaced_key.with_if_match(replacement_match);
    assert!(
        replaced_headers
            .idempotency_key()
            .is_some_and(|key| key.as_ref() == constants_str::VALUE_1)
    );
    assert!(
        replaced_headers
            .if_match()
            .is_some_and(|value| value.as_ref() == constants_str::X)
    );
    assert_eq!(replaced_headers.body().as_ref(), &[1u8, 2u8]);
    assert_eq!(replaced_headers.path().as_ref(), constants_str::SLASH);
    assert_eq!(replaced_headers.route(), route);
}
