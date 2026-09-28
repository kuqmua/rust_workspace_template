#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

pub fn new_pg_table_idempotency_key() -> Result<
    crate::pg_table_idempotency_key::PgTableIdempotencyKey,
    crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError,
> {
    crate::pg_table_idempotency_key::PgTableIdempotencyKey::try_from(
        uuid::Uuid::new_v4().to_string(),
    )
}
