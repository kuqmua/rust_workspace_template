#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::IntoInner,
)]
pub(crate) struct ContractSynIdent(syn::Ident);
