#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct SynFieldRefs<'lt>(&'lt [macro_helpers::syn_field::SynField]);
