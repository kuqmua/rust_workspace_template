#[derive(
    Debug, thiserror::Error, location::Location, optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum ErrorTwo {
    Another {
        #[eo_to_err_string_serde]
        sdasdasd: crate::location_test_text::LocationTestText,
        location: location_lib::location::Location,
    },
    Variant {
        #[eo_to_err_string_serde]
        eo_display_with_serde_field: crate::location_test_text::LocationTestText,
        location: location_lib::location::Location,
    },
}
