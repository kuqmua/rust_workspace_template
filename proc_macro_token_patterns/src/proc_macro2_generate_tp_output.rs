#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub(crate) struct ProcMacro2GenerateTpOutput(proc_macro2::TokenStream);
