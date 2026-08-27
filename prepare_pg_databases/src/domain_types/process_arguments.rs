#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct ProcessArguments(
    bounded_types::domain_types::vector::BoundedVec<super::ProcessArgument, 0, { usize::MAX }>,
);
