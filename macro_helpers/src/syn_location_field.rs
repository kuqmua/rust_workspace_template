#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::IntoInnerFrom, newtype::FromInner,
)]
pub struct SynLocationField(syn::Field);
