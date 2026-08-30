#[test]
fn request_hash_is_stable_and_payload_sensitive() {
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
fn idempotency_text_types_enforce_boundaries_and_protocol_shape() {
    assert_eq!(
        crate::pg_table_idempotency_actor::PgTableIdempotencyActor::try_from(String::new()),
        Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::Empty)
    );
    assert_eq!(
        crate::pg_table_idempotency_method::PgTableIdempotencyMethod::try_from("GET".to_owned()),
        Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::InvalidMethod)
    );
    assert_eq!(
        crate::pg_table_idempotency_route::PgTableIdempotencyRoute::try_from(
            "without-slash".to_owned()
        ),
        Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::InvalidRoute)
    );
    let oversized = constants_str::catalog::A_ALT.repeat(
        crate::pg_tbl_idempotency_text_max_bytes::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES
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
                        crate::pg_tbl_idempotency_text_max_bytes::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES,
                    ),
            }
        )
    );
}

#[test]
fn generated_idempotency_keys_are_valid_and_distinct() {
    let first = crate::new_pg_table_idempotency_key::new_pg_table_idempotency_key();
    let second = crate::new_pg_table_idempotency_key::new_pg_table_idempotency_key();
    assert_ne!(first, second);
    assert!(!first.as_ref().is_empty());
    assert!(
        first.as_ref().len()
            <= crate::pg_tbl_idempotency_text_max_bytes::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES
    );
}

#[test]
fn persisted_idempotency_body_enforces_inclusive_storage_limit() {
    let exact = crate::pg_table_idempotency_body::PgTableIdempotencyBody::try_from(vec![
        constants_u8::ZERO;
        constants_usize::VALUE_1_048_576
    ])
    .expect(
        "aa90ef11 persisted_idempotency_body_enforces_inclusive_storage_limit invariant must hold",
    );
    assert_eq!(exact.as_ref().len(), constants_usize::VALUE_1_048_576);
    assert_eq!(
        crate::pg_table_idempotency_body::PgTableIdempotencyBody::try_from(vec![
            constants_u8::ZERO;
            constants_usize::VALUE_1_048_576
                + constants_usize::ONE
        ])
        .map(drop),
        Err(crate::pg_table_idempotency_body_error::PgTableIdempotencyBodyError::TooLarge)
    );
}
