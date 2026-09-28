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
pub enum StdDurationRangeLengthError {
    IsZero {
        location: location_lib::location::Location,
    },
    SubmicrosecondPrecision {
        location: location_lib::location::Location,
    },
    ExceedsPostgresIntervalDays {
        location: location_lib::location::Location,
    },
}
