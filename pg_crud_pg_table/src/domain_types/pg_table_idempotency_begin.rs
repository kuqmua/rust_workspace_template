#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum PgTableIdempotencyBegin {
    Acquired,
    Conflict,
    InProgress,
    Replay(PgTableIdempotencyReplay),
}
