#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::FromInner,
)]
pub(super) struct ProcMacro2CaseTokenStream(proc_macro2::TokenStream);
impl ProcMacro2CaseTokenStream {
    pub(super) fn into_inner(self) -> proc_macro2::TokenStream {
        self.0
    }
}
