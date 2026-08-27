#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum NotZeroUnsignedPartOfI32TryFromI32Error {
    IsZero {
        location: location_lib::domain_types::Location,
    },
    UnsignedPartOfI32TryFromI32Error {
        #[eo_location]
        v: crate::domain_types::UnsignedPartOfI32TryFromI32Error,
        location: location_lib::domain_types::Location,
    },
}
