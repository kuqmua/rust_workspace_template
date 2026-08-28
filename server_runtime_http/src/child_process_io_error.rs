#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error(transparent)]
#[derive(newtype::FromInner)]
pub struct ChildProcessIoError(std::io::Error);
