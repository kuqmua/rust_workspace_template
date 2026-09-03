#[proc_macro_derive(
    Location,
    attributes(
        eo_to_err_string,
        eo_to_err_string_serde,
        eo_location,
        eo_vec_to_err_string,
        eo_vec_to_err_string_serde,
        eo_vec_location,
        eo_hashmap_k_string_v_to_err_string,
        eo_hashmap_k_string_v_to_err_string_serde,
        eo_hashmap_k_string_v_location,
        location_to_schema,
    )
)]
pub fn derive_location(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_location_shared::derive_location(token_stream.into()).into()
}
