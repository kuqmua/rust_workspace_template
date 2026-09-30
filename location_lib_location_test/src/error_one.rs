#[derive(
    Debug,
    thiserror::Error,
    proc_macro_location_derive_location::Location,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum ErrorOne {
    Variant {
        #[error_field_to_err_string]
        error_field_display_field: crate::display_struct::DisplayStruct,
        #[error_field_to_err_string_serde]
        error_field_serde: crate::serde_struct::SerdeStruct,
        #[error_field_location]
        error_field_location_field: crate::error_two::ErrorTwo,
        #[error_field_vec_to_err_string]
        error_field_vec_display_field: Vec<crate::display_struct::DisplayStruct>,
        #[error_field_vec_to_err_string_serde]
        error_field_vec_serde: Vec<crate::serde_struct::SerdeStruct>,
        #[error_field_vec_location]
        error_field_vec_location_field: Vec<crate::error_unnamed_one::ErrorUnnamedOne>,
        #[error_field_hashmap_key_string_value_to_err_string]
        hashmap_string_string: std::collections::HashMap<
            crate::location_test_text::LocationTestText,
            crate::display_struct::DisplayStruct,
        >,
        #[error_field_hashmap_key_string_value_to_err_string_serde]
        hashmap_string_serde: std::collections::HashMap<
            crate::location_test_text::LocationTestText,
            crate::serde_struct::SerdeStruct,
        >,
        #[error_field_hashmap_key_string_value_location]
        hashmap_string_location: std::collections::HashMap<
            crate::location_test_text::LocationTestText,
            crate::error_unnamed_one::ErrorUnnamedOne,
        >,
        location: location_lib::location::Location,
    },
}
