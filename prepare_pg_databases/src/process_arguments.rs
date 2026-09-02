#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
)]
pub struct ProcessArguments(
    bounded_types::bounded_vec::BoundedVec<
        crate::process_argument::ProcessArgument,
        0,
        { usize::MAX },
    >,
);
