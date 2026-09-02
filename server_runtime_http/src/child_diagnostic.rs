#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
)]
pub struct ChildDiagnostic(bounded_types::bounded_vec::BoundedVec<u8, 0, { usize::MAX }>);
