#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum PgTableIdempotencyBegin {
    Acquired,
    Conflict,
    InProgress,
    Replay(crate::pg_table_idempotency_replay::PgTableIdempotencyReplay),
}
