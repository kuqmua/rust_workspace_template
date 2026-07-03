#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgJsonSourceGeneration;

#[must_use]
pub fn gen_pg_json(_input: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::new()
}
