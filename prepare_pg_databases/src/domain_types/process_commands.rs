#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct ProcessCommands(
    bounded_types::domain_types::vector::BoundedVec<super::ProcessCommand, 0, { usize::MAX }>,
);
