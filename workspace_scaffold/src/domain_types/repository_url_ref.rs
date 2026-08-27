#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct RepositoryUrlRef<'value>(pub(super) &'value str);
