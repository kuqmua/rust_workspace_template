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
    bounded_types::bounded_vec::BoundedVec<
        crate::process_command::ProcessCommand,
        0,
        { usize::MAX },
    >,
);
