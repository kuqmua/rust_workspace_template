#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct ChildDiagnostic(bounded_types::bounded_vec::BoundedVec<u8, 0, { usize::MAX }>);
