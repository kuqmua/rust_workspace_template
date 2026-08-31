#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum PgTableRevisionTryFromStringError {
    #[error("{}", constants_str::REVISION_MUST_BE_A_DECIMAL_INTEGER)]
    Invalid(#[source] crate::pg_table_revision_parse_int_error::PgTableRevisionParseIntError),
    #[error("{}", constants_str::REVISION_MUST_NOT_BE_NEGATIVE)]
    Negative,
}
