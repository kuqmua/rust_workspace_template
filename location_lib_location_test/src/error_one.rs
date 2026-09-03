#[derive(
    Debug,
    thiserror::Error,
    proc_macro_location_derive_location::Location,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum ErrorOne {
    Variant {
        #[eo_to_err_string]
        eo_display_field: crate::display_struct::DisplayStruct,
        #[eo_to_err_string_serde]
        eo_serde: crate::serde_struct::SerdeStruct,
        #[eo_location]
        eo_location_field: crate::error_two::ErrorTwo,
        #[eo_vec_to_err_string]
        eo_vec_display_field: Vec<crate::display_struct::DisplayStruct>,
        #[eo_vec_to_err_string_serde]
        eo_vec_serde: Vec<crate::serde_struct::SerdeStruct>,
        #[eo_vec_location]
        eo_vec_location_field: Vec<crate::error_unnamed_one::ErrorUnnamedOne>,
        #[eo_hashmap_k_string_v_to_err_string]
        hashmap_string_string: std::collections::HashMap<
            crate::location_test_text::LocationTestText,
            crate::display_struct::DisplayStruct,
        >,
        #[eo_hashmap_k_string_v_to_err_string_serde]
        hashmap_string_serde: std::collections::HashMap<
            crate::location_test_text::LocationTestText,
            crate::serde_struct::SerdeStruct,
        >,
        #[eo_hashmap_k_string_v_location]
        hashmap_string_location: std::collections::HashMap<
            crate::location_test_text::LocationTestText,
            crate::error_unnamed_one::ErrorUnnamedOne,
        >,
        location: location_lib::location::Location,
    },
}
