#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct ChildProcessReports(
    bounded_types::BoundedVec<super::ChildProcessReport, 0, { usize::MAX }>,
);
