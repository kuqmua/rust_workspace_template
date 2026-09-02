#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
)]
pub struct ProcessCommands(
    bounded_types::bounded_vec::BoundedVec<
        crate::process_command::ProcessCommand,
        0,
        { usize::MAX },
    >,
);
