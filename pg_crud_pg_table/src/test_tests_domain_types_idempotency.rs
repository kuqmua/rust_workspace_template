#[test]
fn test_request_hash_is_stable_and_payload_sensitive() {
    let first = crate::calculate_pg_table_idempotency_request_hash::calculate_pg_table_idempotency_request_hash(
        crate::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(b"same payload".as_slice()),
    );
    let second = crate::calculate_pg_table_idempotency_request_hash::calculate_pg_table_idempotency_request_hash(
        crate::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(b"same payload".as_slice()),
    );
    let changed = crate::calculate_pg_table_idempotency_request_hash::calculate_pg_table_idempotency_request_hash(
        crate::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(b"changed payload".as_slice()),
    );
    assert_eq!(first, second);
    assert_ne!(first, changed);
}

#[test]
fn test_idempotency_text_types_enforce_boundaries_and_protocol_shape() {
    assert_eq!(
        crate::pg_table_idempotency_actor::PgTableIdempotencyActor::try_from(String::new()),
        Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::Empty)
    );
    assert_eq!(
        crate::pg_table_idempotency_method::PgTableIdempotencyMethod::try_from(
            constants_str::GET.to_owned()
        ),
        Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::InvalidMethod)
    );
    assert_eq!(
        crate::pg_table_idempotency_route::PgTableIdempotencyRoute::try_from(
            constants_str::VALUE_4F7B3F17.to_owned()
        ),
        Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::InvalidRoute)
    );
    let idempotency_text_error =
        crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::InvalidMethod;
    assert_eq!(
        to_err_string::to_err_string::ToErrString::to_err_string(&idempotency_text_error).as_ref(),
        idempotency_text_error.to_string(),
    );
    let oversized = constants_str::A_ALT.repeat(
        crate::pg_table_idempotency_text_max_bytes::PG_TABLE_IDEMPOTENCY_TEXT_MAX_BYTES
            .saturating_add(constants_usize::ONE),
    );
    assert_eq!(
        crate::pg_table_idempotency_key::PgTableIdempotencyKey::try_from(oversized.clone()),
        Err(
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::TooLong {
                actual_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        oversized.len()
                    ),
                maximum_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        crate::pg_table_idempotency_text_max_bytes::PG_TABLE_IDEMPOTENCY_TEXT_MAX_BYTES,
                    ),
            }
        )
    );
}

#[test]
fn test_fixed_uuid_idempotency_keys_preserve_content_and_are_distinct() {
    let first_uuid = uuid::Uuid::from_u128(0x00112233_4455_4677_8899_aabbccddeeffu128);
    let second_uuid = uuid::Uuid::from_u128(0x00112233_4455_4677_8899_aabbccddeefeu128);
    let first =
        crate::pg_table_idempotency_key::PgTableIdempotencyKey::try_from(first_uuid.to_string())
            .expect(constants_str::DIAGNOSTIC_2C2E015B);
    let second =
        crate::pg_table_idempotency_key::PgTableIdempotencyKey::try_from(second_uuid.to_string())
            .expect(constants_str::DIAGNOSTIC_632ACE74);
    assert_eq!(first.as_ref(), first_uuid.to_string());
    assert_eq!(second.as_ref(), second_uuid.to_string());
    assert_ne!(first, second);
    assert!(!first.as_ref().is_empty());
    assert!(
        first.as_ref().len()
            <= crate::pg_table_idempotency_text_max_bytes::PG_TABLE_IDEMPOTENCY_TEXT_MAX_BYTES
    );
}

#[test]
fn test_persisted_idempotency_body_enforces_inclusive_storage_limit() {
    let exact = crate::pg_table_idempotency_body::PgTableIdempotencyBody::try_from(vec![
        constants_u8::ZERO;
        constants_usize::VALUE_1_048_576
    ])
    .expect(constants_str::DIAGNOSTIC_AA90EF11);
    assert_eq!(exact.as_ref().len(), constants_usize::VALUE_1_048_576);
    assert!(matches!(
        crate::pg_table_idempotency_body::PgTableIdempotencyBody::try_from(vec![
            constants_u8::ZERO;
            constants_usize::VALUE_1_048_576
                + constants_usize::ONE
        ]),
        Err(crate::pg_table_idempotency_body_error::PgTableIdempotencyBodyError::TooLarge(
            bounded_types::bounded_value_error::BoundedValueError::AboveMax { actual, max }
        )) if actual.get() == constants_usize::VALUE_1_048_576 + constants_usize::ONE
            && max.get() == constants_usize::VALUE_1_048_576
    ));
}

#[test]
fn test_idempotency_methods_preserve_supported_verbs_and_validation_precedence() {
    assert!(
        [
            constants_str::POST,
            constants_str::PATCH,
            constants_str::DELETE
        ]
        .into_iter()
        .all(|value| {
            crate::pg_table_idempotency_method::PgTableIdempotencyMethod::try_from(value.to_owned())
                .is_ok_and(|method| method.as_ref() == value)
        })
    );
    [
        (
            constants_str::EMPTY.to_owned(),
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::Empty,
        ),
        (
            constants_str::POST.to_lowercase(),
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::InvalidMethod,
        ),
        (
            format!("{}{}", constants_str::SPACE, constants_str::POST),
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::InvalidMethod,
        ),
        (
            format!("{}{}", constants_str::POST, constants_str::SPACE),
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::InvalidMethod,
        ),
        (
            constants_str::X.repeat(255usize),
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::InvalidMethod,
        ),
        (
            constants_str::X.repeat(256usize),
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::TooLong {
                actual_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        256usize,
                    ),
                maximum_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        255usize,
                    ),
            },
        ),
    ]
    .into_iter()
    .fold((), |(), (value, expected)| {
        assert_eq!(
            crate::pg_table_idempotency_method::PgTableIdempotencyMethod::try_from(value),
            Err(expected)
        );
    });
}

#[test]
fn test_idempotency_routes_preserve_prefix_contract_and_exact_byte_boundaries() {
    [
        '/'.to_string(),
        format!("/{}", constants_str::X.repeat(1_023usize)),
        format!(
            "/{}{}",
            '\u{e9}'.to_string().repeat(511usize),
            constants_str::X
        ),
    ]
    .into_iter()
    .fold((), |(), value| {
        assert!(
            crate::pg_table_idempotency_route::PgTableIdempotencyRoute::try_from(value.clone())
                .is_ok_and(|route| route.as_ref() == value)
        );
    });
    [
        (
            constants_str::EMPTY.to_owned(),
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::Empty,
        ),
        (
            constants_str::X.repeat(1_024usize),
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::InvalidRoute,
        ),
        (
            format!("/{}", '\u{e9}'.to_string().repeat(512usize)),
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::TooLong {
                actual_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        1_025usize,
                    ),
                maximum_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        1_024usize,
                    ),
            },
        ),
        (
            constants_str::X.repeat(1_025usize),
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::TooLong {
                actual_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        1_025usize,
                    ),
                maximum_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        1_024usize,
                    ),
            },
        ),
    ]
    .into_iter()
    .fold((), |(), (value, expected)| {
        assert_eq!(
            crate::pg_table_idempotency_route::PgTableIdempotencyRoute::try_from(value),
            Err(expected)
        );
    });
}

#[test]
fn test_idempotency_actor_and_key_preserve_text_and_shared_byte_limits() {
    [
        constants_str::X.to_owned(),
        constants_str::SPACE.to_owned(),
        constants_str::X.repeat(255usize),
        format!(
            "{}{}",
            '\u{e9}'.to_string().repeat(127usize),
            constants_str::X
        ),
    ]
    .into_iter()
    .fold((), |(), value| {
        assert!(
            crate::pg_table_idempotency_actor::PgTableIdempotencyActor::try_from(value.clone())
                .is_ok_and(|actor| actor.as_ref() == value)
        );
        assert!(
            crate::pg_table_idempotency_key::PgTableIdempotencyKey::try_from(value.clone())
                .is_ok_and(|key| key.as_ref() == value)
        );
    });
    [
        (
            constants_str::EMPTY.to_owned(),
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::Empty,
        ),
        (
            '\u{e9}'.to_string().repeat(128usize),
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::TooLong {
                actual_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        256usize,
                    ),
                maximum_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        255usize,
                    ),
            },
        ),
    ]
    .into_iter()
    .fold((), |(), (value, expected)| {
        assert_eq!(
            crate::pg_table_idempotency_actor::PgTableIdempotencyActor::try_from(value.clone()),
            Err(expected.clone())
        );
        assert_eq!(
            crate::pg_table_idempotency_key::PgTableIdempotencyKey::try_from(value),
            Err(expected)
        );
    });
}

#[test]
fn test_idempotency_replay_transfers_status_and_body_without_reallocating() {
    assert!([
        (Vec::new(), Some(constants_str::EMPTY)),
        (constants_str::U_1F496.as_bytes().to_vec(), Some(constants_str::U_1F496)),
        (vec![constants_u8::ZERO; constants_usize::VALUE_1_048_576], None),
    ].into_iter().all(|(bytes, expected_text)| {
        let pointer = bytes.as_ptr();
        let length = bytes.len();
        crate::pg_table_idempotency_body::PgTableIdempotencyBody::try_from(bytes).is_ok_and(|body| {
            let replay = crate::pg_table_idempotency_replay::PgTableIdempotencyReplay::new(
                body,
                crate::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus::internal_server_error(),
            );
            let (status, transferred) = replay.into_parts();
            u16::from(status) == 500u16
                && transferred.as_ref().len() == length
                && transferred.as_ref().as_ptr() == pointer
                && expected_text.map_or_else(|| transferred.as_ref().iter().all(|byte| *byte == constants_u8::ZERO), |text| transferred.as_ref() == text.as_bytes())
        })
    }));
}

#[test]
fn test_idempotency_request_retains_scope_and_empty_payload_digest() {
    assert!(match (
        crate::pg_table_idempotency_actor::PgTableIdempotencyActor::try_from(
            constants_str::X.to_owned()
        ),
        crate::pg_table_idempotency_method::PgTableIdempotencyMethod::try_from(
            constants_str::POST.to_owned()
        ),
        crate::pg_table_idempotency_route::PgTableIdempotencyRoute::try_from('/'.to_string()),
        crate::pg_table_idempotency_key::PgTableIdempotencyKey::try_from(
            constants_str::U_1F496.to_owned()
        ),
    ) {
        (Ok(actor), Ok(method), Ok(route), Ok(key)) => {
            let scope = crate::pg_table_idempotency_scope::PgTableIdempotencyScope::new(
                actor, method, route, key,
            );
            let request = crate::pg_table_idempotency_request::PgTableIdempotencyRequest::new(
                scope,
                crate::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(
                    constants_str::EMPTY.as_bytes(),
                ),
            );
            request.get_scope().get_actor().as_ref() == constants_str::X
                && request.get_scope().get_method().as_ref() == constants_str::POST
                && request
                    .get_scope()
                    .get_route()
                    .as_ref()
                    .chars()
                    .eq(std::iter::once('/'))
                && request.get_scope().get_key().as_ref() == constants_str::U_1F496
                && request.get_request_hash().get()
                    == [
                        227u8, 176u8, 196u8, 66u8, 152u8, 252u8, 28u8, 20u8, 154u8, 251u8, 244u8,
                        200u8, 153u8, 111u8, 185u8, 36u8, 39u8, 174u8, 65u8, 228u8, 100u8, 155u8,
                        147u8, 76u8, 164u8, 149u8, 153u8, 27u8, 120u8, 82u8, 184u8, 85u8,
                    ]
        }
        _ => false,
    });
}

#[test]
fn test_idempotency_sqlx_error_preserves_source_without_exposing_source_text() {
    let wrapped = crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from(
        sqlx::Error::Protocol(constants_str::U_1F496.to_owned()),
    );
    assert_eq!(
        wrapped.to_string(),
        constants_str::POSTGRESQL_IDEMPOTENCY_OPERATION_FAILED
    );
    assert_eq!(
        to_err_string::to_err_string::ToErrString::to_err_string(&wrapped).as_ref(),
        constants_str::POSTGRESQL_IDEMPOTENCY_OPERATION_FAILED
    );
    assert!(std::error::Error::source(&wrapped).is_some_and(|source| {
        source.downcast_ref::<sqlx::Error>().is_some_and(
            |error| matches!(error, sqlx::Error::Protocol(text) if text == constants_str::U_1F496),
        )
    }));
    assert!(
        matches!(sqlx::Error::from(wrapped), sqlx::Error::Protocol(text) if text == constants_str::U_1F496)
    );
}
