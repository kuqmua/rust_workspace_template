#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype_foundation::FromInner,
)]
pub(crate) struct ProcMacroInputTokenStream(proc_macro::TokenStream);
impl ProcMacroInputTokenStream {
    pub(crate) fn into_inner(self) -> proc_macro::TokenStream {
        self.0
    }
}
