#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_foundation::AsRefInner,
    proc_macro_newtype_foundation::FromInner,
)]
pub(crate) struct SynIdentifierRef<'syn_lt>(&'syn_lt syn::Ident);
