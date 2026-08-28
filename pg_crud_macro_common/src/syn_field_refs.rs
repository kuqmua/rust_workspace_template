#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct SynFieldRefs<'lt>(&'lt [macro_helpers::domain_types::syn_field::SynField]);
