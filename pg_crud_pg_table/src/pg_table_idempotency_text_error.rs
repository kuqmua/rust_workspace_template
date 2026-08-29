#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum PgTableIdempotencyTextError {
    #[error("{}", constants_str::catalog::IDEMPOTENCY_TEXT_MUST_NOT_BE_EMPTY)]
    Empty,
    #[error(
        "{}",
        constants_str::catalog::IDEMPOTENCY_METHOD_MUST_BE_POST_PATCH_OR_DELETE
    )]
    InvalidMethod,
    #[error(
        "{}",
        constants_str::catalog::IDEMPOTENCY_ROUTE_MUST_START_WITH_A_SLASH
    )]
    InvalidRoute,
    #[error("idempotency text exceeds {maximum_bytes} bytes: got {actual_bytes}")]
    TooLong {
        actual_bytes: crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes,
        maximum_bytes: crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes,
    },
}
