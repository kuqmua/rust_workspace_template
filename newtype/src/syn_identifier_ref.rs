#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype_foundation::AsRefInner,
    newtype_foundation::FromInner,
)]
pub(crate) struct SynIdentifierRef<'syn_lt>(&'syn_lt syn::Ident);
