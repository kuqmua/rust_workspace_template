#[proc_macro_attribute]
pub fn dm_logic(
    attribute_token_stream: proc_macro::TokenStream,
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_generate_pg_table_shared::dm_logic(
        attribute_token_stream.into(),
        input_token_stream.into(),
    )
    .into()
}
