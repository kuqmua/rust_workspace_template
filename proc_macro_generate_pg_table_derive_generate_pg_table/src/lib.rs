#[proc_macro_derive(
    GeneratePgTable,
    attributes(
        generate_pg_table_db_default,
        generate_pg_table_frontend,
        generate_pg_table_primary_key
    )
)]
pub fn derive_generate_pg_table(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_generate_pg_table_shared::derive_generate_pg_table(token_stream.into()).into()
}
