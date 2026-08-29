use super::*;

#[derive(
    Debug, thiserror::Error, location::Location, optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum ErrorOne {
    Variant {
        #[eo_to_err_string]
        eo_display_field: DisplayStruct,
        #[eo_to_err_string_serde]
        eo_serde: SerdeStruct,
        #[eo_location]
        eo_location_field: ErrorTwo,
        #[eo_vec_to_err_string]
        eo_vec_display_field: Vec<DisplayStruct>,
        #[eo_vec_to_err_string_serde]
        eo_vec_serde: Vec<SerdeStruct>,
        #[eo_vec_location]
        eo_vec_location_field: Vec<ErrorUnnamedOne>,
        #[eo_hashmap_k_string_v_to_err_string]
        hashmap_string_string: std::collections::HashMap<LocationTestText, DisplayStruct>,
        #[eo_hashmap_k_string_v_to_err_string_serde]
        hashmap_string_serde: std::collections::HashMap<LocationTestText, SerdeStruct>,
        #[eo_hashmap_k_string_v_location]
        hashmap_string_location: std::collections::HashMap<LocationTestText, ErrorUnnamedOne>,
        location: location_lib::domain_types::Location,
    },
}
