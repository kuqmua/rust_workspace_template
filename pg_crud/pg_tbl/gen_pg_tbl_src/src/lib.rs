#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgTableSourceGeneration;

#[must_use]
pub fn gen_pg_tbl(_input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::new()
}
