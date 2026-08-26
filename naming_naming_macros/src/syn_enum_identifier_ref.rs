#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::AsRefInner, newtype::FromInner,
)]
pub(crate) struct SynEnumIdentifierRef<'identifier_lt>(&'identifier_lt syn::Ident);
