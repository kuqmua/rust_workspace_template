#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInner,
)]
pub(super) struct ProcMacro2CaseTokenStream(proc_macro2::TokenStream);
