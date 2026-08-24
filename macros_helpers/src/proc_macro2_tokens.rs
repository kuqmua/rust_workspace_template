#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Default,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::Display,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    newtype::ToTokens,
)]
pub struct ProcMacro2GeneratedRustTokenStream(proc_macro2::TokenStream);
