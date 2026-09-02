#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum DatabaseUrlError {
    #[error("{0}", constants_str::DATABASE_URL_MUST_NOT_BE_EMPTY)]
    Empty,
    #[error("{0}", constants_str::DATABASE_URL_EXCEEDS_MAXIMUM_LENGTH)]
    TooLong,
}
