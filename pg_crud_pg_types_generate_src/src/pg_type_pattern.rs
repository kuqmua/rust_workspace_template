#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "lint suppression is required here"
)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    std::hash::Hash,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::Display,
    strum_macros::EnumIter,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) enum PgTypePattern {
    Standard,
}
