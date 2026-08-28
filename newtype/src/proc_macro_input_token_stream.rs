#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct ProcMacroInputTokenStream(proc_macro::TokenStream);
impl From<proc_macro::TokenStream> for ProcMacroInputTokenStream {
    fn from(value: proc_macro::TokenStream) -> Self {
        Self(value)
    }
}
impl ProcMacroInputTokenStream {
    pub(crate) fn into_inner(self) -> proc_macro::TokenStream {
        self.0
    }
}
