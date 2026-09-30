#[proc_macro_attribute]
pub fn delete_many_error_variants(
    attribute_token_stream: proc_macro::TokenStream,
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_generate_pg_table_shared::delete_many_error_variants(
        attribute_token_stream.into(),
        input_token_stream.into(),
    )
    .into()
}
