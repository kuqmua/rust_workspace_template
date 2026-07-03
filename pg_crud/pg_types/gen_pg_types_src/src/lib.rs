#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgTypesSourceGeneration;

#[must_use]
pub fn gen_pg_types(_input: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::new()
}
