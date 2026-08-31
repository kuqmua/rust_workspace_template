#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(crate) struct SynGeneratePgTableModelError(syn::Error);
