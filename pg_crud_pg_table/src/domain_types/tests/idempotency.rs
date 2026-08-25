#[test]
fn request_hash_is_stable_and_payload_sensitive() {
    let first = super::pg_table_idempotency_request_hash(super::PgTableIdempotencyBodyRef::from(
        b"same payload".as_slice(),
    ));
    let second = super::pg_table_idempotency_request_hash(super::PgTableIdempotencyBodyRef::from(
        b"same payload".as_slice(),
    ));
    let changed = super::pg_table_idempotency_request_hash(super::PgTableIdempotencyBodyRef::from(
        b"changed payload".as_slice(),
    ));
    assert_eq!(first, second);
    assert_ne!(first, changed);
}

#[test]
fn idempotency_text_types_enforce_boundaries_and_protocol_shape() {
    assert_eq!(
        super::PgTableIdempotencyActor::try_from(String::new()),
        Err(super::PgTableIdempotencyTextError::Empty)
    );
    assert_eq!(
        super::PgTableIdempotencyMethod::try_from("GET".to_owned()),
        Err(super::PgTableIdempotencyTextError::InvalidMethod)
    );
    assert_eq!(
        super::PgTableIdempotencyRoute::try_from("without-slash".to_owned()),
        Err(super::PgTableIdempotencyTextError::InvalidRoute)
    );
    let oversized = constants_str::A_ALT
        .repeat(super::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES.saturating_add(constants_usize::ONE));
    assert_eq!(
        super::PgTableIdempotencyKey::try_from(oversized.clone()),
        Err(super::PgTableIdempotencyTextError::TooLong {
            actual_bytes: super::PgTableIdempotencyTextBytes::from(oversized.len()),
            maximum_bytes: super::PgTableIdempotencyTextBytes::from(
                super::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES,
            ),
        })
    );
}

#[test]
fn generated_idempotency_keys_are_valid_and_distinct() {
    let first = super::new_pg_table_idempotency_key();
    let second = super::new_pg_table_idempotency_key();
    assert_ne!(first, second);
    assert!(!first.as_ref().is_empty());
    assert!(first.as_ref().len() <= super::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES);
}

#[test]
fn persisted_idempotency_body_enforces_inclusive_storage_limit() {
    let exact = super::PgTableIdempotencyBody::try_from(vec![
        constants_u8::ZERO;
        constants_usize::VALUE_1_048_576
    ])
    .expect(
        "aa90ef11 persisted_idempotency_body_enforces_inclusive_storage_limit invariant must hold",
    );
    assert_eq!(exact.as_ref().len(), constants_usize::VALUE_1_048_576);
    assert_eq!(
        super::PgTableIdempotencyBody::try_from(vec![
            constants_u8::ZERO;
            constants_usize::VALUE_1_048_576
                + constants_usize::ONE
        ])
        .map(drop),
        Err(super::PgTableIdempotencyBodyError)
    );
}
