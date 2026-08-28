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
    bounded_types::domain_types::vector::BoundedVec<
        crate::domain_types::ProcessCommand,
        0,
        { usize::MAX },
    >,
);
