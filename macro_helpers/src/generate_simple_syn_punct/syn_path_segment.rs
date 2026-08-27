#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct SynPathSegment(syn::PathSegment);
