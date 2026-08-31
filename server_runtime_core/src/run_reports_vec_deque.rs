#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub(super) struct RunReportsVecDeque<RunReport>(std::collections::VecDeque<RunReport>);
