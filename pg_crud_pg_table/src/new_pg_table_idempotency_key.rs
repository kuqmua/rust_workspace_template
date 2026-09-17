#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[must_use]
pub fn new_pg_table_idempotency_key() -> crate::pg_table_idempotency_key::PgTableIdempotencyKey {
    match crate::pg_table_idempotency_key::PgTableIdempotencyKey::try_from(
        uuid::Uuid::new_v4().to_string(),
    ) {
        Ok(pg_table_idempotency_key) => pg_table_idempotency_key,
        Err(error) => {
            bounded_types::try_from_bounded_error_text::try_from_bounded_error_text(error)
        }
    }
}
