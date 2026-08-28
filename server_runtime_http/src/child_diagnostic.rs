#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct ChildDiagnostic(bounded_types::domain_types::vector::BoundedVec<u8, 0, { usize::MAX }>);
