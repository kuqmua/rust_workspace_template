struct LocationInput(syn::DeriveInput);

#[proc_macro_derive(
    Location,
    attributes(
        eo_hashmap_k_string_v_loc,
        eo_hashmap_k_string_v_to_err_string,
        eo_hashmap_k_string_v_to_err_string_serde,
        eo_loc,
        eo_to_err_string,
        eo_to_err_string_serde,
        eo_vec_loc,
        eo_vec_to_err_string,
        eo_vec_to_err_string_serde
    )
)]
pub fn loc(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let LocationInput(_derive_input) = match syn::parse::<syn::DeriveInput>(input_token_stream) {
        Ok(derive_input) => LocationInput(derive_input),
        Err(error) => return error.to_compile_error().into(),
    };
    proc_macro::TokenStream::new()
}
