#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[must_use]
pub fn new_pg_table_idempotency_key() -> crate::pg_table_idempotency_key::PgTableIdempotencyKey {
    crate::pg_table_idempotency_key::PgTableIdempotencyKey::from(uuid::Uuid::new_v4())
}
impl crate::pg_table_idempotency_scope::PgTableIdempotencyScope {
    #[must_use]
    pub const fn new(
        actor: crate::pg_table_idempotency_actor::PgTableIdempotencyActor,
        method: crate::pg_table_idempotency_method::PgTableIdempotencyMethod,
        route: crate::pg_table_idempotency_route::PgTableIdempotencyRoute,
        key: crate::pg_table_idempotency_key::PgTableIdempotencyKey,
    ) -> Self {
        Self {
            route,
            method,
            key,
            actor,
        }
    }
}
impl crate::pg_table_idempotency_request::PgTableIdempotencyRequest {
    #[must_use]
    pub fn new(
        scope: crate::pg_table_idempotency_scope::PgTableIdempotencyScope,
        body: crate::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef<'_>,
    ) -> Self {
        Self {
            scope,
            request_hash: crate::calculate_pg_table_idempotency_request_hash::calculate_pg_table_idempotency_request_hash(body),
        }
    }
    #[must_use]
    pub const fn scope(&self) -> &crate::pg_table_idempotency_scope::PgTableIdempotencyScope {
        &self.scope
    }
}
impl crate::pg_table_idempotency_replay::PgTableIdempotencyReplay {
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        crate::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus,
        crate::pg_table_idempotency_body::PgTableIdempotencyBody,
    ) {
        (self.response_status, self.response_body)
    }
}
