#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyScope {
    pub(super) route: crate::pg_table_idempotency_route::PgTableIdempotencyRoute,
    pub(super) method: crate::pg_table_idempotency_method::PgTableIdempotencyMethod,
    pub(super) key: crate::pg_table_idempotency_key::PgTableIdempotencyKey,
    pub(super) actor: crate::pg_table_idempotency_actor::PgTableIdempotencyActor,
}
