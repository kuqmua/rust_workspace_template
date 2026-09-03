#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_foundation_foundation_from_inner::FromInner,
)]
pub(crate) struct ProcMacroInputTokenStream(proc_macro2::TokenStream);
impl ProcMacroInputTokenStream {
    pub(crate) fn into_inner(self) -> proc_macro2::TokenStream {
        self.0
    }
}
