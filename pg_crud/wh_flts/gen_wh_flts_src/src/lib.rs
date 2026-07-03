#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhereFiltersSourceGeneration;

#[must_use]
pub fn gen_wh_flts(_input: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::new()
}
