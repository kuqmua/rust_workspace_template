#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    proc_macro_location_derive_location::Location,
    schemars::JsonSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum NotZeroUnsignedPartOfI32TryFromI32Error {
    IsZero {
        location: location_lib::location::Location,
    },
    UnsignedPartOfI32TryFromI32Error {
        #[eo_location]
        v: crate::unsigned_part_of_i32_try_from_i32_error::UnsignedPartOfI32TryFromI32Error,
        location: location_lib::location::Location,
    },
}
