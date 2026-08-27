#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct RouteContractMismatches(
    bounded_types::domain_types::vector::BoundedVec<
        super::RouteContractMismatch,
        0,
        { usize::MAX },
    >,
);
