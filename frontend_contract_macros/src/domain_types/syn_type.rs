#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynType(syn::Type);
impl SynType {
    #[allow(clippy::single_call_fn)] // this conversion keeps the wrapped syn type private at the proc-macro boundary
    pub(crate) fn into_inner(self) -> syn::Type {
        self.0
    }
}
