#[must_use]
pub fn calculate_pg_table_idempotency_request_hash(
    pg_table_idempotency_body_ref: crate::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef<
        '_,
    >,
) -> crate::pg_table_idempotency_request_hash::PgTableIdempotencyRequestHash {
    let digest = <sha2::Sha256 as sha2::Digest>::digest(pg_table_idempotency_body_ref.as_ref());
    let mut bytes = [constants_u8::ZERO; 32usize];
    bytes.copy_from_slice(&digest);
    crate::pg_table_idempotency_request_hash::PgTableIdempotencyRequestHash::from(bytes)
}
