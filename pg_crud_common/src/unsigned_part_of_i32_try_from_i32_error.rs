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
pub enum UnsignedPartOfI32TryFromI32Error {
    LessThanZero {
        location: location_lib::location::Location,
        #[eo_to_err_string_serde]
        v: crate::unsigned_part_of_i32_raw::UnsignedPartOfI32Raw,
    },
}
