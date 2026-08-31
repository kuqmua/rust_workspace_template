#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    generate_accessor::Getters,
    generate_constructor::New,
)]
pub struct PgTableIdempotencyScope {
    actor: crate::pg_table_idempotency_actor::PgTableIdempotencyActor,
    method: crate::pg_table_idempotency_method::PgTableIdempotencyMethod,
    route: crate::pg_table_idempotency_route::PgTableIdempotencyRoute,
    key: crate::pg_table_idempotency_key::PgTableIdempotencyKey,
}
