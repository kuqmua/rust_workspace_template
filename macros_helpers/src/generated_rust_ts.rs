#[derive(Debug, Clone, Default, newtype::Newtype)]
#[newtype(as_ref_owned, display, from_inner, to_tokens)]
pub struct GeneratedRustTs(proc_macro2::TokenStream);
impl From<GeneratedRustTs> for proc_macro2::TokenStream {
    fn from(value: GeneratedRustTs) -> Self {
        value.0
    }
}
impl std::ops::Deref for GeneratedRustTs {
    type Target = proc_macro2::TokenStream;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
