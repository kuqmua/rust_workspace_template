pub mod cfg;
pub mod types;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgJsonObjSourceGeneration;

#[must_use]
pub fn gen_pg_json_obj(_input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::new()
}
