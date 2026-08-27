#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::*;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyScope {
    pub(super) route: PgTableIdempotencyRoute,
    pub(super) method: PgTableIdempotencyMethod,
    pub(super) key: PgTableIdempotencyKey,
    pub(super) actor: PgTableIdempotencyActor,
}
