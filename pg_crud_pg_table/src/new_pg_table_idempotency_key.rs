#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[must_use]
pub fn new_pg_table_idempotency_key() -> crate::pg_table_idempotency_key::PgTableIdempotencyKey {
    crate::pg_table_idempotency_key::PgTableIdempotencyKey::from(uuid::Uuid::new_v4())
}
