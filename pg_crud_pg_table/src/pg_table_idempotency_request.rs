#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
)]
pub struct PgTableIdempotencyRequest {
    scope: crate::pg_table_idempotency_scope::PgTableIdempotencyScope,
    request_hash: crate::pg_table_idempotency_request_hash::PgTableIdempotencyRequestHash,
}

impl PgTableIdempotencyRequest {
    #[must_use]
    pub fn new(
        pg_table_idempotency_scope: crate::pg_table_idempotency_scope::PgTableIdempotencyScope,
        pg_table_idempotency_body_ref: crate::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef<'_>,
    ) -> Self {
        Self {
            scope: pg_table_idempotency_scope,
            request_hash: crate::calculate_pg_table_idempotency_request_hash::calculate_pg_table_idempotency_request_hash(pg_table_idempotency_body_ref),
        }
    }
}
