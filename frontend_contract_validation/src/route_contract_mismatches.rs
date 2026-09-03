#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct RouteContractMismatches(
    bounded_types::bounded_vec::BoundedVec<
        crate::route_contract_mismatch::RouteContractMismatch,
        0,
        { usize::MAX },
    >,
);
