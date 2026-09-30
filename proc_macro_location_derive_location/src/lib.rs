#[proc_macro_derive(
    Location,
    attributes(
        error_field_to_err_string,
        error_field_to_err_string_serde,
        error_field_location,
        error_field_vec_to_err_string,
        error_field_vec_to_err_string_serde,
        error_field_vec_location,
        error_field_hashmap_key_string_value_to_err_string,
        error_field_hashmap_key_string_value_to_err_string_serde,
        error_field_hashmap_key_string_value_location,
        location_to_schema,
    )
)]
pub fn derive_location(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_location_shared::derive_location(token_stream.into()).into()
}
