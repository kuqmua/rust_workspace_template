#[derive(
    Debug,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_foundation_foundation_from_inner::FromInner,
)]
pub struct ProcMacroInputTokenStream(proc_macro2::TokenStream);
impl ProcMacroInputTokenStream {
    pub(crate) fn into_inner(self) -> proc_macro2::TokenStream {
        self.0
    }
}

impl From<proc_macro::TokenStream> for ProcMacroInputTokenStream {
    fn from(value: proc_macro::TokenStream) -> Self {
        Self::from(proc_macro2::TokenStream::from(value))
    }
}
