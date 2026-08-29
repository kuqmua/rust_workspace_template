use super::*;

#[derive(
    Debug, thiserror::Error, location::Location, optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum ErrorTwo {
    Another {
        #[eo_to_err_string_serde]
        sdasdasd: LocationTestText,
        location: location_lib::domain_types::Location,
    },
    Variant {
        #[eo_to_err_string_serde]
        eo_display_with_serde_field: LocationTestText,
        location: location_lib::domain_types::Location,
    },
}
