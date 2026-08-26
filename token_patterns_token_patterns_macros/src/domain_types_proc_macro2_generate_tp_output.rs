#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(crate) struct ProcMacro2GenerateTpOutput(proc_macro2::TokenStream);
