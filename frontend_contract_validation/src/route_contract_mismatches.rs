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
    bounded_types::bounded_vec::BoundedVec<
        crate::route_contract_mismatch::RouteContractMismatch,
        0,
        { usize::MAX },
    >,
);
