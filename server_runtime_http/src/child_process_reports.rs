#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct ChildProcessReports(
    bounded_types::bounded_vec::BoundedVec<
        crate::child_process_report::ChildProcessReport,
        0,
        { usize::MAX },
    >,
);
