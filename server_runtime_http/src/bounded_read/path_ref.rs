#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct PathRef<'path_lt>(pub(super) &'path_lt std::path::Path);
