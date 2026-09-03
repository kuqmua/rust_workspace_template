#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_to_tokens::ToTokens,
)]
pub struct SynFieldVis(syn::Visibility);
