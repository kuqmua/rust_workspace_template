#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
)]
pub struct RouteContractMismatches(
    bounded_types::bounded_vec::BoundedVec<
        crate::route_contract_mismatch::RouteContractMismatch,
        0,
        { usize::MAX },
    >,
);
