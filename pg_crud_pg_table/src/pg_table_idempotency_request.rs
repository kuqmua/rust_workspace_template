#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyRequest {
    pub(super) scope: crate::pg_table_idempotency_scope::PgTableIdempotencyScope,
    pub(super) request_hash:
        crate::pg_table_idempotency_request_hash::PgTableIdempotencyRequestHash,
}
