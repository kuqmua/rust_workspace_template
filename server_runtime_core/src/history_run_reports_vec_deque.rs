#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the history owner module mutates this private storage wrapper"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub(super) struct RunReportsVecDeque<RunReport>(std::collections::VecDeque<RunReport>);
