#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub(crate) struct WorkspaceRootPathRef<'root_lt>(&'root_lt std::path::Path);
