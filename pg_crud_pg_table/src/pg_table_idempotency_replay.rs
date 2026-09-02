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
    proc_macro_new::New,
)]
pub struct PgTableIdempotencyReplay {
    response_body: crate::pg_table_idempotency_body::PgTableIdempotencyBody,
    response_status: crate::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus,
}

impl PgTableIdempotencyReplay {
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
