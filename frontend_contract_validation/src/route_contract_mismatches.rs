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
    bounded_types::BoundedVec<
        crate::route_contract_validation::RouteContractMismatch,
        0,
        { usize::MAX },
    >,
);
