use crate::domain_types::{LocationColumn, LocationFile, LocationLine};

// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct Occr {
    pub file: LocationFile,
    pub line: LocationLine,
    pub column: LocationColumn,
}
