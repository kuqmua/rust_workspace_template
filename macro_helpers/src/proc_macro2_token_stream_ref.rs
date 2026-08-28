#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct ProcMacro2TokenStreamRef<'ts_lt>(&'ts_lt proc_macro2::TokenStream);
