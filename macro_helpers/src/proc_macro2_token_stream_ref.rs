#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub struct ProcMacro2TokenStreamRef<'ts_lt>(&'ts_lt proc_macro2::TokenStream);
