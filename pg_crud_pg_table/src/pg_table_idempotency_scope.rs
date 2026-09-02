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
    proc_macro_new::New,
)]
pub struct PgTableIdempotencyScope {
    actor: crate::pg_table_idempotency_actor::PgTableIdempotencyActor,
    method: crate::pg_table_idempotency_method::PgTableIdempotencyMethod,
    route: crate::pg_table_idempotency_route::PgTableIdempotencyRoute,
    key: crate::pg_table_idempotency_key::PgTableIdempotencyKey,
}
