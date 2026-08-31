#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    thiserror::Error,
    newtype::DerefInner,
    newtype::FromInner,
)]
#[error(transparent)]
pub struct BoundedReadIoError(std::io::Error);
