#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct ScaffoldPathRef<'path_lt>(pub(super) &'path_lt std::path::Path);
