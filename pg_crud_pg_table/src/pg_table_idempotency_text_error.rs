#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum PgTableIdempotencyTextError {
    #[error("{}", constants_str::IDEMPOTENCY_TEXT_MUST_NOT_BE_EMPTY)]
    Empty,
    #[error("{}", constants_str::IDEMPOTENCY_METHOD_MUST_BE_POST_PATCH_OR_DELETE)]
    InvalidMethod,
    #[error("{}", constants_str::IDEMPOTENCY_ROUTE_MUST_START_WITH_A_SLASH)]
    InvalidRoute,
    #[error("idempotency text exceeds {maximum_bytes} bytes: got {actual_bytes}")]
    TooLong {
        actual_bytes: crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes,
        maximum_bytes: crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes,
    },
}

impl to_err_string::to_err_string::ToErrString for PgTableIdempotencyTextError {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
