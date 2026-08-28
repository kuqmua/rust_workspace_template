use super::*;

#[must_use]
pub fn calculate_pg_table_idempotency_request_hash(
    body: PgTableIdempotencyBodyRef<'_>,
) -> PgTableIdempotencyRequestHash {
    let digest = <sha2::Sha256 as sha2::Digest>::digest(body.0);
    let mut bytes = [constants_u8::ZERO; 32usize];
    bytes.copy_from_slice(&digest);
    PgTableIdempotencyRequestHash::from(bytes)
}
