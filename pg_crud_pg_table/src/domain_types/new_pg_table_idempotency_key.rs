#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

#[must_use]
pub fn new_pg_table_idempotency_key() -> PgTableIdempotencyKey {
    PgTableIdempotencyKey::from(uuid::Uuid::new_v4())
}
impl PgTableIdempotencyScope {
    #[must_use]
    pub const fn new(
        actor: PgTableIdempotencyActor,
        method: PgTableIdempotencyMethod,
        route: PgTableIdempotencyRoute,
        key: PgTableIdempotencyKey,
    ) -> Self {
        Self {
            route,
            method,
            key,
            actor,
        }
    }
}
impl PgTableIdempotencyRequest {
    #[must_use]
    pub fn new(scope: PgTableIdempotencyScope, body: PgTableIdempotencyBodyRef<'_>) -> Self {
        Self {
            scope,
            request_hash: pg_table_idempotency_request_hash(body),
        }
    }
    #[must_use]
    pub const fn scope(&self) -> &PgTableIdempotencyScope {
        &self.scope
    }
}
impl PgTableIdempotencyReplay {
    #[must_use]
    pub fn into_parts(self) -> (PgTableIdempotencyResponseStatus, PgTableIdempotencyBody) {
        (self.response_status, self.response_body)
    }
}
