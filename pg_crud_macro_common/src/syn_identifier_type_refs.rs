#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub struct SynIdentifierTypeRefs<'lt>(&'lt [(&'lt syn::Ident, &'lt syn::Type)]);
