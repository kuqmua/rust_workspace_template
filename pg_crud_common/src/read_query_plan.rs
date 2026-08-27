#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct ReadQueryPlan(crate::domain_types::QueryPartFragment);
