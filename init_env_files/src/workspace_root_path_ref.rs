#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
)]
pub(crate) struct WorkspaceRootPathRef<'root_lt>(&'root_lt std::path::Path);
