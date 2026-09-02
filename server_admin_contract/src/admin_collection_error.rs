#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum AdminCollectionError {
    #[error(
        "{}",
        constants_str::ADMINISTRATOR_COLLECTION_EXCEEDS_MAXIMUM_ITEM_COUNT
    )]
    TooLong,
}
