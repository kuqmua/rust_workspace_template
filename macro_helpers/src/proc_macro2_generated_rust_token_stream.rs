#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Default,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
    proc_macro_newtype::ToTokens,
)]
pub struct ProcMacro2GeneratedRustTokenStream(proc_macro2::TokenStream);
