#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::ToTokens,
)]
pub struct SynFieldVis(syn::Visibility);
