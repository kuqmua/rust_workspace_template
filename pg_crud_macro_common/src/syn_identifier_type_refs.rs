#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct SynIdentifierTypeRefs<'lt>(&'lt [(&'lt syn::Ident, &'lt syn::Type)]);
